// SOLUTION — iter3_fold_sum

fn sum_via_sum(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn product_of(xs: &[i32]) -> i32 {
    xs.iter().product()
}

fn sum_via_fold(xs: &[i32]) -> i32 {
    xs.iter().fold(0, |acc, &x| acc + x)
}

fn count_via_fold(xs: &[i32]) -> usize {
    xs.iter().fold(0usize, |acc, _| acc + 1)
}


// WHY THIS IS OPTIMAL:
//
//   sum_via_sum / product_of — these are the textbook calls. The `Sum` and
//   `Product` traits know about `i32`, so passing through `&i32` items
//   "just works" because std implements both `Sum<i32>` and `Sum<&i32>`.
//   The return type annotation on the function fixes which numeric type to
//   produce, no turbofish required.
//
//   sum_via_fold — fold takes (initial, combiner). Start at 0, add each
//   element, return the running total. The pattern `|acc, &x|` destructures
//   the `&i32` item into a plain `i32`. Conceptually this IS `Iterator::sum`
//   for primitive numerics — once you write it once you've earned the
//   shortcut.
//
//   count_via_fold — same idea, ignoring the item with `_`. Note the
//   `0usize` annotation: `count` returns `usize`, so we have to start the
//   accumulator there or the closure return type won't match the function's
//   return type.
//
// ALTERNATIVES:
//
//   `xs.iter().copied().sum()` — sum after removing the references. Same
//   answer; some prefer it because the fold-style closures look prettier.
//
//   `xs.len()` instead of count_via_fold for slices — O(1) vs O(n). When
//   the underlying source has a length, prefer it. `count` is for lazy
//   chains where you've already filtered/mapped and don't know the size.
//
//   `try_fold` — the short-circuiting cousin. The closure returns
//   `ControlFlow` (or `Result`); if it ever signals "stop", fold halts.
//   Useful when summing values that might overflow:
//       xs.iter().try_fold(0i32, |acc, &x| acc.checked_add(x))
//   yields `Option<i32>` — `None` if any partial sum overflowed.
