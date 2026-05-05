// =============================================================================
//  mutex1 — Mutex<T>: shared mutation across threads
// =============================================================================
//
// `Mutex<T>` ("MUTual EXclusion") wraps a value so that only ONE thread
// at a time can hold a mutable reference to it. The API is small:
//
//     use std::sync::Mutex;
//
//     let m: Mutex<i32> = Mutex::new(0);
//
//     {
//         let mut guard = m.lock().unwrap();   // MutexGuard<i32>
//         *guard += 1;                          // deref to &mut i32
//     } // guard drops here -> lock released
//
// `lock()` BLOCKS the current thread until the mutex is available, then
// returns `LockResult<MutexGuard<T>>`. The `Result` only goes Err if a
// previous holder PANICKED while the lock was held — that's lock
// "poisoning". For tests and normal code, `.unwrap()` is fine.
//
// THE GUARD
//
// `MutexGuard<T>` is the RAII handle that represents "I currently hold
// the lock." It implements `Deref<Target = T>` and `DerefMut`, so:
//
//     *guard          // -> &T  (and &mut T through DerefMut)
//     guard.method()  // auto-deref applies, like a smart pointer
//
// When the guard goes out of scope, its `Drop` impl releases the lock.
// You don't call `unlock()` explicitly — there is no such method. To
// release early, drop the guard explicitly:
//
//     let mut g = m.lock().unwrap();
//     *g += 1;
//     drop(g);                  // lock released NOW
//     do_other_work();
//
// SHADOWS OF REFCELL
//
// If `RefCell<T>` looks familiar — that's exactly the comparison.
//
//     RefCell<T>     single-threaded interior mutability; PANICS on conflict
//     Mutex<T>       multi-threaded interior mutability;  BLOCKS on conflict
//
// Both let you mutate T behind a `&` reference. RefCell is `!Sync`
// (single-threaded only); Mutex is `Sync` (sharable across threads).
// Crossing thread boundaries normally involves wrapping in `Arc`,
// which is the next exercise.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `bump_once(m)` takes a `&Mutex<i32>`, locks it, increments the
//     inner value by 1, and returns the new value.
//   - `read_value(m)` locks the mutex briefly and returns a copy of
//     the inner i32.

// I AM NOT DONE

use std::sync::Mutex;

pub fn bump_once(m: &Mutex<i32>) -> i32 {
    // Lock the mutex (panic on poisoning is fine), bump the inner
    // value through the guard, and return its new value.
    let mut guard = m.???.unwrap();
    *guard += ???;
    *guard
}

pub fn read_value(m: &Mutex<i32>) -> i32 {
    let guard = m.???.unwrap();
    *guard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn bumps_from_zero() {
        let m = Mutex::new(0);
        assert_eq!(bump_once(&m), 1);
        assert_eq!(bump_once(&m), 2);
        assert_eq!(read_value(&m), 2);
    }

    #[test] fn read_does_not_modify() {
        let m = Mutex::new(42);
        assert_eq!(read_value(&m), 42);
        assert_eq!(read_value(&m), 42);
    }

    #[test] fn guard_is_released_between_calls() {
        // If we held the lock past the function boundary, the second
        // call would deadlock. The fact that this terminates is the
        // proof that the guard drops at end of `bump_once`.
        let m = Mutex::new(10);
        bump_once(&m);
        bump_once(&m);
        bump_once(&m);
        assert_eq!(read_value(&m), 13);
    }
}

fn main() {}
