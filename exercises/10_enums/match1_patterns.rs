// =============================================================================
//  match1 — pattern toolkit: alternatives, ranges, `@` bindings
// =============================================================================
//
// `match` patterns are a tiny language unto themselves. This exercise drills
// three of the most useful operators.
//
// 1.  `|` — ALTERNATIVES (or-patterns)
//
//        match c {
//            'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
//            _ => "other",
//        }
//
//     Read `|` as "or". The arm fires if the value matches ANY of the
//     listed patterns.
//
// 2.  `..=`  /  `..` — RANGE PATTERNS
//
//        match n {
//            0       => "zero",
//            1..=9   => "single digit",     // inclusive at both ends
//            10..100 => "small",            // half-open: 10..=99
//            _       => "big",
//        }
//
//     Inclusive (`..=`) is the most useful in `match`. Half-open (`..`)
//     in pattern position requires Rust 1.80+; we'll mostly use `..=`.
//
// 3.  `@` — BINDING WHILE TESTING
//
//        match n {
//            x @ 1..=9 => println!("{x} is a single digit"),
//            _ => (),
//        }
//
//     `name @ pattern` says: "if the value matches `pattern`, ALSO bind
//     it to `name` so the arm body can use it." Without `@` you'd have
//     to choose: test the range OR get the value, not both.
//
// MEMORISE THIS HIERARCHY
//   - `_`        match anything, throw it away
//   - `name`     match anything, bind it to `name`
//   - `pattern`  match shape only — fails to fire if mismatched
//   - `name @ pattern` match shape AND bind
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `classify(n: i32) -> &'static str`:
//   - n == 0                       → "zero"
//   - n == 1, 2, 3, 5, 7, or 11    → "small prime"
//   - n in 1..=9                   → "single digit"     (use `@` to bind, even if unused)
//   - n in 10..=99                 → "double digit"
//   - everything else              → "other"
//
// Note the order: arms are tried top-to-bottom. The "small prime" arm fires
// BEFORE the "single digit" arm, so 7 → "small prime", but 4 → "single digit".
//
// Use `|` for the prime list, `..=` for the ranges, and `x @ 1..=9` for the
// single-digit arm so you have a name to work with.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 ??? 2 ??? 3 ??? 5 ??? 7 ??? 11 => "small prime",
        _x @ ??? => "single digit",
        ??? => "double digit",
        _ => ???,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn zero()        { assert_eq!(classify(0), "zero"); }
    #[test] fn primes()      {
        for &p in &[1, 2, 3, 5, 7, 11] { assert_eq!(classify(p), "small prime", "{p}"); }
    }
    #[test] fn singles()     { assert_eq!(classify(4), "single digit"); assert_eq!(classify(9), "single digit"); }
    #[test] fn doubles()     { assert_eq!(classify(10), "double digit"); assert_eq!(classify(99), "double digit"); }
    #[test] fn others()      { assert_eq!(classify(100), "other"); assert_eq!(classify(-1), "other"); }
}

fn main() {}
