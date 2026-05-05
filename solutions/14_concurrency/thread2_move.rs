// SOLUTION — thread2_move

use std::thread;

pub fn sum_in_thread(v: Vec<i32>) -> i32 {
    let handle = thread::spawn(move || v.iter().sum());
    handle.join().unwrap()
}

pub fn prepend_in_thread(prefix: String, name: String) -> String {
    let handle = thread::spawn(move || format!("{prefix}{name}"));
    handle.join().unwrap()
}

// WHY THIS IS OPTIMAL:
//
//   Without `move`, the closure would borrow `v` (or `prefix`/`name`)
//   from the spawning frame. The compiler can't prove the new thread
//   ends before the spawning function returns, so it rejects the
//   borrow with "may outlive borrowed value." Adding `move` switches
//   the capture from `&Vec<i32>` to `Vec<i32>` (and similarly for the
//   `String`s) — the closure now OWNS its data, satisfies `'static`,
//   and is free to outlive the spawner.
//
//   Note that `move` is a property of the CLOSURE LITERAL, not of any
//   particular variable: it forces by-value capture for EVERY name
//   the closure refers to. For Copy types this is invisible; for
//   owning types like Vec/String it's the load-bearing keyword.
//
// ALTERNATIVES:
//
//   - `thread::scope` lets you skip `move` and BORROW from the outer
//     scope, because the scope guarantees join-before-return:
//
//         let v = vec![1, 2, 3];
//         let total: i32 = thread::scope(|s| {
//             s.spawn(|| v.iter().sum::<i32>()).join().unwrap()
//         });
//
//     Use it for fan-out over a slice you don't want to clone.
//
//   - To share an owned value READ-ONLY across many threads, wrap it
//     in `Arc<T>` and clone the Arc per thread:
//
//         let v = Arc::new(vec![1, 2, 3]);
//         for _ in 0..N {
//             let v = Arc::clone(&v);
//             thread::spawn(move || { let _ = v.iter().sum::<i32>(); });
//         }
//
//   - To MUTATE shared state, you also need synchronisation
//     (`Arc<Mutex<T>>` — coming up in arc_mutex).
