// =============================================================================
//  arc_mutex — Arc<Mutex<T>>: the bread-and-butter shared-state pattern
// =============================================================================
//
// You know the pieces. `Arc<T>` lets multiple threads OWN the same
// value (atomic refcount). `Mutex<T>` lets multiple threads MUTATE the
// same value (one at a time). Together:
//
//     let counter = Arc::new(Mutex::new(0_i32));
//
//     for _ in 0..N {
//         let counter = Arc::clone(&counter);          // bumps refcount
//         thread::spawn(move || {
//             let mut g = counter.lock().unwrap();      // takes the lock
//             *g += 1;                                  // mutates
//         });                                           // g drops -> lock free
//     }
//
// This is THE shape for "many threads writing to a shared piece of
// state." You'll see it in real codebases more than any other
// concurrency primitive.
//
// READ THE PIECES TOP TO BOTTOM
//
//     Arc<Mutex<T>>
//      ^   ^   ^
//      |   |   |
//      |   |   the actual data
//      |   serialises access from multiple threads
//      lets multiple owners point at the same allocation
//
// You ALMOST ALWAYS want this exact ordering. `Mutex<Arc<T>>` is rare
// and usually wrong: the lock would protect ownership of an Arc, but
// any thread that obtained an Arc clone could then bypass the lock.
//
// CLONING THE ARC, NOT THE MUTEX
//
// The `Mutex<T>` itself is NOT cloneable. The Arc IS, and that's how
// each thread gets its own owning handle:
//
//     let h = Arc::clone(&counter);
//     thread::spawn(move || {
//         let mut g = h.lock().unwrap();       // h.lock() == (&*h).lock()
//         *g += 1;
//     });
//
// Inside the closure, `h` is the captured Arc. `h.lock()` works via
// `Deref` — `Arc<T>` derefs to `&T`, so calling `.lock()` reaches
// through to the Mutex method.
//
// JOIN, THEN READ
//
// To inspect the final state, JOIN every worker first so all writes
// are flushed, then take one last lock on the main thread:
//
//     for h in handles { h.join().unwrap(); }
//     let final_value = *counter.lock().unwrap();
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   Write `parallel_count(num_threads, increments_per_thread)` that:
//     - constructs an Arc<Mutex<i32>> initialized to 0,
//     - spawns `num_threads` threads, each of which acquires the lock
//       `increments_per_thread` times and bumps the counter by 1,
//     - joins every thread,
//     - returns the final counter value.
//
//   The test calls it with (10, 100) and asserts the result is 1000.

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_count(num_threads: usize, increments_per_thread: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        // Each thread needs its OWN Arc clone — cloning an Arc only
        // bumps the atomic refcount; the underlying Mutex stays put.
        let c = Arc::???(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut g = c.???.unwrap();
                *g += 1;
                // g drops at end of iteration -> lock released, so
                // other threads get a turn.
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Read out the final value through one final lock.
    *counter.???.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn ten_threads_one_hundred_each() {
        assert_eq!(parallel_count(10, 100), 1000);
    }

    #[test] fn no_threads_no_change() {
        assert_eq!(parallel_count(0, 100), 0);
    }

    #[test] fn one_thread_baseline() {
        assert_eq!(parallel_count(1, 50), 50);
    }
}

fn main() {}
