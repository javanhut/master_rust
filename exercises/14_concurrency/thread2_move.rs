// =============================================================================
//  thread2 — `move` closures: transferring ownership into a thread
// =============================================================================
//
// The closure passed to `thread::spawn` must be `'static`. That means
// every name it captures must either be `'static` or be OWNED by the
// closure itself — references that point at the spawning function's
// stack are forbidden, because the spawning function might return long
// before the new thread finishes.
//
// THE FAILING SHAPE
//
//     let v = vec![1, 2, 3];
//     thread::spawn(|| {
//         println!("{:?}", v);   // captures &v — borrows from the
//                                //  spawning frame
//     });
//     // ^^^ ERROR: closure may outlive borrowed value `v`
//
// The fix is the keyword `move`:
//
//     let v = vec![1, 2, 3];
//     thread::spawn(move || {
//         println!("{:?}", v);   // captures v BY VALUE
//     });
//     // `v` no longer usable here — it has been moved into the closure.
//
// `move` doesn't change WHAT the closure does; it changes the CAPTURE
// MODE. With `move`, every captured name is taken by value, including
// `Copy` types (which makes no observable difference for them). For a
// non-Copy type like `Vec<T>` or `String`, `move` is the difference
// between "compiles" and "lifetime error".
//
// CONSEQUENCE: ONE OWNER PER PIECE OF DATA
//
// Because the closure now owns its captures, you can't move the SAME
// vector into TWO threads:
//
//     let v = vec![1, 2, 3];
//     thread::spawn(move || println!("{:?}", v));     // moves v
//     thread::spawn(move || println!("{:?}", v));     // ERROR: use after move
//
// To share read-only data across threads, wrap it in `Arc<T>` and clone
// the `Arc` per thread (we'll do this in arc_mutex). To split owned
// chunks across threads, give each thread its own piece via slicing or
// `chunks` + `to_vec()`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `sum_in_thread(v)` takes a `Vec<i32>` BY VALUE, moves it into a
//     spawned thread, sums the elements there, and returns the sum.
//   - `prepend_in_thread(prefix, name)` takes two owned `String`s,
//     moves both into a spawned thread, and returns
//     `format!("{prefix}{name}")` from the thread.

// I AM NOT DONE

use std::thread;

pub fn sum_in_thread(v: Vec<i32>) -> i32 {
    // The closure must own `v`, because the new thread may run after
    // `sum_in_thread` returns. Add the keyword that forces by-value
    // capture.
    let handle = thread::spawn(??? || {
        v.iter().sum()
    });
    handle.join().unwrap()
}

pub fn prepend_in_thread(prefix: String, name: String) -> String {
    let handle = thread::spawn(??? || {
        format!("{prefix}{name}")
    });
    handle.???.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sums_owned_vec() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(sum_in_thread(v), 10);
    }

    #[test] fn sums_empty_vec() {
        assert_eq!(sum_in_thread(Vec::new()), 0);
    }

    #[test] fn concatenates_strings() {
        let p = String::from("Hello, ");
        let n = String::from("world");
        assert_eq!(prepend_in_thread(p, n), "Hello, world");
    }
}

fn main() {}
