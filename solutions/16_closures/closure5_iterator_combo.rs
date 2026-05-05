// SOLUTION — closure5_iterator_combo

fn squares_of_evens(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect()
}

fn sum_of_positive(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &n| if n > 0 { acc + n } else { acc })
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

// WHY THIS IS OPTIMAL:
//
//   squares_of_evens — the classic "filter then map then collect" pipeline.
//   `.iter()` yields `&i32`. `.filter(|&&n| ...)` double-dereferences in the
//   pattern: the outer `&` is because filter passes `&Item` (so `&&i32`),
//   and the inner `&` strips down to `i32`. `.map(|&n| n*n)` likewise
//   destructures the `&i32` reference into `i32` (i32 is Copy, so this is
//   free). `.collect::<Vec<i32>>()` builds the result; the return type
//   annotation lets type inference pick `Vec<i32>` automatically.
//
//   Order of operations matters for performance: `filter` BEFORE `map`
//   means we only square the elements that survive. (Reversing the order
//   would also be correct here, but wasteful — and for non-Copy types you
//   could end up doing extra work.)
//
//   sum_of_positive — `fold` is the workhorse reducer. We start at 0 and
//   for each positive element, add it; otherwise, pass the accumulator
//   through unchanged. We could equivalently use:
//
//       nums.iter().filter(|&&n| n > 0).sum()
//
//   …which is arguably more idiomatic. The fold form is shown to make
//   the closure shape explicit.
//
//   longest_word — `max_by_key` takes a closure that maps each element to
//   a comparable key, and returns the element with the largest key.
//   `w.len()` gives a `usize`. Returns `Option<&str>`; we use
//   `.unwrap_or("")` to handle the empty-text case (where
//   `split_whitespace()` yields no elements). This also handles the
//   "all whitespace" case automatically — `split_whitespace` skips it.
//
//   Note we return `&str` borrowed FROM `text`. Lifetime elision handles
//   this: the input lifetime flows through `split_whitespace`, through
//   `max_by_key`, and out of the function with no annotation needed.
//
// ALTERNATIVES:
//
//   squares_of_evens:
//       nums.iter().copied().filter(|n| n % 2 == 0).map(|n| n * n).collect()
//   `copied()` upfront converts the iterator from `&i32` to `i32`, so the
//   later closures don't need the destructuring patterns. Often cleaner.
//
//   sum_of_positive:
//       nums.iter().filter(|&&n| n > 0).sum()
//   Shorter. The Iterator::sum method exists precisely for this.
//
//   longest_word ties: `max_by_key` returns the LAST element for ties,
//   which matches what most callers expect ("the longest word at or after
//   the front"). If you wanted the first one, use `.rev().max_by_key(...)`
//   or a manual fold.
