// =============================================================================
//  thread1 — spawning OS threads with std::thread
// =============================================================================
//
// `std::thread::spawn(f)` starts a NEW operating-system thread that runs
// the closure `f`. It returns a `JoinHandle<T>`:
//
//     use std::thread;
//
//     let handle = thread::spawn(|| {
//         // runs concurrently with the spawning thread
//         42
//     });
//
//     let value: i32 = handle.join().unwrap();   // waits, returns the result
//
// THE TYPE SIGNATURE (paraphrased):
//
//     pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static,
//
// Three things to internalise from those bounds:
//
//   1. `F: FnOnce() -> T`   — the closure runs exactly once.
//   2. `F: Send + 'static`  — it must be safe to MOVE to another thread,
//                             and it must contain no borrows that could
//                             dangle. Anything captured must outlive the
//                             new thread, which (since the new thread can
//                             outlive the spawning function) means the
//                             closure can only capture OWNED data or
//                             `'static` references.
//   3. `T: Send + 'static`  — the returned value must also cross threads
//                             back through `join()`.
//
// `JoinHandle::join()` returns `Result<T, Box<dyn Any + Send>>`. The Err
// case happens only if the thread PANICKED — `unwrap()` on the join
// re-propagates the panic into the caller. For tests where the thread
// is supposed to succeed, `.join().unwrap()` is exactly right.
//
// SCOPED THREADS (chapter aside)
//
// `std::thread::scope` lets a block spawn threads that may BORROW from
// the surrounding scope, because the scope guarantees they all join
// before the function returns. We'll meet it briefly later; this
// exercise is about the plain `spawn`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `spawn_and_get(n)` spawns one thread that returns `n * 2`, joins
//     it, and returns the doubled value.
//   - `spawn_many(n)` spawns `n` threads, each returning its index,
//     joins them all in order, and returns the sum (0 + 1 + ... + n-1).

// I AM NOT DONE

use std::thread;

pub fn spawn_and_get(n: i32) -> i32 {
    let handle = thread::spawn(??? {
        n * 2
    });
    handle.???.unwrap()
}

pub fn spawn_many(n: i32) -> i32 {
    let mut handles = Vec::new();
    for i in 0..n {
        let h = thread::spawn(??? {
            i
        });
        handles.push(h);
    }
    let mut total = 0;
    for h in handles {
        total += h.???.unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn doubles_one_value() {
        assert_eq!(spawn_and_get(21), 42);
    }

    #[test] fn doubles_zero() {
        assert_eq!(spawn_and_get(0), 0);
    }

    #[test] fn sums_indices() {
        // 0 + 1 + 2 + 3 + 4 = 10
        assert_eq!(spawn_many(5), 10);
    }

    #[test] fn empty_spawn_many() {
        assert_eq!(spawn_many(0), 0);
    }
}

fn main() {}
