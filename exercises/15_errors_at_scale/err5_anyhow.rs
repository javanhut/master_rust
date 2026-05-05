// =============================================================================
//  err5 — `anyhow`, by hand
// =============================================================================
//
// `thiserror` (err4) is the "I'm writing a LIBRARY, callers will match
// on my variants" tool. `anyhow` is its sibling for the OTHER half of
// the world: APPLICATIONS — the binary at the top of the call stack
// that just wants to bubble errors up with context, log them, and
// exit with a nonzero status.
//
// WHAT `anyhow` LOOKS LIKE
// ────────────────────────
// Add to Cargo.toml:
//
//     [dependencies]
//     anyhow = "1"
//
// Then:
//
//     use anyhow::{Context, Result};
//
//     fn load(path: &str) -> Result<u32> {
//         let raw = std::fs::read_to_string(path)
//             .with_context(|| format!("reading {path}"))?;
//         let n: u32 = raw.trim().parse()
//             .with_context(|| format!("parsing {raw:?} as u32"))?;
//         Ok(n)
//     }
//
// Three magic things happen here:
//
//   1. `Result<T>` is a type alias for `Result<T, anyhow::Error>`.
//      You write the success type only.
//   2. `anyhow::Error` is a polished `Box<dyn Error + Send + Sync +
//      'static>`. ANY type implementing `std::error::Error` (and
//      Send+Sync) auto-converts via `?` — no hand-written `From` impl,
//      because the blanket impl already exists.
//   3. `.with_context(|| "...")` wraps the underlying error with a
//      message, building a SOURCE CHAIN as the error bubbles up.
//      `eprintln!("{:#}", e)` prints the chain.
//
// WHAT WE'LL BUILD HERE (NO CARGO)
// ────────────────────────────────
// The exercise runner has no Cargo, so we can't actually pull in
// `anyhow`. But the SHAPE of `anyhow::Error` is just a boxed trait
// object, so we can imitate the API in plain std:
//
//     type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
//     type Result<T> = std::result::Result<T, DynError>;
//
// Why those bounds?
//   - `dyn std::error::Error`  — Display + Debug + source().
//   - `Send + Sync`            — so the error can cross threads.
//                                 (anyhow requires this.)
//   - `'static`                 — owns its data; no borrowed references.
//
// Once you have `Box<dyn Error + ...>` as your error type, `?` works
// across HETEROGENEOUS error types automatically. There's a blanket
// impl `From<E> for Box<dyn Error + ...>` for any `E: Error`, so any
// concrete error you `?` is auto-boxed.
//
// CONTEXT, BY HAND
// ────────────────
// We're going to manually implement an `Errored` struct that wraps
// another error with a message — this is what `with_context` does.
//
//     struct Errored {
//         msg: String,
//         source: DynError,
//     }
//
// Implement Display + Debug + Error::source for it, and you have your
// own one-line `with_context`.
//
// WHEN TO REACH FOR `anyhow`
// ──────────────────────────
//   - Binary crates / CLIs / glue code where the error type is
//     "whatever went wrong, plus some context".
//   - Test code — `#[test] fn t() -> anyhow::Result<()> { ... }` lets
//     you `?` everything.
//   - Quick scripts where defining a custom enum would be overkill.
//
// When NOT to reach for it:
//   - Library APIs where callers might want to MATCH on variants. Use
//     `thiserror` instead. (Or both: library uses thiserror, the
//     binary calling it uses anyhow.)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define the `DynError` and `Result<T>` aliases.
//   - Implement `parse_pair(s: &str) -> Result<(u32, u32)>` that:
//       splits `s` on ',' (use `split_once`),
//       parses both halves as u32 with `?`,
//       returns Ok((a, b)).
//     The two `?` calls handle DIFFERENT error types (the missing-
//     comma case and the parse case), but both fit Box<dyn Error+...>.
//   - For the "no comma" case: build an io::Error with
//       std::io::Error::new(std::io::ErrorKind::InvalidInput, "no comma")
//     and `?` it (or convert directly with `.into()`).
//   - Implement a `with_context` helper as a method on Result via
//     a small extension trait (`Context`).
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

use std::fmt;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, ???>;

// A small wrapper error that carries a message + a wrapped cause.
// Roughly what `anyhow::Context::context` builds internally.
#[derive(Debug)]
struct Errored {
    msg: String,
    source: DynError,
}

impl fmt::Display for Errored {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Errored {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.???)
    }
}

// Extension trait that adds `.with_context` to any `Result<T, E>`
// whose E is convertible into our DynError. Mirrors `anyhow::Context`.
trait Context<T> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: Into<DynError>,
{
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        match self {
            Ok(v)  => Ok(v),
            Err(e) => Err(Box::new(Errored {
                msg: f(),
                source: e.into(),
            })),
        }
    }
}

fn parse_pair(s: &str) -> Result<(u32, u32)> {
    let (a, b) = s
        .split_once(',')
        // No comma — build an io::Error and let ? auto-box it into DynError.
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "no comma",
        ))???;

    let a: u32 = a.parse::<u32>()
        .with_context(|| format!("parsing left half {:?}", a))???;
    let b: u32 = b.parse::<u32>()
        .with_context(|| format!("parsing right half {:?}", b))???;

    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        let (a, b) = parse_pair("3,4").unwrap();
        assert_eq!((a, b), (3, 4));
    }

    #[test]
    fn no_comma_is_io_input() {
        let e = parse_pair("nocomma").unwrap_err();
        // The boxed error should still be downcastable to io::Error.
        let io = e.downcast_ref::<std::io::Error>()
            .expect("expected an io::Error");
        assert_eq!(io.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn bad_left_carries_context() {
        let e = parse_pair("oops,4").unwrap_err();
        // Top-level message comes from `with_context`.
        let msg = format!("{}", e);
        assert!(msg.contains("parsing left half"), "got: {msg}");
        // And the source chain leads back to a ParseIntError.
        let mut cur: Option<&(dyn std::error::Error + 'static)> = Some(&*e);
        let mut found_parse = false;
        while let Some(c) = cur {
            if c.downcast_ref::<std::num::ParseIntError>().is_some() {
                found_parse = true;
                break;
            }
            cur = c.source();
        }
        assert!(found_parse, "source chain should reach the ParseIntError");
    }

    #[test]
    fn bad_right_also_works() {
        let e = parse_pair("3,oops").unwrap_err();
        assert!(format!("{}", e).contains("parsing right half"));
    }
}

fn main() {}
