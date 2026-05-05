// =============================================================================
//  err3 — multiple `From` impls and the `?` convergence point
// =============================================================================
//
// In err1 you BUILT the enum. In err2 you taught it Display + Error.
// Now we wire up the conversions so a single function can use `?` on
// errors of DIFFERENT underlying types and have them all converge into
// `MyError`.
//
// REVIEW: how `?` invokes `From`
// ──────────────────────────────
//     match expr {
//         Ok(v)  => v,
//         Err(e) => return Err(From::from(e)),
//     }
//
// In a function returning `Result<T, MyError>`, the `From::from(e)`
// looks for `impl From<TypeOfE> for MyError`. Provide one impl per
// foreign error you want `?` to lift.
//
// THE GOAL FOR THIS EXERCISE
// ──────────────────────────
// We want a function `parse_then_lookup(s: &str) -> Result<&'static str, MyError>`
// that:
//
//   1. parses `s` to a `u32`         — failure: ParseIntError
//   2. translates that number to a "tier"
//        1 → "bronze"
//        2 → "silver"
//        3 → "gold"
//        anything else → MyError::NotFound
//   3. additionally rejects parsed values > 1000 with
//      MyError::Invalid("too large")
//
// All three failures funnel through ONE return type via `?`.
//
// WHAT YOU'LL ADD
// ───────────────
//   - `impl From<std::num::ParseIntError> for MyError` mapping into
//     `MyError::Parse(_)`. (Yes, we're adding a fourth variant, `Parse`,
//     specifically because the parse failure is its own category — not
//     "invalid input we generated" but "third-party parser said no".)
//   - An override of `Error::source` that exposes the inner io / parse
//     error when the variant carries one. This is the FIRST exercise
//     where we implement `source`, so look closely.
//
// SOURCE CHAIN — CRYSTALLISED
// ───────────────────────────
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             MyError::Io(e)    => Some(e),
//             MyError::Parse(e) => Some(e),
//             _                 => None,
//         }
//     }
//
//   - Return `Some(e)` for variants that wrap another error.
//   - Return `None` for variants that originate HERE
//     (`NotFound`, `Invalid(_)` — there's no "underlying cause").
//   - The `dyn std::error::Error + 'static` return type is verbatim
//     — both `io::Error` and `ParseIntError` satisfy it.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Add a `Parse(std::num::ParseIntError)` variant to `MyError`.
//   - Implement `From<std::num::ParseIntError> for MyError`.
//   - Override `Error::source` so `Io` and `Parse` expose their inner.
//   - Implement `parse_then_lookup` using `?` for the parse step and
//     direct `Err(...)` returns for the domain checks.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
    ???(ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound      => write!(f, "not found"),
            MyError::Invalid(msg)  => write!(f, "invalid: {msg}"),
            MyError::Io(inner)     => write!(f, "io error: {inner}"),
            MyError::Parse(inner)  => write!(f, "parse error: {inner}"),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e)    => Some(???),
            MyError::Parse(e) => Some(???),
            _                 => None,
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::???(e)
    }
}

fn parse_then_lookup(s: &str) -> Result<&'static str, MyError> {
    // `?` here calls `From::from` on a ParseIntError to turn it into MyError::Parse.
    let n: u32 = s.parse::<u32>()???;

    if n > 1000 {
        return Err(MyError::Invalid("too large".to_string()));
    }

    match n {
        1 => Ok("bronze"),
        2 => Ok("silver"),
        3 => Ok("gold"),
        _ => Err(MyError::???),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn happy_paths() {
        assert_eq!(parse_then_lookup("1").unwrap(), "bronze");
        assert_eq!(parse_then_lookup("2").unwrap(), "silver");
        assert_eq!(parse_then_lookup("3").unwrap(), "gold");
    }

    #[test]
    fn parse_failure_lifts_via_question_mark() {
        match parse_then_lookup("oops") {
            Err(MyError::Parse(_)) => {}
            other => panic!("expected Parse(_), got {:?}", other),
        }
    }

    #[test]
    fn too_large_is_invalid() {
        match parse_then_lookup("9999") {
            Err(MyError::Invalid(msg)) => assert_eq!(msg, "too large"),
            other => panic!("expected Invalid, got {:?}", other),
        }
    }

    #[test]
    fn unknown_tier_is_not_found() {
        match parse_then_lookup("7") {
            Err(MyError::NotFound) => {}
            other => panic!("expected NotFound, got {:?}", other),
        }
    }

    #[test]
    fn source_is_set_for_parse() {
        let e = parse_then_lookup("oops").unwrap_err();
        // The source should point at the inner ParseIntError.
        assert!(e.source().is_some(), "Parse variant should expose source()");
    }

    #[test]
    fn source_is_none_for_invalid() {
        let e = MyError::Invalid("x".to_string());
        assert!(e.source().is_none(), "Invalid variant has no underlying cause");
    }
}

fn main() {}
