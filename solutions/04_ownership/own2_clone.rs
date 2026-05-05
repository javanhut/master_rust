// SOLUTION — own2_clone

fn duplicate_string(s: &String) -> (String, String) {
    let a = s.clone();
    let b = s.clone();
    (a, b)
}

fn pair_of_ints(n: i32) -> (i32, i32) {
    (n, n)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `s.clone()` does the heap allocation + memcpy. We call it twice and we
//   really do get two independent buffers, which is what the test asserts
//   when we mutate `a` without touching `b`.
//
//   For `pair_of_ints`, we just write `(n, n)`. `i32` is `Copy`, so the
//   compiler bitwise-duplicates it on each use. There is no allocation,
//   no `.clone()` call, and no move. This is the implicit "free" copy
//   you've been relying on since chapter 1 without realising it.
//
// IDIOMATIC ALTERNATIVES:
//
//   - In real Rust we'd take `&str` instead of `&String`:
//
//         fn duplicate_string(s: &str) -> (String, String) {
//             (s.to_owned(), s.to_owned())
//         }
//
//     `&str` accepts both `&String` (via deref coercion) and string
//     literals, so it's strictly more flexible. We use `&String` here only
//     to keep the spotlight on `.clone()` — slices come in own6.
//
//   - `s.to_string()` and `s.to_owned()` are equivalent to `s.clone()` for
//     `String`. Stylistically, `.clone()` is preferred when the source is
//     already a `String`; `.to_owned()` reads better when going from `&str`.
//
// WHEN TO REACH FOR .clone() vs BORROWING:
//
//   - Need to keep using the original AND hand off ownership? clone.
//   - Just need to read it briefly inside a function? borrow with `&` (own4).
//   - Need to mutate someone else's value temporarily? `&mut` (own5).
//   - Sharing read-only across many owners with runtime cost? `Rc`/`Arc`
//     (later chapter).
