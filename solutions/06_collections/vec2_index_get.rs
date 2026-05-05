// SOLUTION — vec2_index_get

fn safe_first(v: &Vec<i32>) -> Option<i32> {
    v.get(0).copied()
}

fn length(v: &Vec<i32>) -> usize {
    v.len()
}

fn is_blank(v: &Vec<i32>) -> bool {
    v.is_empty()
}

fn unsafe_at(v: &Vec<i32>, i: usize) -> i32 {
    v[i]
}

// WHY THESE ARE OPTIMAL:
//
//   `v.get(0).copied()` — `get` returns `Option<&i32>`. We don't want to
//   hand out a reference to a function-local Vec in the test, so `.copied()`
//   converts `Option<&T>` to `Option<T>` for `T: Copy`. (For non-Copy types
//   you'd use `.cloned()`.)
//
//   `v.is_empty()` — clearer and lint-friendly versus `v.len() == 0`. On
//   `Vec` the two compile to the same machine code, but `is_empty` is the
//   convention and works on every collection.
//
//   `v[i]` — `Index::index` panics on out-of-bounds. Use this ONLY when you
//   already know `i < v.len()`, e.g. inside `for i in 0..v.len()`.
//
// IDIOMATIC ALTERNATIVES:
//
//   `v.first().copied()`  — Vec has dedicated `first()` and `last()` methods
//   that return Option<&T>, equivalent to `get(0)` / `get(len-1)` but more
//   readable. We'll use them in vec5_slicing.
//
// THE BIG DESIGN POINT:
//
//   Rust splits "fast but panicky" (`v[i]`) from "safe but Optional"
//   (`v.get(i)`). The compiler can't tell whether `i` is in bounds at a
//   given call site, so it leaves the choice to you. Most code should reach
//   for `get` until profiling proves the bounds check matters.
