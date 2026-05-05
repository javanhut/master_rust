// =============================================================================
//  err4 — `thiserror`, by hand
// =============================================================================
//
// You've now written, for one error enum:
//   - `#[derive(Debug)]`
//   - `impl Display`            (a match arm per variant)
//   - `impl std::error::Error`  (with `source()` for wrapped variants)
//   - `impl From<X> for MyError` for each foreign X
//
// That is a LOT of boilerplate for a five-variant enum. In production
// Rust, almost nobody writes those impls by hand. They use the
// `thiserror` crate.
//
// WHAT `thiserror` LOOKS LIKE
// ───────────────────────────
// Add to Cargo.toml:
//
//     [dependencies]
//     thiserror = "1"
//
// Then:
//
//     use thiserror::Error;
//     use std::num::ParseIntError;
//
//     #[derive(Debug, Error)]
//     pub enum AppError {
//         #[error("not found")]
//         NotFound,
//
//         #[error("invalid: {0}")]
//         Invalid(String),
//
//         #[error("io error: {0}")]
//         Io(#[from] std::io::Error),
//
//         #[error("parse error: {0}")]
//         Parse(#[from] ParseIntError),
//     }
//
// That's it. Six lines per variant — and from those six lines the
// `Error` derive GENERATES:
//
//     impl Display for AppError { ... match arms ... }
//     impl std::error::Error for AppError {
//         fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
//     }
//     impl From<std::io::Error>  for AppError { ... }
//     impl From<ParseIntError>   for AppError { ... }
//
// The `#[error("...")]` attribute becomes the `Display` arm. `{0}`
// refers to the tuple field; `{name}` works for named-field variants.
// `#[from]` triggers a `From<InnerType> for AppError` impl AND wires
// `source()` to expose that field.
//
// WHY WRITE IT BY HAND HERE?
// ──────────────────────────
// The exercise runner has no Cargo, so we can't actually pull in
// `thiserror`. But that's fine — by writing the impls by hand, you
// internalise WHAT the macro generates, which makes its compile errors
// and edge cases much easier to debug later. (When `#[error("{x}")]`
// fails because `x` isn't a field, the message is much clearer once
// you know the macro is just emitting `write!(f, "{}", self.x)`.)
//
// Once you reach for `thiserror` in a real project you go from ~50
// hand-written lines for a fat error enum to ~10. Same behaviour,
// less code to read.
//
// HOW IT FITS WITH `anyhow` (PREVIEW OF err5)
// ───────────────────────────────────────────
// Two crates, two roles. Use `thiserror` when you're WRITING A LIBRARY
// — you want to expose a concrete, named error type so callers can
// match on variants. Use `anyhow` when you're WRITING AN APPLICATION
// (a binary) — you don't care which variant it was, you just want to
// bubble the error up with context. The two compose: a library exports
// `thiserror`-derived enums; the binary that uses the library handles
// them via `anyhow::Result<T>`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Re-derive, by hand, what `#[derive(thiserror::Error)]` would emit
// for the enum below. Specifically:
//
//   - Define `AppError` with the FOUR variants commented next to each
//     `#[error("...")]` line in the imagined macro version above.
//   - `#[derive(Debug)]` (the macro adds it for you; here you write it).
//   - `impl Display for AppError` — one match arm per variant, matching
//     the `#[error("...")]` strings exactly.
//   - `impl std::error::Error for AppError` with a `source()` that
//     exposes `Io`'s and `Parse`'s inner errors.
//   - `impl From<std::io::Error>  for AppError` (the `#[from]` line).
//   - `impl From<std::num::ParseIntError> for AppError` (ditto).
//
// Then implement `load_number(s: &str) -> Result<u32, AppError>`:
//   - `s.parse::<u32>()?` — the `From<ParseIntError>` impl lifts the err.
//   - if the parsed value is 0, return Err(AppError::Invalid("zero".into()))
//   - otherwise Ok(n).
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    // #[error("not found")]
    NotFound,
    // #[error("invalid: {0}")]
    Invalid(String),
    // #[error("io error: {0}")]
    Io(std::io::Error),
    // #[error("parse error: {0}")]
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound      => write!(f, "???"),
            AppError::Invalid(msg)  => write!(f, "invalid: {???}"),
            AppError::Io(inner)     => write!(f, "io error: {???}"),
            AppError::Parse(inner)  => write!(f, "parse error: {???}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e)    => Some(???),
            AppError::Parse(e) => Some(???),
            _                  => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::???(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::???(e)
    }
}

fn load_number(s: &str) -> Result<u32, AppError> {
    let n: u32 = s.parse::<u32>()???;
    if n == 0 {
        return Err(AppError::Invalid("???".to_string()));
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn happy() {
        assert_eq!(load_number("42").unwrap(), 42);
    }

    #[test]
    fn parse_err_is_lifted() {
        match load_number("nope") {
            Err(AppError::Parse(_)) => {}
            other => panic!("expected Parse(_), got {:?}", other),
        }
    }

    #[test]
    fn zero_is_invalid() {
        match load_number("0") {
            Err(AppError::Invalid(msg)) => assert_eq!(msg, "zero"),
            other => panic!("expected Invalid, got {:?}", other),
        }
    }

    #[test]
    fn display_strings_match_macro() {
        assert_eq!(format!("{}", AppError::NotFound), "not found");
        assert_eq!(
            format!("{}", AppError::Invalid("oops".into())),
            "invalid: oops"
        );
    }

    #[test]
    fn source_chains_through_parse() {
        let e = load_number("nope").unwrap_err();
        assert!(e.source().is_some());
    }

    #[test]
    fn from_io_error_works() {
        // Constructing AppError via `?` from io::Error means the From impl is wired up.
        fn try_io() -> Result<(), AppError> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "boom"))?
        }
        match try_io() {
            Err(AppError::Io(_)) => {}
            other => panic!("expected Io(_), got {:?}", other),
        }
    }
}

fn main() {}
