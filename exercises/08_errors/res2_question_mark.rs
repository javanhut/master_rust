// =============================================================================
//  res2 — the `?` operator
// =============================================================================
//
// Manually matching every Result is exhausting. The `?` operator is
// Rust's syntactic sugar for "if Err, return it; otherwise extract the
// Ok and keep going":
//
//     let n: i32 = "42".parse::<i32>()?;   // n is i32, not Result<i32,_>
//
// EXACT DESUGARING
// ────────────────
// `expr?` expands to:
//
//     match expr {
//         Ok(v)  => v,
//         Err(e) => return Err(From::from(e)),
//     }
//
// Three things to internalise:
//
//   1. SUCCESS extracts the value: `?` returns the `T` from `Ok(T)` so
//      the surrounding expression sees a plain `T`.
//   2. FAILURE early-returns from the function. Not from a loop, not
//      from a closure — from the WHOLE function the `?` is in.
//   3. The error is `From::from`-converted on the way out, which lets
//      you bubble different child error types up into a parent error
//      type. We'll exploit this in `res4_from`.
//
// WHEN CAN YOU USE `?`
// ────────────────────
// The function containing `?` MUST itself return a `Result<_, E>` (or
// `Option<_>`, but we'll focus on Result here). The compiler enforces
// this:
//
//     fn try_thing() -> Result<i32, std::num::ParseIntError> {
//         let a: i32 = "10".parse()?;   // OK — function returns Result
//         let b: i32 = "20".parse()?;
//         Ok(a + b)
//     }
//
//     fn broken() -> i32 {
//         let a: i32 = "10".parse()?;   // ❌ ERROR: `?` in fn returning i32
//         a
//     }
//
// CHAINING — the real superpower
// ──────────────────────────────
// You can string several `?`-using calls together; the first failure
// short-circuits the whole expression:
//
//     fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
//         Ok(a.parse::<i32>()? + b.parse::<i32>()?)
//     }
//
// If `a.parse()` fails, we return that error. Otherwise we extract the
// number, parse `b`, return ITS error if it fails, otherwise add them
// and wrap in `Ok`. In one line.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `parse_i32(s)`  -> Result<i32, std::num::ParseIntError>
//                        Use `?` once. Body is a single trailing expression.
//
//   - `add_two(a, b)` -> Result<i32, std::num::ParseIntError>
//                        Use `?` twice. Return the SUM wrapped in Ok.
//
//   - `triple(s)`     -> Result<i32, std::num::ParseIntError>
//                        Parse with `?`, then return Ok(n * 3).
//                        Use a `let` binding so the desugaring is
//                        visible (no inline arithmetic on the parse).
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn parse_i32(s: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(s.parse::<i32>()???)
}

fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(a.parse::<i32>()??? + b.parse::<i32>()???)
}

fn triple(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse::<i32>()???;
    Ok(n * 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn p_ok()   { assert_eq!(parse_i32("42").unwrap(), 42); }
    #[test] fn p_err()  { assert!(parse_i32("nope").is_err()); }

    #[test] fn add_ok()    { assert_eq!(add_two("10", "20").unwrap(), 30); }
    #[test] fn add_err_a() { assert!(add_two("oops", "20").is_err()); }
    #[test] fn add_err_b() { assert!(add_two("10", "oops").is_err()); }

    #[test] fn tri_ok()  { assert_eq!(triple("4").unwrap(), 12); }
    #[test] fn tri_err() { assert!(triple("x").is_err()); }
}

fn main() {}
