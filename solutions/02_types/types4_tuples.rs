// SOLUTION — types4_tuples

fn swap(p: (i32, &str)) -> (&str, i32) {
    let (a, b) = p;
    (b, a)
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

// WHY THIS IS OPTIMAL:
//
//   Destructuring `let (a, b) = p` makes the swap visually obvious: the
//   return literal `(b, a)` reads as "the original second, then the
//   original first".
//
//   `i32::min` / `i32::max` exist exactly for this. They are inherent
//   methods on every primitive integer/float type, return the same type,
//   and the compiler will inline them — they cost no more than a manual
//   `if a < b { a } else { b }`.
//
// EQUIVALENT BUT NOISIER:
//
//   fn swap(p: (i32, &str)) -> (&str, i32) {
//       (p.1, p.0)
//   }
//   Field access works fine. Destructuring tends to win when the names
//   carry meaning (e.g. `let (head, tail) = ...`) but for a 2-tuple this
//   is just stylistic.
//
//   fn min_max(a: i32, b: i32) -> (i32, i32) {
//       if a < b { (a, b) } else { (b, a) }
//   }
//   Identical behaviour. Use this when you want to do extra work in the
//   "ordered" branch (e.g. log a swap). For a pure ordering, `min`/`max`
//   reads better.
