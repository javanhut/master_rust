// =============================================================================
//  err1 — a domain error enum (Debug-only, the foundation)
// =============================================================================
//
// Welcome to chapter 15 — "errors at scale". Chapter 8 taught you the
// language-level mechanics: `Option`, `Result`, the `?` operator, and
// using `From` to bubble a child error up into a parent enum. THAT
// chapter ended at "your code compiles and the tests pass."
//
// This chapter takes the next step. Real production code goes further:
//
//   - implements `std::fmt::Display` and `std::error::Error` so the
//     error can be PRINTED nicely and chained with `source()`,
//   - uses the `thiserror` crate to derive those impls in 6 lines,
//   - or reaches for `anyhow` when the caller doesn't care about
//     specific variants and just wants "something went wrong (here's
//     what)".
//
// We'll build all of that BY HAND. The runner here compiles each file
// as a single `.rs` with `rustc` — there's no Cargo.toml, so we can't
// `use thiserror::Error` for real. That's a feature, not a bug: by the
// end of the chapter you'll know exactly what those macros generate,
// and switching to the crates is just `cargo add thiserror anyhow`.
//
// THE TEACHING ARC OF THIS CHAPTER
// ────────────────────────────────
//   err1 — define a multi-variant `MyError` enum.        ← you are here
//   err2 — implement `Display` + `std::error::Error`.
//   err3 — converge multiple foreign errors via `From`.
//   err4 — imitate `#[derive(thiserror::Error)]` by hand.
//   err5 — imitate `anyhow::Error` via `Box<dyn Error>`.
//   errors_quiz — capstone: a CLI input validator.
//
// REVIEW FROM CHAPTER 8
// ─────────────────────
// `?` desugars to:
//
//     match expr {
//         Ok(v)  => v,
//         Err(e) => return Err(From::from(e)),
//     }
//
// So in a function returning `Result<T, MyError>`, every `?` will look
// for `impl From<E> for MyError` to lift the small error into yours.
//
// FIRST INGREDIENT: A WELL-NAMED ENUM
// ───────────────────────────────────
// A domain error enum names the categories your CALLER will distinguish
// — not every line of `match` they'll write, but every kind of failure
// they might handle differently. For this exercise we'll model three:
//
//     enum MyError {
//         NotFound,                  // a unit variant — no payload needed.
//         Invalid(String),           // a tuple variant — what was wrong?
//         Io(std::io::Error),        // wraps a foreign error verbatim.
//     }
//
//   - `NotFound` is shape-only — the caller distinguishes "key absent"
//     by the variant alone; no string is required.
//   - `Invalid(String)` carries a human-readable explanation. This is
//     where you'd put parser messages, validation reasons, etc.
//   - `Io(std::io::Error)` wraps the underlying foreign error so that
//     callers can `match e { MyError::Io(inner) => inner.kind(), ... }`
//     and reach into it. The original error is NOT lost.
//
// `#[derive(Debug)]` is non-negotiable. `Result::unwrap`, `assert_eq!`,
// `?` in tests — they all print the error via Debug when something
// goes wrong. Forgetting it makes your error type painful to use.
//
// Notice we are NOT yet implementing `Display` or `std::error::Error`.
// That's err2's job. For now: just structure + Debug.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Complete the `MyError` enum: `NotFound` (unit), `Invalid(String)`,
//     `Io(std::io::Error)`. Derive `Debug`.
//   - Implement `lookup(key: &str) -> Result<&'static str, MyError>` so
//     that:
//       "alpha" → Ok("first")
//       "beta"  → Ok("second")
//       ""      → Err(MyError::Invalid("empty key".to_string()))
//       _       → Err(MyError::NotFound)
//   - Implement `read_or_default(path: &str) -> Result<String, MyError>`:
//     try `std::fs::read_to_string(path)`. On Ok return the contents;
//     on Err, MANUALLY wrap the io error in `MyError::Io(...)`. We're
//     not using `?` yet — that's err3.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

#[derive(???)]
enum MyError {
    NotFound,
    Invalid(???),
    Io(???),
}

fn lookup(key: &str) -> Result<&'static str, MyError> {
    match key {
        "alpha" => Ok("first"),
        "beta"  => Ok("second"),
        ""      => Err(MyError::???("empty key".to_string())),
        _       => Err(MyError::???),
    }
}

fn read_or_default(path: &str) -> Result<String, MyError> {
    match std::fs::read_to_string(path) {
        Ok(s)  => Ok(s),
        Err(e) => Err(MyError::???(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_hit() {
        assert_eq!(lookup("alpha").unwrap(), "first");
        assert_eq!(lookup("beta").unwrap(),  "second");
    }

    #[test]
    fn lookup_empty_is_invalid() {
        match lookup("") {
            Err(MyError::Invalid(msg)) => assert_eq!(msg, "empty key"),
            other => panic!("expected Invalid, got {:?}", other),
        }
    }

    #[test]
    fn lookup_unknown_is_not_found() {
        match lookup("zzz") {
            Err(MyError::NotFound) => {}
            other => panic!("expected NotFound, got {:?}", other),
        }
    }

    #[test]
    fn read_missing_file_is_io() {
        match read_or_default("/definitely/does/not/exist/zzz.txt") {
            Err(MyError::Io(inner)) => {
                // The wrapped error is the real io::Error — we can ask it questions.
                assert_eq!(inner.kind(), std::io::ErrorKind::NotFound);
            }
            other => panic!("expected Io(NotFound), got {:?}", other),
        }
    }
}

fn main() {}
