// SOLUTION — match1_patterns

fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 | 5 | 7 | 11 => "small prime",
        _x @ 1..=9 => "single digit",
        10..=99 => "double digit",
        _ => "other",
    }
}

// WHY THIS IS OPTIMAL:
//
//   The arms read top-down as a priority list:
//     1. Explicit zero, special-cased.
//     2. The exact "small prime" set, listed by value with `|`.
//     3. Anything else in 1..=9 falls to "single digit".
//     4. The 10..=99 range catches the rest of two-digit territory.
//     5. `_` mops up negatives and 100+.
//
//   Order matters here BECAUSE patterns overlap (3, 5, 7 are both
//   primes and single digits) — Rust always picks the first matching
//   arm, so listing primes first gives them priority.
//
//   `_x @ 1..=9` shows the `@` binding form. We named it `_x` to silence
//   the unused-variable warning while still demonstrating the syntax.
//   Drop the leading underscore the moment you actually use the value.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Nested `if/else` chains
//     - Loses the visual "table" structure. Match patterns scan top-to-
//       bottom in a way reviewers can read at a glance.
//
//   `if (1..=9).contains(&n)` style
//     - Fine for one check, but you give up `@` bindings and the
//       compiler's pattern-overlap analysis.
//
// SUBTLETY:
//   `..=` is "range pattern", not the regular range expression. In
//   pattern position you cannot use a runtime value as a bound — only
//   constants. `1..=MAX_DIGIT` works if MAX_DIGIT is `const`; a `let`
//   binding does not.
