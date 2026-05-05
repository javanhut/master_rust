// =============================================================================
//  err2 — `Display` and `std::error::Error`
// =============================================================================
//
// In err1 you defined a `MyError` enum with `#[derive(Debug)]`. That's
// enough to PANIC with — but not enough to be a first-class error.
// A first-class error in Rust implements two traits beyond Debug:
//
//   1. `std::fmt::Display`  — the human-friendly text. `{}` formatting,
//                             error logs, user-facing CLI messages.
//   2. `std::error::Error`  — the marker trait that says "I'm an error
//                             value." It also exposes `source()` for
//                             chaining a CAUSE to a wrapper error.
//
// Why both? Because Debug is for programmers (`MyError::Io(Os { code:
// 2, kind: NotFound, ... })`) and Display is for humans ("io error:
// file not found"). Confusing the two annoys users on one side and
// hides information on the other.
//
// THE SHAPE OF A `Display` IMPL
// ─────────────────────────────
// You match on `self`, write to a formatter, and return `fmt::Result`:
//
//     use std::fmt;
//
//     impl fmt::Display for MyError {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             match self {
//                 MyError::NotFound        => write!(f, "not found"),
//                 MyError::Invalid(msg)    => write!(f, "invalid: {msg}"),
//                 MyError::Io(inner)       => write!(f, "io error: {inner}"),
//             }
//         }
//     }
//
// `write!` returns `fmt::Result` (which is `Result<(), fmt::Error>`),
// so each match arm's tail expression IS the return value. No `?`,
// no `Ok(())` — the `write!` macro IS the answer.
//
// THE SHAPE OF AN `Error` IMPL
// ────────────────────────────
//     impl std::error::Error for MyError {}
//
// That's it. The trait has two methods (`source` and the deprecated
// `description`) but BOTH have default implementations. The empty impl
// declares "this type is an error"; that alone is what other libraries
// (logging, web frameworks, ?-into-`Box<dyn Error>`) hook into.
//
// `Error` requires `Debug + Display` as supertraits, so you must have
// both impls before you can write `impl Error for MyError {}`.
//
// SOURCE CHAINS — A QUICK PEEK
// ────────────────────────────
// In production, when one error wraps another, you implement `source`
// to expose the inner error:
//
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             MyError::Io(e) => Some(e),     // io::Error implements Error
//             _              => None,
//         }
//     }
//
// Loggers and pretty-printers walk `source()` repeatedly to print
// chains like:
//     "configuration error: failed to read config: io error: file not found"
//      |________________| |____________________| |___________________|
//        outer Display      middle Display          deepest Display
//
// We'll write the empty impl in this exercise (the tests don't probe
// the chain), and you'll see explicit `source()` impls in err3 / err4.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Re-state the `MyError` enum from err1 (NotFound, Invalid(String),
//     Io(std::io::Error)) and `#[derive(Debug)]` it.
//   - Implement `std::fmt::Display` with the three messages shown above.
//     Match exactly: "not found", "invalid: <msg>", "io error: <inner>".
//   - Implement `std::error::Error` with an empty body.
//   - Write the helper `report(e: &MyError) -> String` that just
//     formats `e` with `{}` (the Display version) — the tests use it.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

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
            MyError::NotFound      => write!(f, "???"),
            MyError::Invalid(msg)  => write!(f, "invalid: {???}"),
            MyError::Io(inner)     => write!(f, "io error: {???}"),
        }
    }
}

impl std::error::Error for MyError ??? // empty body — the trait's defaults are fine

fn report(e: &MyError) -> String {
    // format! with `{}` calls Display, NOT Debug.
    format!("{???}", e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_not_found() {
        assert_eq!(report(&MyError::NotFound), "not found");
    }

    #[test]
    fn display_invalid() {
        let e = MyError::Invalid("empty key".to_string());
        assert_eq!(report(&e), "invalid: empty key");
    }

    #[test]
    fn display_io_includes_inner() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "boom");
        let e = MyError::Io(io_err);
        // io::Error's Display is the message we passed in ("boom").
        assert_eq!(report(&e), "io error: boom");
    }

    #[test]
    fn implements_error_trait() {
        // If the impl is missing or wrong, this won't compile.
        fn assert_is_error<E: std::error::Error>(_: &E) {}
        assert_is_error(&MyError::NotFound);
    }
}

fn main() {}
