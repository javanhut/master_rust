// =============================================================================
//  res3 — Result combinators
// =============================================================================
//
// Just like `Option`, `Result` has a family of combinators that let you
// transform the value, transform the error, choose a fallback, and
// switch between Result and Option without ever writing an explicit
// match.
//
// TRANSFORMING — keep the Result shape
// ────────────────────────────────────
//   .map(f)         — Ok(x) -> Ok(f(x));   Err(e) -> Err(e)
//   .map_err(f)     — Ok(x) -> Ok(x);      Err(e) -> Err(f(e))
//   .and_then(f)    — Ok(x) -> f(x);       Err(e) -> Err(e)
//                     `f` must return Result<U, E>. Same idea as
//                     Option::and_then — "chain another fallible step".
//   .or_else(f)     — Ok(x) -> Ok(x);      Err(e) -> f(e)
//                     `f` must return Result<T, F>. Use this to
//                     RECOVER from an error by trying something else.
//
// CONVERTING TO Option
// ────────────────────
//   .ok()   — Result<T,E> -> Option<T>:  Ok(x) -> Some(x); Err(_) -> None.
//             Use this when you want to discard the error and treat
//             missing-or-broken as the same thing.
//   .err()  — Result<T,E> -> Option<E>:  Err(e) -> Some(e); Ok(_) -> None.
//             Useful when you only care WHY it failed.
//
//     "42".parse::<i32>().ok()   // Some(42)
//     "x" .parse::<i32>().ok()   // None
//
// EXAMPLE — translating an error
// ──────────────────────────────
//   Suppose `parse::<i32>()` gives you a `ParseIntError`, but your API
//   wants a `String` describing the failure:
//
//       fn parse_or_msg(s: &str) -> Result<i32, String> {
//           s.parse::<i32>().map_err(|e| format!("bad input: {e}"))
//       }
//
//   `map_err` only fires on the Err arm; success values pass through
//   untouched. Pair it with `?` and you can fluently translate errors
//   at function boundaries.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `double(r)`        -> Result<i32, String>
//                            On Ok(n), return Ok(n*2); pass Err through.
//                            (Use `.map`.)
//
//   - `tag_error(r)`     -> Result<i32, String>
//                            Convert the original `String` error to
//                            `format!("error: {}", e)`. Use `.map_err`.
//
//   - `to_option(r)`     -> Option<i32>      (use `.ok`)
//
//   - `to_err_option(r)` -> Option<String>   (use `.err`)
//
//   - `recover(r)`       -> Result<i32, String>
//                            On Err, try `.parse::<i32>()` of the error
//                            string and wrap that. Use `.or_else` and
//                            `map_err` inside.
//                            Hint: |e| e.parse::<i32>().map_err(|_| e)
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn double(r: Result<i32, String>) -> Result<i32, String> {
    r.???(|n| n * 2)
}

fn tag_error(r: Result<i32, String>) -> Result<i32, String> {
    r.???(|e| format!("error: {}", e))
}

fn to_option(r: Result<i32, String>) -> Option<i32> {
    r.???()
}

fn to_err_option(r: Result<i32, String>) -> Option<String> {
    r.???()
}

fn recover(r: Result<i32, String>) -> Result<i32, String> {
    r.???(|e| e.parse::<i32>().???(|_| e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn d_ok()  { assert_eq!(double(Ok(5)),               Ok(10)); }
    #[test] fn d_err() { assert_eq!(double(Err("nope".into())),  Err("nope".to_string())); }

    #[test] fn t_ok()  { assert_eq!(tag_error(Ok(1)),               Ok(1)); }
    #[test] fn t_err() { assert_eq!(tag_error(Err("boom".into())),  Err("error: boom".to_string())); }

    #[test] fn opt_ok()  { assert_eq!(to_option(Ok(7)),              Some(7)); }
    #[test] fn opt_err() { assert_eq!(to_option(Err("x".into())),    None); }

    #[test] fn err_ok()  { assert_eq!(to_err_option(Ok(7)),           None); }
    #[test] fn err_err() { assert_eq!(to_err_option(Err("x".into())), Some("x".to_string())); }

    #[test] fn rec_ok()       { assert_eq!(recover(Ok(3)),             Ok(3)); }
    #[test] fn rec_recovered() { assert_eq!(recover(Err("42".into())), Ok(42)); }
    #[test] fn rec_failed()    { assert_eq!(recover(Err("xx".into())), Err("xx".to_string())); }
}

fn main() {}
