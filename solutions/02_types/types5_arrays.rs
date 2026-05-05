// SOLUTION — types5_arrays

const WEEK: [&str; 7] = [
    "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
];

fn sum_slice(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {
        total += x;
    }
    total
}

// WHY THIS IS OPTIMAL:
//
//   `[&str; 7]` — the LENGTH is part of the type. The compiler verifies
//   at compile time that we put exactly 7 strings in. Forgetting one or
//   adding an extra is a build error.
//
//   `xs: &[i32]` (slice, not array) — `sum_slice` accepts ANY contiguous
//   sequence of i32: arrays of any length, Vecs, sub-slices. This is what
//   "write to the slice, not the array" means in Rust.
//
//   `for &x in xs` — the `&x` pattern destructures `&i32` into `i32`. Without
//   the `&` you'd get x: &i32, and `total += x` wouldn't compile (you'd
//   have to write `total += *x`).
//
// ITERATORS REVEAL:
//
//     fn sum_slice(xs: &[i32]) -> i32 { xs.iter().sum() }
//
//   Same machine code in release. Iterator chains will be your daily
//   bread once we get to chapter 7.
//
// COMMON QUESTION — "Why not a function that takes [i32; N]?":
//   You CAN, with const generics: `fn sum<const N: usize>(xs: [i32; N]) -> i32`.
//   But then every call site with a different length compiles a separate
//   copy of the function, and you can't pass a `Vec`. Slices are nearly
//   always the right choice.
