// SOLUTION — thread1_spawn

use std::thread;

pub fn spawn_and_get(n: i32) -> i32 {
    // `move` isn't strictly required when the only capture is a Copy
    // primitive (rustc will infer move for `i32` here either way), but
    // writing it explicitly makes the intent obvious: the closure owns
    // `n`, and it travels with the closure to the new thread.
    let handle = thread::spawn(move || n * 2);
    handle.join().unwrap()
}

pub fn spawn_many(n: i32) -> i32 {
    let mut handles = Vec::new();
    for i in 0..n {
        // Each closure captures its own `i` by value (Copy).
        let h = thread::spawn(move || i);
        handles.push(h);
    }
    let mut total = 0;
    for h in handles {
        total += h.join().unwrap();
    }
    total
}

// WHY THIS IS OPTIMAL:
//
//   `thread::spawn` requires the closure to be `Send + 'static`. For an
//   `i32` capture both are automatic — `i32` is Send, and being Copy it
//   contains no borrows, so it satisfies `'static`. A bare `||` would
//   capture by reference and FAIL to compile because the borrow of `n`
//   wouldn't be `'static`. `move ||` forces capture by value.
//
//   Joining in order is fine here because every thread does the same
//   trivial work and we just want the sum. If you wanted "first to
//   finish" semantics you'd reach for channels (next exercise) — `join`
//   only returns when the targeted handle is done.
//
// ALTERNATIVES:
//
//   - `thread::scope(|s| { s.spawn(|| ...); })` allows BORROWING from
//     the surrounding scope; the scope blocks until every spawned
//     thread joins, so borrows can't outlive their referents. Use it
//     when you want fan-out compute over a borrowed `&[T]` without
//     cloning.
//
//   - `thread::Builder::new().name("worker").stack_size(8 << 20).spawn(...)`
//     when you need to name the thread (useful in panic messages and
//     debuggers) or override the default stack size.
//
//   - For data-parallel work, rayon's `par_iter()` is usually the
//     right answer — a battle-tested work-stealing thread pool with
//     the same iterator combinators you already know.
