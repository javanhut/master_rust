// SOLUTION — errors_quiz (CLI command parser, full err1–err4 toolkit)

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
            MyError::UnknownVerb(v)   => write!(f, "unknown verb: {v}"),
            MyError::MissingArg(role) => write!(f, "missing argument: {role}"),
            MyError::BadCount(inner)  => write!(f, "bad count: {inner}"),
            MyError::Trailing(extra)  => write!(f, "trailing input: {extra}"),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::BadCount(e) => Some(e),
            _                    => None,
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::BadCount(e)
    }
}

fn parse_user_command(line: &str) -> Result<Command, MyError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(MyError::Empty);
    }

    let mut it = line.split_whitespace();
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
            let n: u32 = raw.parse::<u32>()?;
            Command::Count(n)
        }
        other => return Err(MyError::UnknownVerb(other.to_string())),
    };

    let leftover: Vec<&str> = it.collect();
    if !leftover.is_empty() {
        return Err(MyError::Trailing(leftover.join(" ")));
    }

    Ok(cmd)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The error enum is the design centre. Five variants, each one a
//   FAILURE CATEGORY a real caller might handle differently:
//
//     - Empty               → "show usage"
//     - UnknownVerb(String) → "did you mean ...?"
//     - MissingArg(role)    → "expected: <role>"
//     - BadCount(parse)     → "count must be a non-negative integer"
//     - Trailing(extra)     → "did you mean to quote the value?"
//
//   Carrying a `&'static str` in `MissingArg` (rather than a String)
//   skips an allocation on the error path — there are only three
//   possible roles and they're all literals. `UnknownVerb` and
//   `Trailing` carry owned `String`s because their content is
//   user-controlled. `BadCount` wraps the `ParseIntError` verbatim
//   so the source chain works.
//
//   Display strings follow the chapter convention: short, lowercase,
//   no trailing punctuation, parameterised on the variant payload.
//   `BadCount`'s message embeds the inner error's Display so a CLI
//   prints "bad count: invalid digit found in string" — which is
//   exactly the chain-flattening behaviour you want for short-form
//   error messages.
//
//   Overriding `source()` for `BadCount` exposes the underlying
//   `ParseIntError` to logging / pretty-printing layers. The other
//   variants originate the failure HERE, so they return None.
//
//   `From<ParseIntError>` is the single line that lets `raw.parse::
//   <u32>()?` work in this function. Without it, the `?` desugaring's
//   `From::from` call wouldn't type-check.
//
//   The function body itself uses `.ok_or(...)?` (Option → Result)
//   for missing tokens and `?` (Result → Result) for the parse step.
//   No nested matches, no early-return pyramids. The unwrap on
//   `it.next()` for the verb is sound: we already checked
//   `line.is_empty()` AFTER trimming, so split_whitespace yields at
//   least one token.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `.unwrap_or_else(|| panic!(...))` instead of `.ok_or(...)?`:
//   panics aren't recoverable. User input failing is expected.
//
//   Using anyhow-style `Box<dyn Error>` here:
//   This is a LIBRARY function (the parser) — callers might want to
//   match on `UnknownVerb` to suggest corrections, or on
//   `MissingArg("key")` to point at the right argument. Throwing
//   away the variant identity costs you that. err5 covers when
//   boxing IS appropriate (in the binary that USES this parser).
//
//   Stuffing a `format!` into a single `Msg(String)` variant:
//   The tests would still pass with the right strings, but real
//   callers couldn't react to specific failures without parsing
//   the message back out. Same anti-pattern we've avoided all chapter.
//
//   Manual `match` instead of `?`:
//       let key = match it.next() {
//           Some(k) => k,
//           None    => return Err(MyError::MissingArg("key")),
//       };
//   Five lines per token for a function that handles four verbs is
//   how parsers become unreadable. `.ok_or(...)?` is the same logic
//   in one line.
//
// SUBTLETY:
//   `split_whitespace` collapses runs of any whitespace AND drops
//   empty leading/trailing pieces — that's what makes
//   `   get   foo   ` parse cleanly. If you used `split(' ')` you'd
//   get spurious empty tokens that the leftover check would flag as
//   `Trailing("")`.
//
//   The order of checks matters for `set onlykey extra`: the
//   `value` arg is taken FIRST as `extra`, leaving no leftovers, so
//   it parses as `Set { key: "onlykey", value: "extra" }`. That's
//   intentional — we count ONLY positional arguments. To require
//   exactly two and reject a third, the leftover check at the end
//   handles it.
//
//   In a real production parser you'd add `#[non_exhaustive]` to
//   `MyError` so adding a sixth variant later isn't a breaking
//   change. We've kept it off here for clarity, matching the rest
//   of the chapter.
//
//   To migrate to crates: `cargo add thiserror anyhow`. The five
//   impls collapse to a single `#[derive(thiserror::Error, Debug)]`
//   with `#[error("...")]` attributes. Function bodies don't change
//   at all — the `?` operator was already doing all the work.
