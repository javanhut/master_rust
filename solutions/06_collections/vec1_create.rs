// SOLUTION — vec1_create

fn make_empty() -> Vec<i32> {
    Vec::new()
}

fn make_three() -> Vec<i32> {
    vec![1, 2, 3]
}

fn make_reserved(n: usize) -> Vec<i32> {
    Vec::with_capacity(n)
}

// WHY THESE ARE OPTIMAL:
//
//   `Vec::new()` is a `const fn` and allocates nothing. It's the right tool
//   for "I don't know how much I'll need yet."
//
//   `vec![1, 2, 3]` lowers to roughly:
//       let mut v = Vec::with_capacity(3);
//       v.push(1); v.push(2); v.push(3);
//   The macro is the idiomatic way to build a literal Vec.
//
//   `Vec::with_capacity(n)` skips the geometric-growth dance when you know
//   the size up front. A Vec doubles its capacity each time it overflows
//   (1 → 4 → 8 → 16 → ...), copying every element on each grow. Reserving
//   ahead of time turns that O(n log n) of churn into a single allocation.
//
// ALTERNATIVES:
//
//   `Vec::from([1, 2, 3])` works too and is what the macro effectively does
//   on modern compilers. `[1, 2, 3].to_vec()` also produces a Vec<i32>.
//   The `vec![]` macro stays preferred because it reads like a literal.
//
// COMMON PITFALL — capacity ≠ length:
//
//   `Vec::with_capacity(10)` has len() == 0. You can't index v[0] until
//   you've actually pushed something. Capacity is just "how big is the
//   allocation"; length is "how many initialised elements live in it".
