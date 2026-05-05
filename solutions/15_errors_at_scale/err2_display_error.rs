// SOLUTION — err2_display_error

use std::fmt;

#[derive(Debug)]
enum MyError {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound      => write!(f, "not found"),
            MyError::Invalid(msg)  => write!(f, "invalid: {msg}"),
            MyError::Io(inner)     => write!(f, "io error: {inner}"),
        }
    }
}

impl std::error::Error for MyError {}

fn report(e: &MyError) -> String {
    format!("{}", e)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The `Display` impl is the "human" projection of your error. It's
//   what shows up in CLI messages and log lines, so the wording
//   matters: short, lowercase, no trailing punctuation, no "Error: "
//   prefix (the caller adds that). Each variant gets one tight phrase.
//   For wrapped errors like `Io(inner)`, we delegate to the inner
//   error's own Display via `{inner}` so the full chain reads
//   naturally: "io error: file not found".
//
//   Each match arm's expression IS a `write!(f, ...)` call, which
//   already returns `fmt::Result`. There's no `?`, no `Ok(())` — the
//   macro itself IS the function's return value. New Rustaceans
//   often write `write!(...)?; Ok(())` reflexively; trim that.
//
//   `impl std::error::Error for MyError {}` (empty braces) is the
//   smallest legal impl. The trait has two methods (`source`,
//   `description`) and BOTH have defaults. We accept the defaults
//   here because we're not yet exposing a source chain. err3 will
//   override `source` to reveal the wrapped `io::Error`.
//
//   The supertraits of `Error` are `Debug + Display`, so this impl
//   only compiles after both of those are in place. The compiler
//   error you'd get for forgetting one ("`MyError` doesn't implement
//   `std::fmt::Display`") points you straight at the missing piece.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Reusing Debug as Display:
//       impl fmt::Display for MyError {
//           fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//               write!(f, "{:?}", self)
//           }
//       }
//   Compiles, but exposes implementation details (variant names,
//   inner field structure) to end-users. Always write a real Display.
//
//   Stringifying inside the variant:
//       MyError::Io(format!("{}", e))   // bad — drops the original
//   We covered this in err1: keep the structured cause so callers
//   can reach in.
//
//   Implementing `Error` with an explicit `source` returning `None`:
//       fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
//   That's just the default. Empty impl is more honest about
//   "we accept the defaults".
//
//   Including punctuation in Display: `"not found."` or `"NOT FOUND"`.
//   The convention is lowercase, no trailing punctuation — it's a
//   PHRASE that the caller composes into a sentence: `eprintln!("error:
//   {}", e)` reads as "error: not found", not "error: NOT FOUND.".
//
// SUBTLETY:
//   `Display` for `io::Error` (which we re-emit) is implemented in
//   std and produces messages like "file not found" or "permission
//   denied" — not the underlying error code. That's why we test
//   against `"io error: boom"`: the `io::Error` we constructed with
//   `Error::new(_, "boom")` displays as "boom".
//
//   In a real public-API crate you'd add `#[non_exhaustive]` to the
//   enum so adding a fourth variant later isn't a breaking change
//   for downstream `match` users.
