// =============================================================================
//  res1 — Result<T, E> basics
// =============================================================================
//
// `Option<T>` says "value or nothing". `Result<T, E>` says "value or a
// REASON it failed":
//
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
//
// Like `Option`, it's in the prelude. The `E` parameter lets you carry
// a description of the failure — a string, a custom enum, an error
// type from another crate. The compiler enforces that you handle BOTH
// cases before you can use the inner value.
//
// CONSTRUCTING
// ────────────
//     fn parse_age(s: &str) -> Result<u32, String> {
//         match s.parse::<u32>() {
//             Ok(n)  => Ok(n),
//             Err(_) => Err(format!("not a number: {s}")),
//         }
//     }
//
// QUICK QUERIES
// ─────────────
//     r.is_ok()    // true if Ok(_)
//     r.is_err()   // true if Err(_)
//
// EXTRACTING — the loud ways
// ──────────────────────────
//     r.unwrap()         // returns T from Ok(T); PANICS on Err.
//     r.expect("msg")    // returns T from Ok(T); PANICS with msg on Err.
//
// Same shape as `Option`, but the panic message includes the `E` value
// (via `Debug`), so the failure mode is much more informative:
//
//     // PANIC: called `Result::unwrap()` on an `Err` value: "not a number: foo"
//     parse_age("foo").unwrap();
//
// RESULT vs PANIC — when to use which
// ───────────────────────────────────
//   Return `Result<T, E>` when the failure is EXPECTED:
//     - parsing user input
//     - opening a file
//     - network IO
//     - any operation where "this can fail" is part of the contract
//
//   `panic!` when the failure means the program is in an IMPOSSIBLE state:
//     - an invariant the code itself controls is violated
//     - "this index is in bounds because I just checked it"
//     - a bug, not an environmental problem
//
// The rough test: if a sane caller might want to RECOVER, use Result.
// If recovery is impossible or meaningless, panic.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `parse_u32(s)`            -> Result<u32, String>
//                                   On parse error, return Err with the message
//                                   `format!("not a number: {}", s)`.
//   - `succeeded(r)`            -> bool                (use is_ok)
//   - `failed(r)`               -> bool                (use is_err)
//   - `force_ok(r)`             -> u32                 (use unwrap on Result<u32,String>)
//   - `force_ok_msg(r)`         -> u32                 (use expect("must be Ok"))
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn parse_u32(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n)  => ???,
        Err(_) => ???,
    }
}

fn succeeded(r: &Result<u32, String>) -> bool {
    r.???()
}

fn failed(r: &Result<u32, String>) -> bool {
    r.???()
}

fn force_ok(r: Result<u32, String>) -> u32 {
    r.???()
}

fn force_ok_msg(r: Result<u32, String>) -> u32 {
    r.???("must be Ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn parse_ok()  { assert_eq!(parse_u32("42"),   Ok(42)); }
    #[test] fn parse_err() { assert_eq!(parse_u32("nope"), Err("not a number: nope".to_string())); }

    #[test] fn s_ok()  { assert!( succeeded(&Ok(1))); }
    #[test] fn s_err() { assert!(!succeeded(&Err("x".into()))); }

    #[test] fn f_ok()  { assert!(!failed(&Ok(1))); }
    #[test] fn f_err() { assert!( failed(&Err("x".into()))); }

    #[test] fn force_ok_works() { assert_eq!(force_ok(Ok(7)), 7); }

    #[test]
    #[should_panic]
    fn force_ok_err_panics() { let _ = force_ok(Err("boom".into())); }

    #[test] fn force_msg_ok() { assert_eq!(force_ok_msg(Ok(9)), 9); }

    #[test]
    #[should_panic(expected = "must be Ok")]
    fn force_msg_err_panics() { let _ = force_ok_msg(Err("boom".into())); }
}

fn main() {}
