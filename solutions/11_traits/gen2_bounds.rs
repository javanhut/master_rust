// SOLUTION — gen2_bounds

use std::fmt::Display;

fn show_clone<T>(x: T) -> String
where
    T: Display + Clone,
{
    let copy = x.clone();
    format!("{}, {}", x, copy)
}

fn pair_show<A, B>(a: A, b: B) -> String
where
    A: Display,
    B: Display,
{
    format!("{a} & {b}")
}

fn min_then_show<T>(a: T, b: T) -> String
where
    T: Display + PartialOrd,
{
    let smaller = if a < b { a } else { b };
    format!("{}", smaller)
}

// WHY THIS IS OPTIMAL:
//
//   Each function declares the SMALLEST set of bounds it actually uses:
//
//     - show_clone calls `.clone()` (needs Clone) and formats the value
//       with `{}` (needs Display). Two bounds, joined with `+`.
//
//     - pair_show only formats both arguments with `{}`. Each gets just
//       Display, on its own row. Notice `A` and `B` are independent type
//       parameters — they can be the same type or completely different.
//
//     - min_then_show needs `<` (PartialOrd) AND `{}` (Display). Two
//       bounds again, on T.
//
//   The `where` clause keeps each signature one line wide and lines up
//   the bounds vertically. As soon as a function has two or more type
//   parameters or two or more bounds per parameter, this is the form to
//   reach for.
//
// EQUIVALENT INLINE:
//
//   fn show_clone<T: Display + Clone>(x: T) -> String { ... }
//
//   Identical code, slightly noisier signature. Editorial preference once
//   you have multiple bounds; the Rust style guide nudges you toward
//   `where`.
//
// WHY DISPLAY AND NOT DEBUG:
//
//   `{}` is Display. `{:?}` is Debug. The format string we used is `{}`,
//   so the bound has to be Display. If a test wrote `format!("{:?}", x)`
//   we'd need `Debug` instead. The compiler will tell you exactly which
//   trait is missing if you mismatch them.
//
// MULTIPLE PARAMETERS WITH OVERLAPPING BOUNDS:
//
//   For a hypothetical `fn key_value<K, V>(k: K, v: V)` where both must be
//   Hash + Eq, you'd write:
//
//     where
//         K: Hash + Eq,
//         V: Hash + Eq,
//
//   There is no way to "factor out" the shared `Hash + Eq` — bounds attach
//   to the type parameter, not to a group of them. A trait alias would let
//   you name the bundle, but trait aliases are still nightly-only.
//
// WHY NOT REQUIRE `T: Copy` IN show_clone INSTEAD?
//
//   `Copy` is stricter than `Clone` (every Copy is Clone, not vice versa).
//   `String: Clone` but `String: !Copy`, so requiring Copy would reject
//   the String test case. `Clone` is the right minimum bound here.
