// SOLUTION — err1_custom_enum

#[derive(Debug)]
enum MyError {
    NotFound,
    Invalid(String),
    Io(std::io::Error),
}

fn lookup(key: &str) -> Result<&'static str, MyError> {
    match key {
        "alpha" => Ok("first"),
        "beta"  => Ok("second"),
        ""      => Err(MyError::Invalid("empty key".to_string())),
        _       => Err(MyError::NotFound),
    }
}

fn read_or_default(path: &str) -> Result<String, MyError> {
    match std::fs::read_to_string(path) {
        Ok(s)  => Ok(s),
        Err(e) => Err(MyError::Io(e)),
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   Three variants, three failure SHAPES. `NotFound` is a unit variant
//   because no extra data is meaningful — the caller distinguishes
//   "missing" by the tag alone and acts accordingly. `Invalid(String)`
//   carries an owned message so the function can describe WHY a value
//   was rejected (empty, malformed, out of range...). `Io(io::Error)`
//   wraps the foreign error verbatim — callers can pattern-match on
//   `inner.kind()`, log it, or re-raise it. The original error is
//   preserved, not stringified.
//
//   `#[derive(Debug)]` shows up on EVERY error type in real codebases
//   for a reason: `Result::unwrap`, `assert_eq!`, `?`-in-tests, and
//   `panic!("{:?}", e)` all use Debug to print. Without it, a failing
//   test gives you a compile error instead of a useful message.
//
//   We MANUALLY construct `MyError::Io(e)` from `Err(e)` here on
//   purpose — it makes the wrapping visible. Once we add an
//   `impl From<io::Error> for MyError` (in err3), the same code
//   collapses to `Ok(std::fs::read_to_string(path)?)`. You'll see
//   the difference clearly when you do the conversion yourself.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `enum MyError { Msg(String) }`:
//   The "stringly typed" trap. The tests would pass, but the caller
//   can no longer tell "missing key" from "invalid key" without
//   parsing the message back out. Structured variants are the
//   point of having an enum at all.
//
//   `Box<dyn std::error::Error>` everywhere:
//   Skips the design work of naming your failures. We'll cover the
//   right time to use that in err5 — for libraries where you DO want
//   to name your errors, a concrete enum is correct.
//
//   `enum MyError { Io(String), ... }`:
//   Stringifies the io error on the way in (`format!("{}", io_err)`),
//   losing `kind()` and the chain. If you find yourself doing this,
//   you're throwing away information that callers will eventually want.
//
// SUBTLETY:
//   The runner intentionally probes `/definitely/does/not/exist/...`
//   to trigger a real `io::Error`. If your environment somehow has
//   that path, swap to any other guaranteed-missing path — the test
//   only cares that the io error is faithfully wrapped.
//
//   In a real crate you'd add `#[non_exhaustive]` to public error
//   enums so adding a variant later is not a breaking change for
//   downstream `match` users. We'll skip that here for clarity.
