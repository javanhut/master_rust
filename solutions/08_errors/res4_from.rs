// SOLUTION — res4_from

use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    Other(String),
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn parse_then_check(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse::<i32>()?;
    if n < 0 {
        return Err(MyError::Other("negative".to_string()));
    }
    Ok(n)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The error enum names the categories your function actually
//   distinguishes — here, "the input wasn't a number" vs "the input
//   was a number but failed our domain rule". That's what callers
//   will pattern-match on. Each variant carries the data needed to
//   diagnose the failure.
//
//   `#[derive(Debug)]` is mandatory in practice — `Result::unwrap`
//   prints the error using `Debug` when it panics, and `assert_eq!`
//   uses `Debug` for its diff messages. Skipping the derive means
//   you can't unwrap or assert on `Result<_, MyError>` without ugly
//   workarounds.
//
//   `impl From<ParseIntError> for MyError` is the wiring that lets
//   `s.parse::<i32>()?` work in a `Result<_, MyError>` function. The
//   `?` operator's desugaring CALLS `From::from` on the inner error;
//   without the impl you'd get a "the trait `From<ParseIntError>` is
//   not implemented" compile error.
//
//   Direct construction — `Err(MyError::Other("...".to_string()))` —
//   is what you do when the error originates HERE rather than from a
//   nested fallible call. There's nothing to convert; we're producing
//   the variant ourselves.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Manual translation with `.map_err`:
//       let n: i32 = s.parse::<i32>().map_err(MyError::Parse)?;
//     Works perfectly, no `From` impl needed. Use this when you only
//     have ONE call site and don't want to commit to an `impl From`
//     globally. For multiple call sites, `From` keeps the noise out
//     of the function bodies.
//
//   Stringly-typed errors:
//       enum MyError { Msg(String) }
//   Tempting for tiny programs, but you lose the ability to handle
//   different failure categories DIFFERENTLY at a `match` later. Once
//   the error becomes a string it's just a log line — callers can't
//   reason about it. Carry the structured cause whenever you can.
//
//   `Box<dyn std::error::Error>`:
//     The "I want everything to fit" escape hatch. Dynamic dispatch,
//     loses the variant identity. We deliberately avoid it here —
//     chapter 15 introduces `Box<dyn Error>`, `thiserror`, and
//     `anyhow` as the production-grade tools for "errors at scale".
//
// SUBTLETY:
//   The `From::from` call inside `?` is INFERRED from the function's
//   return type. The compiler knows the function returns
//   `Result<_, MyError>`, so `?` looks for `From<ChildErr> for
//   MyError`. If you write `?` in a function returning the WRONG
//   error type, you get a compile error pointing at the missing
//   `From` impl — a great signal of what to add next.
//
//   Adding more variants later (`Io(io::Error)`, `NotFound`, ...) is
//   purely additive: write the variant, write the `impl From`, and
//   every existing `?` keeps working. New call sites get the
//   conversion for free.
