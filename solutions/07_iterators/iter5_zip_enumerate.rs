// SOLUTION — iter5_zip_enumerate

fn index_of_max(xs: &[i32]) -> Option<usize> {
    xs.iter()
        .enumerate()
        .max_by_key(|(_, v)| **v)
        .map(|(i, _)| i)
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn head_then_tail(head: &[i32], tail: &[i32]) -> Vec<i32> {
    head.iter().chain(tail.iter()).copied().collect()
}

fn evens_then_first_three() -> Vec<i32> {
    (0..).step_by(2).take(3).collect()
}


// WHY THIS IS OPTIMAL:
//
//   index_of_max — pairing `.enumerate()` with `.max_by_key` is the canonical
//   "argmax" idiom in Rust. Items become `(usize, &i32)`. The closure must
//   return a comparable key; we double-deref `**v` because the closure
//   itself receives a reference: `&(usize, &i32)` so `v: &&i32`. Finally
//   `.map(|(i, _)| i)` peels off the index. Returns `None` for empty input
//   automatically — `max_by_key` returns `Option`.
//
//   dot_product — `.zip` stops at the shorter side, which is exactly the
//   behaviour you want for "componentwise" operations on slices of possibly
//   different lengths. `(x, y)` here are `(&i32, &i32)`; the `*` operator
//   on `&i32` is auto-applied through `Mul`, so `x * y` works without an
//   explicit deref.
//
//   head_then_tail — `.chain` glues two iterators of the same item type.
//   `.copied()` is needed because we want to collect into `Vec<i32>`, not
//   `Vec<&i32>`. The order matters: head first, tail second.
//
//   evens_then_first_three — `(0..)` is the infinite iterator from 0. By
//   itself, calling `.collect()` on it would never return. `.step_by(2)`
//   keeps every second element; `.take(3)` makes it finite. This three-line
//   chain is more or less the official advert for lazy iterators: cheap,
//   composable, doesn't allocate anything until `collect`.
//
// ALTERNATIVES:
//
//   index_of_max with `.position(|x| x == max)` — possible, but two passes.
//   The enumerate+max_by_key form is one pass.
//
//   For `dot_product` you can write `.zip().fold(0, |acc, (x, y)| acc + x*y)`.
//   Same answer; `.sum()` is cleaner.
//
//   For `head_then_tail`, sometimes the simplest thing is
//   `[head, tail].concat()` — slices implement `Concat`. Slightly less
//   educational, equally correct.
//
//   `take_while` / `skip_while` are the predicate-driven cousins of `take`
//   and `skip`: keep going AS LONG AS the closure says yes.
