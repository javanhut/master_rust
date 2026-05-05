// =============================================================================
//  res4 — `From` and automatic error conversion via `?`
// =============================================================================
//
// Recall the exact desugaring of `?`:
//
//     match expr {
//         Ok(v)  => v,
//         Err(e) => return Err(From::from(e)),    // ← look here
//     }
//
// The `From::from(e)` call means: "if the surrounding function's error
// type implements `From<EOfThisExpr>`, convert the small error into
// the big one for free."
//
// THE PROBLEM `From` SOLVES
// ─────────────────────────
// Real functions tend to combine MULTIPLE fallible operations:
//
//     fn load(path: &str) -> Result<i32, ???> {
//         let raw  = std::fs::read_to_string(path)?;        // io::Error
//         let n: i32 = raw.trim().parse::<i32>()?;          // ParseIntError
//         Ok(n)
//     }
//
// The two `?`s produce DIFFERENT error types. There's no single `E` we
// can put in the signature that matches both. Solution: a custom error
// enum that wraps both, plus `From` impls so `?` can lift each one.
//
//     #[derive(Debug)]
//     enum LoadError {
//         Io(std::io::Error),
//         Parse(std::num::ParseIntError),
//     }
//
//     impl From<std::io::Error> for LoadError {
//         fn from(e: std::io::Error) -> Self { LoadError::Io(e) }
//     }
//     impl From<std::num::ParseIntError> for LoadError {
//         fn from(e: std::num::ParseIntError) -> Self { LoadError::Parse(e) }
//     }
//
// Now the original `load` function compiles, with `Result<i32, LoadError>`.
// The two `?`s call the appropriate `From::from` automatically.
//
// THE PATTERN, CRYSTALLISED
// ─────────────────────────
//   1. Define a parent error enum that names every CHILD error category
//      meaningful to your domain.
//   2. `#[derive(Debug)]` so panics and tests can print it.
//   3. For each variant that wraps a foreign error, write
//      `impl From<ChildError> for ParentError`.
//   4. Functions return `Result<T, ParentError>` and use `?` freely.
//
// We're NOT introducing the `thiserror` crate or `Box<dyn Error>` here
// — those are chapter 15. This chapter teaches you what the macros are
// generating BEHIND your back, by writing it out by hand.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Define `MyError` with two variants — `Parse(ParseIntError)` and
// `Other(String)` — and write `impl From<ParseIntError> for MyError`.
//
// Then implement `parse_then_check(s)`:
//   - Parse `s` to i32 with `?` (the From impl converts the error).
//   - If the parsed value is negative, return
//       `Err(MyError::Other("negative".to_string()))` directly.
//   - Otherwise return Ok(value).
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    Other(String),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        ???
    }
}

fn parse_then_check(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse::<i32>()???;
    if n < 0 {
        return Err(???);
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn happy() { assert_eq!(parse_then_check("42").unwrap(), 42); }
    #[test] fn zero()  { assert_eq!(parse_then_check("0").unwrap(),   0); }

    #[test]
    fn negative() {
        match parse_then_check("-3") {
            Err(MyError::Other(msg)) => assert_eq!(msg, "negative"),
            other => panic!("expected Other(\"negative\"), got {:?}", other),
        }
    }

    #[test]
    fn bad_input() {
        match parse_then_check("nope") {
            Err(MyError::Parse(_)) => {} // good — `?` converted it for us
            other => panic!("expected Parse(_), got {:?}", other),
        }
    }
}

fn main() {}
