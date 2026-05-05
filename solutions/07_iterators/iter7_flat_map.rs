// SOLUTION — iter7_flat_map

fn flatten_vecs(vs: Vec<Vec<i32>>) -> Vec<i32> {
    vs.into_iter().flatten().collect()
}

fn keep_somes(opts: Vec<Option<i32>>) -> Vec<i32> {
    opts.into_iter().flatten().collect()
}

fn words_of(text: &str) -> Vec<&str> {
    text.lines().flat_map(|line| line.split_whitespace()).collect()
}


// WHY THIS IS OPTIMAL:
//
//   flatten_vecs — `vs.into_iter()` yields owned `Vec<i32>`s. Each is
//   itself `IntoIterator<Item = i32>`, so `.flatten()` produces a single
//   `i32` stream. Three lines of a `for` loop with a manual extend, gone.
//
//   keep_somes — the lovely fact that `Option<T>` implements
//   `IntoIterator<Item = T>` (yielding 0 or 1 element) means `.flatten()`
//   on a `Vec<Option<T>>` is exactly "drop the Nones, unwrap the Somes".
//   This is the canonical idiom; reach for it whenever you have an
//   iterator of optionals you want to compact.
//
//   words_of — perfect `flat_map` shape. For each line we want zero or
//   more words. `.split_whitespace()` returns its own iterator type;
//   `flat_map` chains all of them end-to-end. Empty lines and lines made
//   of pure whitespace contribute nothing because `split_whitespace`
//   yields nothing for them — so we don't need to filter them out by hand.
//
//   The returned `&str`s borrow directly from `text`. Lifetime elision
//   carries the input lifetime through `lines`, through `flat_map`,
//   through `split_whitespace`, and out the function — no annotations
//   required.
//
// ALTERNATIVES:
//
//   `.flat_map(...)` is exactly `.map(...).flatten()`. The single-call
//   form usually reads better, but if you already have a `.map(...)`
//   that yields nested things, just tack `.flatten()` on the end.
//
//   `keep_somes` could also be written as
//       opts.into_iter().filter_map(|o| o).collect()
//   `filter_map` takes a closure returning `Option<U>` and keeps only the
//   `Some`s — basically "filter + map" for cases where you need the
//   identity of "succeeded vs not" anyway. For raw Options either form
//   works; for "parse each string and keep the successes" reach for
//   `.filter_map(|s| s.parse::<i32>().ok())`.
