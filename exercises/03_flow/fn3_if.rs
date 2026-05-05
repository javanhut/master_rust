// =============================================================================
//  fn3 — `if` deep dive
// =============================================================================
//
// `if` rules to internalise:
//
//   1. The CONDITION must be a `bool`. No "truthy" — `if 1 { }` is a type
//      error.  Compare explicitly:  `if x != 0 { }`,  `if !v.is_empty() { }`
//
//   2. NO PARENTHESES needed around the condition — `if x > 5 { ... }`.
//
//   3. When `if/else` is used as an EXPRESSION, every arm must return the
//      same type, and there must BE an `else` arm (otherwise the type would
//      have to be `()`).
//
//   4. `if let` is a special form for pattern-matching with one pattern:
//
//          if let Some(x) = maybe {
//              // x is bound here
//          }
//
//      We'll meet that properly when we cover Option<T>.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `classify` that takes an `i32` and returns:
//   "negative" if x < 0
//   "zero"     if x == 0
//   "positive" if x > 0

// I AM NOT DONE

fn classify(x: i32) -> &'static str {
    if x < 0 {
        ???
    } else if x ??? 0 {
        "zero"
    } else {
        ???
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn neg()  { assert_eq!(classify(-5), "negative"); }
    #[test] fn zero() { assert_eq!(classify(0),  "zero"); }
    #[test] fn pos()  { assert_eq!(classify(7),  "positive"); }
    #[test] fn min()  { assert_eq!(classify(i32::MIN), "negative"); }
    #[test] fn max()  { assert_eq!(classify(i32::MAX), "positive"); }
}

fn main() {}
