// =============================================================================
//  errors_quiz — capstone: a tiny CLI command parser
// =============================================================================
//
// Time to put chapter 15 together. You're going to write a parser for
// a tiny REPL-style command language:
//
//     get <key>
//     set <key> <value>
//     delete <key>
//     count <number>
//
// The function `parse_user_command(line: &str) -> Result<Command, MyError>`
// takes one input line and returns either the structured command or
// an error variant explaining what went wrong.
//
// You will:
//
//   - Define a `Command` enum with FOUR variants (one per verb).
//   - Define a `MyError` enum with FIVE variants covering the failure
//     modes (see below).
//   - Implement `Display`, `std::error::Error` (with a real `source()`),
//     and `From<ParseIntError>` for `MyError` — by hand, like err4.
//   - Make the function body a clean tower of `?` calls.
//
// THE FIVE FAILURE MODES
// ──────────────────────
//   - Empty                 — input was empty or whitespace-only.
//                             No payload — the kind alone says it.
//   - UnknownVerb(String)   — the first word wasn't `get`/`set`/
//                             `delete`/`count`. Carries the offending
//                             verb.
//   - MissingArg(&'static str)
//                           — a required argument was absent. The
//                             &'static str is the role: "key", "value",
//                             "count".
//   - BadCount(ParseIntError)
//                           — `count <number>` got a non-integer.
//                             Wraps the underlying parse error so
//                             callers can inspect kind() and
//                             pretty-printers can chain source().
//   - Trailing(String)      — extra unexpected words after the
//                             expected arguments. Carries the leftover.
//
// THE `Command` ENUM
// ──────────────────
//     enum Command {
//         Get(String),                 // get <key>
//         Set { key: String, value: String },
//         Delete(String),              // delete <key>
//         Count(u32),                  // count <number>
//     }
//
//   - Tuple variant for one-arg commands; named-field for two-arg.
//   - The `Set` value is the SECOND token only — no spaces allowed.
//
// PARSING SHAPE
// ─────────────
//   1. Trim the line. If empty → MyError::Empty.
//   2. `split_whitespace()` to get an iterator of tokens.
//   3. First token is the verb. Match on it:
//        "get"    — exactly one arg (the key)
//        "set"    — exactly two args (key, value)
//        "delete" — exactly one arg (the key)
//        "count"  — exactly one arg, parse as u32 with `?`
//        other    — UnknownVerb(other.to_string())
//   4. After taking required tokens, ensure no leftovers — otherwise
//      Trailing(joined_remainder).
//
// Use the iterator `.next()` to pull tokens. After the expected ones,
// `.collect::<Vec<&str>>().join(" ")` joins any leftovers.
//
// SOURCE CHAIN (REQUIRED)
// ───────────────────────
// Override `Error::source()` so `BadCount` exposes the wrapped
// `ParseIntError`. A test verifies this.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Fill in everything marked with `???`. You're applying err1–err4 in
// one shot: enum, Display, Error+source, From, ? throughout the parser.

// I AM NOT DONE

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Command {
    Get(String),
    Set { key: String, value: String },
    Delete(String),
    Count(u32),
}

#[derive(Debug)]
enum MyError {
    Empty,
    UnknownVerb(String),
    MissingArg(&'static str),
    BadCount(ParseIntError),
    Trailing(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Empty            => write!(f, "empty input"),
            MyError::UnknownVerb(v)   => write!(f, "unknown verb: {???}"),
            MyError::MissingArg(role) => write!(f, "missing argument: {???}"),
            MyError::BadCount(inner)  => write!(f, "bad count: {???}"),
            MyError::Trailing(extra)  => write!(f, "trailing input: {???}"),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::BadCount(e) => Some(???),
            _                    => None,
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::???(e)
    }
}

fn parse_user_command(line: &str) -> Result<Command, MyError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(MyError::???);
    }

    let mut it = line.split_whitespace();
    // The first split_whitespace token always exists when the trimmed line is non-empty.
    let verb = it.next().unwrap();

    let cmd = match verb {
        "get" => {
            let key = it.next().ok_or(MyError::MissingArg("key"))?;
            Command::Get(key.to_string())
        }
        "set" => {
            let key   = it.next().ok_or(MyError::MissingArg("key"))?;
            let value = it.next().ok_or(MyError::MissingArg("value"))?;
            Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }
        }
        "delete" => {
            let key = it.next().ok_or(MyError::MissingArg("key"))?;
            Command::Delete(key.to_string())
        }
        "count" => {
            let raw = it.next().ok_or(MyError::MissingArg("count"))?;
            // ? here uses From<ParseIntError> for MyError — variant BadCount.
            let n: u32 = raw.parse::<u32>()???;
            Command::Count(n)
        }
        other => return Err(MyError::???(other.to_string())),
    };

    // No more tokens allowed.
    let leftover: Vec<&str> = it.collect();
    if !leftover.is_empty() {
        return Err(MyError::Trailing(leftover.join(" ")));
    }

    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn ok_get() {
        assert_eq!(parse_user_command("get foo").unwrap(), Command::Get("foo".into()));
    }

    #[test]
    fn ok_set() {
        let c = parse_user_command("set name alice").unwrap();
        assert_eq!(c, Command::Set { key: "name".into(), value: "alice".into() });
    }

    #[test]
    fn ok_delete() {
        assert_eq!(parse_user_command("delete x").unwrap(), Command::Delete("x".into()));
    }

    #[test]
    fn ok_count() {
        assert_eq!(parse_user_command("count 42").unwrap(), Command::Count(42));
    }

    #[test]
    fn ok_extra_whitespace_trimmed() {
        // Surrounding whitespace and runs between tokens collapse.
        assert_eq!(parse_user_command("   get   foo   ").unwrap(),
                   Command::Get("foo".into()));
    }

    #[test]
    fn empty_is_empty() {
        match parse_user_command("") {
            Err(MyError::Empty) => {}
            other => panic!("expected Empty, got {:?}", other),
        }
        match parse_user_command("    ") {
            Err(MyError::Empty) => {}
            other => panic!("expected Empty for whitespace-only, got {:?}", other),
        }
    }

    #[test]
    fn unknown_verb() {
        match parse_user_command("frobnicate x") {
            Err(MyError::UnknownVerb(v)) => assert_eq!(v, "frobnicate"),
            other => panic!("expected UnknownVerb, got {:?}", other),
        }
    }

    #[test]
    fn missing_key_for_get() {
        match parse_user_command("get") {
            Err(MyError::MissingArg("key")) => {}
            other => panic!("expected MissingArg(\"key\"), got {:?}", other),
        }
    }

    #[test]
    fn missing_value_for_set() {
        match parse_user_command("set onlykey") {
            Err(MyError::MissingArg("value")) => {}
            other => panic!("expected MissingArg(\"value\"), got {:?}", other),
        }
    }

    #[test]
    fn missing_count_arg() {
        match parse_user_command("count") {
            Err(MyError::MissingArg("count")) => {}
            other => panic!("expected MissingArg(\"count\"), got {:?}", other),
        }
    }

    #[test]
    fn bad_count_lifts_via_question_mark() {
        match parse_user_command("count NaN") {
            Err(MyError::BadCount(_)) => {}
            other => panic!("expected BadCount, got {:?}", other),
        }
    }

    #[test]
    fn trailing_input_caught() {
        match parse_user_command("get foo bar") {
            Err(MyError::Trailing(extra)) => assert_eq!(extra, "bar"),
            other => panic!("expected Trailing, got {:?}", other),
        }
        match parse_user_command("set k v extra1 extra2") {
            Err(MyError::Trailing(extra)) => assert_eq!(extra, "extra1 extra2"),
            other => panic!("expected Trailing, got {:?}", other),
        }
    }

    #[test]
    fn display_messages() {
        assert_eq!(format!("{}", MyError::Empty), "empty input");
        assert_eq!(
            format!("{}", MyError::UnknownVerb("frob".into())),
            "unknown verb: frob"
        );
        assert_eq!(
            format!("{}", MyError::MissingArg("key")),
            "missing argument: key"
        );
        assert_eq!(
            format!("{}", MyError::Trailing("rest".into())),
            "trailing input: rest"
        );
    }

    #[test]
    fn source_chains_for_bad_count() {
        let e = parse_user_command("count NaN").unwrap_err();
        assert!(e.source().is_some(), "BadCount should expose its source");
    }

    #[test]
    fn source_is_none_for_other_variants() {
        assert!(MyError::Empty.source().is_none());
        assert!(MyError::UnknownVerb("x".into()).source().is_none());
        assert!(MyError::MissingArg("key").source().is_none());
        assert!(MyError::Trailing("z".into()).source().is_none());
    }
}

fn main() {}
