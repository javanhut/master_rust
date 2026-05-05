// SOLUTION — arc_mutex

use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_count(num_threads: usize, increments_per_thread: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut g = c.lock().unwrap();
                *g += 1;
                // g drops here — other threads can take the lock.
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    *counter.lock().unwrap()
}

// WHY THIS IS OPTIMAL:
//
//   The pattern is `Arc<Mutex<T>>` — Arc for shared OWNERSHIP across
//   threads, Mutex for serialised MUTATION. Each spawned thread gets
//   its own Arc clone (cheap atomic refcount bump), captures it by
//   value via `move`, and locks for the duration of one increment.
//   The lock is held for the SHORTEST window that contains the
//   read-modify-write, which is exactly one statement here.
//
//   Why doesn't this race? `*g += 1` looks like three operations
//   (read, add, write), but it happens entirely under the mutex,
//   which `lock()` enforces. Between threads, the lock provides both
//   mutual exclusion AND a happens-before relation: every write made
//   by thread A while it held the lock is visible to thread B the
//   next time B takes the lock. That's what stops the lost-update
//   race that a naive `static mut` counter would suffer.
//
//   At the end, every worker has joined, so no further increments
//   are coming. We take the lock one last time to read out the final
//   value. After this function returns, the local `counter` Arc
//   drops, refcount goes to zero, the Mutex (and the i32 inside) is
//   freed.
//
// PERFORMANCE NOTES:
//
//   For a SIMPLE counter like this, an atomic is dramatically faster
//   than `Arc<Mutex<i32>>`:
//
//       use std::sync::atomic::{AtomicI32, Ordering};
//       let counter = Arc::new(AtomicI32::new(0));
//       counter.fetch_add(1, Ordering::Relaxed);
//
//   No locking, no contention beyond the cache line itself. Reach
//   for `Atomic*` whenever the protected state is a single primitive
//   that needs only fetch-and-modify operations.
//
//   Reach for `Mutex` when the protected state is COMPOSITE — a
//   struct, a HashMap, a Vec — and the invariant spans multiple
//   fields. The lock then protects the whole-state invariant, which
//   no atomic can express.
//
// ALTERNATIVES:
//
//   - `Arc<RwLock<T>>` for many-readers / one-writer. Faster than
//     Mutex on read-heavy workloads, slightly slower on write-heavy.
//
//   - `thread::scope(|s| ...)` plus a single `&Mutex<T>` lets you
//     skip the Arc entirely — the scope guarantees the threads end
//     before the surrounding function returns, so a borrow into the
//     Mutex is fine. Smaller, faster, fewer moving parts when you
//     don't need ownership to outlive the call.
//
//   - For data-parallel reduction (sum, count, max), rayon's
//     `par_iter().sum()` will outpace a hand-rolled `Arc<Mutex<i32>>`
//     and not even contend on a single counter — it sums per-thread,
//     then combines.
