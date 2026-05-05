// SOLUTION — iter1_basics

fn first_two(xs: &[i32]) -> (Option<i32>, Option<i32>) {
    let mut it = xs.iter().copied();
    let a = it.next();
    let b = it.next();
    (a, b)
}

fn count_with_next(xs: &[i32]) -> usize {
    let mut it = xs.iter();
    let mut n = 0usize;
    loop {
        match it.next() {
            Some(_) => n += 1,
            None => break,
        }
    }
    n
}


// WHY THIS IS OPTIMAL:
//
//   first_two — `xs.iter()` yields `&i32`. Chaining `.copied()` produces an
//   adapter that yields `i32` (since `i32: Copy`). Now `next()` returns
//   `Option<i32>`, exactly what the signature wants. Calling `next` twice in
//   a row is fine because each call advances the cursor — `it` is `mut`
//   precisely so it can update its internal index.
//
//   count_with_next — this is the LITERAL desugaring of a `for` loop. The
//   point of writing it the long way once is to internalise that `for` is
//   not a magic keyword: it just calls `next` until `None`. After this
//   exercise, every iterator method you meet is "what would I do with
//   `loop { match next() }`?".
//
// ALTERNATIVES & WHEN TO USE THEM:
//
//   first_two without `.copied()`:
//       let mut it = xs.iter();
//       let a = it.next().copied();
//       let b = it.next().copied();
//   `.copied()` on `Option<&T>` works the same way as on iterators — you
//   pick whichever placement reads cleaner.
//
//   count_with_next idiomatically: `xs.iter().count()`. (Or `xs.len()` for
//   slices, which is O(1) instead of walking.) But the manual form is the
//   ONLY way to feel that `for` is sugar over `next`.
//
//   `while let`: `while let Some(_) = it.next() { n += 1; }` is the same
//   shape with one less line — once you see the `loop/match` desugaring,
//   `while let` is the natural cleanup.
