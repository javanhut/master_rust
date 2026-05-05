// SOLUTION — mutex1_basics

use std::sync::Mutex;

pub fn bump_once(m: &Mutex<i32>) -> i32 {
    let mut guard = m.lock().unwrap();
    *guard += 1;
    *guard
    // guard drops at end of scope — lock released.
}

pub fn read_value(m: &Mutex<i32>) -> i32 {
    let guard = m.lock().unwrap();
    *guard
}

// WHY THIS IS OPTIMAL:
//
//   `m.lock()` returns a `LockResult<MutexGuard<i32>>`. Unwrapping it
//   gives you the guard, which derefs to `&mut i32` (in `bump_once`,
//   bound mutably) or `&i32` (in `read_value`). The mutation goes
//   through that `*guard += 1`, then we copy the new value out via
//   `*guard` (i32 is Copy) before returning. No need to `clone` —
//   primitives travel by bit-copy.
//
//   The lock is held for exactly the body of each function, then
//   released when the guard's destructor runs at the closing brace.
//   Holding locks for the SHORTEST USEFUL window is the cardinal
//   rule of concurrent code: every nanosecond a lock is held is a
//   nanosecond other threads can't make progress.
//
//   `unwrap` on `lock()` panics on POISONED mutexes. A mutex becomes
//   poisoned if the thread holding it panics — Rust assumes the
//   protected invariant may now be broken and forces you to opt in
//   to seeing the inner value. For most code, propagating the panic
//   is the right behaviour.
//
// ALTERNATIVES:
//
//   - `m.try_lock()` returns `TryLockResult` immediately:
//
//         Ok(guard)                   — got the lock
//         Err(TryLockError::WouldBlock) — locked by someone else
//         Err(TryLockError::Poisoned(...))  — the previous holder panicked
//
//     Useful when you'd rather do something else than block.
//
//   - `RwLock<T>` allows MANY simultaneous readers OR one writer:
//
//         lock.read().unwrap()    // shared
//         lock.write().unwrap()   // exclusive
//
//     Faster than Mutex when reads vastly outnumber writes; slower
//     for write-heavy workloads because of the bookkeeping.
//
//   - For primitives, `std::sync::atomic::{AtomicUsize, ...}` give
//     you lock-free counters via `fetch_add(1, Ordering::Relaxed)`.
//     Faster than `Mutex<usize>` when the only operation is a counter.
//
//   - `parking_lot::Mutex` is a popular drop-in replacement: smaller,
//     no poisoning, often faster under contention. Worth knowing.
