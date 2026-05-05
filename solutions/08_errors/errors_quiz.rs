// SOLUTION — errors_quiz (tiny config parser)

use std::num::ParseIntError;

#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
}

#[derive(Debug)]
enum ConfigError {
    MissingEquals(String),
    UnknownKey(String),
    BadPort(ParseIntError),
    Missing(&'static str),
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadPort(e)
    }
}

fn parse_config(s: &str) -> Result<Config, ConfigError> {
    let mut port: Option<u16> = None;
    let mut host: Option<String> = None;

    for line in s.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::MissingEquals(line.to_string()))?;

        match key {
            "port" => {
                let n: u16 = value.parse::<u16>()?;
                port = Some(n);
            }
            "host" => {
                host = Some(value.to_string());
            }
            other => return Err(ConfigError::UnknownKey(other.to_string())),
        }
    }

    let port = port.ok_or(ConfigError::Missing("port"))?;
    let host = host.ok_or(ConfigError::Missing("host"))?;
    Ok(Config { port, host })
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The error enum is the design centre. FOUR variants, one per
//   distinct failure mode the caller might want to react to:
//   `MissingEquals` and `UnknownKey` both carry an owned `String`
//   (the offending text), `BadPort` wraps the underlying parse
//   error so it can be inspected, and `Missing` carries a `&'static
//   str` because the only two possible values are the literals
//   "port" and "host" — no allocation needed.
//
//   `impl From<ParseIntError> for ConfigError` is the single line
//   that lets `value.parse::<u16>()?` work inside a function that
//   returns `Result<_, ConfigError>`. Without it, the desugaring's
//   `From::from` call would fail to type-check.
//
//   `.ok_or_else(|| ...)` and `.ok_or(...)` are the converse of
//   `.ok()` from res3 — they convert an `Option` into a `Result`,
//   supplying the error to use when the Option was `None`.
//   `_else` is for non-trivial constructors (here, allocating a
//   `String` from `line`); the bare form `.ok_or(value)` is fine
//   for cheap or already-constructed values.
//
//   Tracking required keys as `Option<T>` while scanning, then
//   `ok_or`-ing them after the loop, separates "did we see this
//   key at all?" from "was the value valid?". That separation is
//   what gives us crisp, actionable error variants.
//
//   The whole function body uses `?` four times. No nested match,
//   no pyramid of doom. The happy path is a sequence of normal
//   statements; failures bubble out via `?` with the right
//   variant attached.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Stringly-typed errors:
//       enum ConfigError { Msg(String) }
//   The tests would still pass with `format!("missing port")`,
//   but callers couldn't distinguish "wrong format" from "missing
//   key" without parsing the message back. Structured variants
//   are the whole point.
//
//   Boxed dynamic errors:
//       fn parse_config(s: &str) -> Result<Config, Box<dyn std::error::Error>>
//   Saves the `From` impl but loses the variant tag. We're staying
//   concrete — chapter 15 covers the dynamic toolkit.
//
//   `eprintln!` and `panic!` instead of returning errors:
//   Calling code can't recover. Reserve panics for impossible
//   states; user input failing is expected and recoverable.
//
//   Manual `match` for every `?`:
//       let (key, value) = match line.split_once('=') {
//           Some(pair) => pair,
//           None => return Err(ConfigError::MissingEquals(line.to_string())),
//       };
//   That's exactly what `.ok_or_else(...)?` desugars to. Use the
//   combinator form unless the manual one genuinely reads better
//   (rare).
//
// SUBTLETY:
//   `line.split_once('=')` only splits on the FIRST `=`. So
//   `host=user=foo` becomes ("host", "user=foo"), which is what
//   you want — values are allowed to contain `=`. If you used
//   `line.split('=').collect::<Vec<_>>()` you'd have to handle
//   the multi-piece case yourself.
//
//   In `missing_both_when_empty`, the empty-string input means
//   `s.lines()` yields zero lines. After the loop, both `port`
//   and `host` are still `None`. Because we check `port` first,
//   the function returns `Missing("port")`. If you wanted to
//   report ALL missing keys you'd accumulate into a `Vec` and
//   return a single `MissingMany(Vec<&'static str>)` variant —
//   a fine extension exercise.
