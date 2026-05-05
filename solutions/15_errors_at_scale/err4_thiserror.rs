// SOLUTION — err4_thiserror (manually written, simulating the macro)

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound      => write!(f, "not found"),
            AppError::Invalid(msg)  => write!(f, "invalid: {msg}"),
            AppError::Io(inner)     => write!(f, "io error: {inner}"),
            AppError::Parse(inner)  => write!(f, "parse error: {inner}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e)    => Some(e),
            AppError::Parse(e) => Some(e),
            _                  => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn load_number(s: &str) -> Result<u32, AppError> {
    let n: u32 = s.parse::<u32>()?;
    if n == 0 {
        return Err(AppError::Invalid("zero".to_string()));
    }
    Ok(n)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   What you just wrote is, byte-for-byte, what `#[derive(thiserror::
//   Error)]` would emit for the enum at the top. Each `#[error("...")]`
//   attribute became one Display arm; each `#[from]` produced a
//   `From` impl AND wired `source()` to expose that field.
//
//   In a real crate the comparable code is six lines per variant:
//
//       #[error("invalid: {0}")]
//       Invalid(String),
//
//       #[error("io error: {0}")]
//       Io(#[from] std::io::Error),
//
//   ...and the macro generates everything else. That's the trade.
//   Knowing what the macro generates makes its compile errors
//   trivial to debug — when `#[error("{x}")]` complains about a
//   missing field, it's because the macro is trying to call
//   `write!(f, "{}", self.x)`.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Hand-written impls with stringly-typed wrappers:
//       Io(String)   // bad — no source(), no kind()
//   We covered this in earlier exercises. Don't stringify on the
//   way in.
//
//   Reaching for `Box<dyn Error>` instead of a custom enum:
//       fn load_number(s: &str) -> Result<u32, Box<dyn std::error::Error>>
//   Loses the variant tag — callers can't match on "was this a parse
//   error or a domain failure?". For LIBRARIES this is the wrong
//   trade; for binaries it's often fine, and that's exactly what
//   `anyhow` provides (next exercise).
//
//   Manually delegating Display via Debug:
//       write!(f, "{:?}", self)
//   The same anti-pattern from err2. Always write a real Display.
//
// SUBTLETY:
//   The `#[from]` macro attribute does TWO things: emit the `From`
//   impl AND mark the field as the source. Without `#[from]`, you'd
//   write the variant as `Io { source: io::Error }` plus a
//   `#[source]` attribute on the field, or implement source() by
//   hand. In real codebases ~95% of error variants are simple
//   wrappers and `#[from]` covers them all.
//
//   `thiserror` lives at compile time only — it generates code and
//   then disappears. There's no runtime dependency, no allocation
//   overhead, nothing in your binary that wasn't there if you'd
//   written the impls yourself. Same goes for `anyhow::Error`'s
//   trait-object inside (different story, see err5).
//
//   When you migrate this file to a real crate:
//       cargo add thiserror
//   then replace the four impls with `#[derive(thiserror::Error)]`
//   and inline `#[error("...")]` and `#[from]` attributes on the
//   enum. The function bodies — including `?` — don't change at all.
