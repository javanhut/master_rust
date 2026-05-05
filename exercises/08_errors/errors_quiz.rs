// =============================================================================
//  errors_quiz — capstone: a tiny config parser
// =============================================================================
//
// Time to put it all together. You're building a parser for a tiny
// config-file format:
//
//     port=8080
//     host=localhost
//
// Every line has the shape `key=value`, separated by exactly one `=`.
// Required keys are `port` (parses to u16) and `host` (a String).
//
// You'll define:
//
//   - A `Config` struct: `{ port: u16, host: String }`.
//   - A `ConfigError` enum with FOUR variants — one per failure mode.
//   - An `impl From<ParseIntError> for ConfigError` so `?` lifts the
//     parse error automatically.
//   - `parse_config(s: &str) -> Result<Config, ConfigError>` using `?`
//     throughout — no nested matches.
//
// THE FOUR FAILURE MODES — be precise about WHY a parse fails:
//
//   - MissingEquals(String)   — a line had no `=` separator.
//                                The String is the offending line.
//   - UnknownKey(String)      — the key wasn't `port` or `host`.
//                                The String is the unknown key.
//   - BadPort(ParseIntError)  — value for `port` didn't parse as u16.
//                                Wraps the underlying error.
//   - Missing(&'static str)   — a required key never appeared.
//                                The &'static str is the missing key
//                                ("port" or "host").
//
// HINTS — read these BEFORE you start
// ───────────────────────────────────
//   - Use `s.lines()` to iterate over lines. Empty input is allowed,
//     but missing required keys must produce `Missing(...)`.
//   - For each line, `line.split_once('=')` returns `Option<(&str, &str)>`.
//     Use `.ok_or_else(|| ConfigError::MissingEquals(line.to_string()))?`
//     to convert the Option into a Result and short-circuit on None.
//   - Track the parsed values as `Option<u16>` and `Option<String>` while
//     scanning. After the loop, `.ok_or(ConfigError::Missing("port"))?`
//     turns `None` into the right error.
//   - The `?` operator on `s.parse::<u16>()` will call `From::from` on
//     the `ParseIntError` — that's what your `impl From` is for.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Fill in `ConfigError`, the `From` impl, and the body of `parse_config`.
// Tests will pin down every edge case.

// I AM NOT DONE

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
        ???
    }
}

fn parse_config(s: &str) -> Result<Config, ConfigError> {
    let mut port: Option<u16> = None;
    let mut host: Option<String> = None;

    for line in s.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::MissingEquals(line.???()))?;

        match key {
            "port" => {
                let n: u16 = value.parse::<u16>()???;
                port = Some(n);
            }
            "host" => {
                host = Some(value.???());
            }
            other => return Err(ConfigError::???(other.to_string())),
        }
    }

    let port = port.???(ConfigError::Missing("port"))?;
    let host = host.???(ConfigError::Missing("host"))?;
    Ok(Config { port, host })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let cfg = parse_config("port=8080\nhost=localhost").unwrap();
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.host, "localhost");
    }

    #[test]
    fn ok_reordered() {
        let cfg = parse_config("host=example.com\nport=80").unwrap();
        assert_eq!(cfg.port, 80);
        assert_eq!(cfg.host, "example.com");
    }

    #[test]
    fn missing_equals() {
        match parse_config("portxx8080") {
            Err(ConfigError::MissingEquals(line)) => assert_eq!(line, "portxx8080"),
            other => panic!("expected MissingEquals, got {:?}", other),
        }
    }

    #[test]
    fn unknown_key() {
        match parse_config("color=red\nport=80\nhost=h") {
            Err(ConfigError::UnknownKey(k)) => assert_eq!(k, "color"),
            other => panic!("expected UnknownKey, got {:?}", other),
        }
    }

    #[test]
    fn bad_port() {
        match parse_config("port=NaN\nhost=h") {
            Err(ConfigError::BadPort(_)) => {} // good
            other => panic!("expected BadPort, got {:?}", other),
        }
    }

    #[test]
    fn missing_port() {
        match parse_config("host=h") {
            Err(ConfigError::Missing("port")) => {} // good
            other => panic!("expected Missing(\"port\"), got {:?}", other),
        }
    }

    #[test]
    fn missing_host() {
        match parse_config("port=80") {
            Err(ConfigError::Missing("host")) => {} // good
            other => panic!("expected Missing(\"host\"), got {:?}", other),
        }
    }

    #[test]
    fn missing_both_when_empty() {
        match parse_config("") {
            // Empty input -> port checked first -> Missing("port").
            Err(ConfigError::Missing("port")) => {}
            other => panic!("expected Missing(\"port\"), got {:?}", other),
        }
    }
}

fn main() {}
