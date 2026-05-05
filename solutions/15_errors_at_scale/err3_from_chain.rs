// SOLUTION — err3_from_chain

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
    Parse(ParseIntError),
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
            MyError::Io(e)    => Some(e),
            MyError::Parse(e) => Some(e),
            _                 => None,
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn parse_then_lookup(s: &str) -> Result<&'static str, MyError> {
    let n: u32 = s.parse::<u32>()?;

    if n > 1000 {
        return Err(MyError::Invalid("too large".to_string()));
    }

    match n {
        1 => Ok("bronze"),
        2 => Ok("silver"),
        3 => Ok("gold"),
        _ => Err(MyError::NotFound),
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `From<ParseIntError> for MyError` is the single-line "wiring"
//   that turns `s.parse::<u32>()?` into a working expression inside
//   a function returning `Result<_, MyError>`. The `?` desugaring
//   calls `From::from` on the inner `ParseIntError`; without this
//   impl you'd get "the trait `From<ParseIntError>` is not
//   implemented for `MyError`" pointing at the `?`.
//
//   Naming the variant `Parse` (rather than overloading `Invalid`)
//   matters: callers that want to react SPECIFICALLY to "the user
//   typed something non-numeric" can match on `Parse(_)` without
//   pulling apart strings inside `Invalid`. Each variant is a
//   distinct category of failure that callers might handle
//   differently.
//
//   Overriding `source()` exposes the underlying error to logging
//   and pretty-printing layers. A pretty-printer walks `e.source()`
//   in a loop, printing each Display in turn — so a config error
//   wrapping a parse error wrapping the actual digit-parse failure
//   shows the full causal chain. Variants that ORIGINATE the failure
//   (`NotFound`, `Invalid`) return `None` because there's nothing
//   underneath to chain to.
//
//   The body of `parse_then_lookup` reads as a normal happy path:
//   `?` for foreign errors, direct `Err(...)` for domain checks.
//   No nested matches. Compare to the manual `match` form:
//       let n = match s.parse::<u32>() {
//           Ok(n)  => n,
//           Err(e) => return Err(MyError::Parse(e)),
//       };
//   Three lines of noise vs. one `?`.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `.map_err(MyError::Parse)?` at the call site:
//       let n = s.parse::<u32>().map_err(MyError::Parse)?;
//   Works without the `From` impl. Use this when there's exactly ONE
//   call site and you don't want to commit to a global conversion.
//   Once you have two or more, the `From` impl pays for itself.
//
//   Stuffing the parse error into `Invalid`:
//       Invalid(format!("{}", parse_err))
//   Loses `parse_err.kind()` (PosOverflow, InvalidDigit, ...) and
//   breaks `source()` chaining — the wrapped error becomes a flat
//   string the caller can no longer reason about.
//
//   Auto-deriving `Error` with macros not available here:
//       #[derive(thiserror::Error, Debug)]
//   That's exactly what err4 explores. The macros generate the
//   Display + From impls for you; the result is identical to what
//   we wrote by hand.
//
// SUBTLETY:
//   `From::from` is INVOKED via `?`, but `?` only fires on `Err`. On
//   the happy path the call is zero-cost (just an unwrap). That's
//   why `From` impls for error types are a free abstraction in
//   practice.
//
//   Adding a fifth variant later (say `Network(reqwest::Error)`) is
//   purely additive: write the variant, the `Display` arm, the
//   `From` impl, and update `source()`. Every existing `?` in your
//   code keeps working — and any new call site can `?` a network
//   error into your enum without touching the function bodies in
//   between.
