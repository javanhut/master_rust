// =============================================================================
//  str6 — parsing strings into numbers (and other types)
// =============================================================================
//
// Going text -> number is one of the most common operations a program does.
// In Rust the entry point is the `str::parse` method:
//
//     fn parse<F: FromStr>(&self) -> Result<F, F::Err>
//
// Two things to notice:
//
//   1. It is GENERIC over the target type `F`. The compiler cannot guess
//      what you want, so you must annotate, either at the binding site or
//      with the "turbofish":
//
//          let n: i32 = "42".parse().unwrap();
//          let n     = "42".parse::<i32>().unwrap();
//
//   2. It returns `Result<F, F::Err>` — parsing CAN fail (bad characters,
//      out of range, trailing whitespace...). Rust forces you to confront
//      that. For now we'll just `.unwrap()` on success and let the test
//      itself check the failure case.
//
// `FromStr` is implemented for every primitive (i8..i128, u..., f32, f64,
// bool, IpAddr, etc.) and you can implement it for your own types.
//
// ZERO PADDING / SIGNS / WHITESPACE
//
//     "  7".parse::<i32>()   // Err — leading whitespace not tolerated
//     "+7".parse::<i32>()    // Ok(7)
//     "-7".parse::<i32>()    // Ok(-7)
//     "07".parse::<i32>()    // Ok(7) — leading zeros are fine
//     "7.0".parse::<i32>()   // Err — i32 doesn't accept fractional part
//     "7.0".parse::<f64>()   // Ok(7.0)
//
// Always `.trim()` user input first if you can't trust it.
//
// HANDLING ERRORS
//
//   Proper error handling — propagating with `?`, custom error types, the
//   `thiserror` crate — is the topic of CHAPTER 8. For this chapter we
//   stick to `.unwrap()` on success paths and explicitly inspect the
//   `Result` when the test wants to confirm a failure case. We are NOT
//   writing production code yet; we are isolating the string-to-number
//   conversion mechanic.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `parse_i32`:    parse a `&str` into an `i32`, unwrap on success.
//   - `parse_f64`:    parse a `&str` into an `f64`, unwrap on success.
//   - `parse_or_zero`: parse to i64; on parse failure return 0 instead of
//                      panicking. (Preview of the next chapters: this is
//                      `.unwrap_or(0)` on a Result.)
//   - `sum_csv_ints`: given "1,2,3", trim each piece, parse to i32, sum.
//                      You may .unwrap() each parse; tests pass clean input.

// I AM NOT DONE

fn parse_i32(s: &str) -> i32 {
    s.parse::<???>().???()
}

fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap()
}

fn parse_or_zero(s: &str) -> i64 {
    s.parse::<i64>().???(0)
}

fn sum_csv_ints(s: &str) -> i32 {
    let mut total = 0;
    for piece in s.split(',') {
        total += piece.???().parse::<i32>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn pi_basic()     { assert_eq!(parse_i32("42"),    42); }
    #[test] fn pi_neg()       { assert_eq!(parse_i32("-7"),    -7); }

    #[test] fn pf_basic()     { assert_eq!(parse_f64("2.5"),   2.5); }
    #[test] fn pf_int_form()  { assert_eq!(parse_f64("3"),     3.0); }

    #[test] fn poz_ok()       { assert_eq!(parse_or_zero("99"),       99); }
    #[test] fn poz_garbage()  { assert_eq!(parse_or_zero("nope"),     0); }
    #[test] fn poz_empty()    { assert_eq!(parse_or_zero(""),         0); }

    #[test] fn sum_basic()    { assert_eq!(sum_csv_ints("1,2,3"),     6); }
    #[test] fn sum_padded()   { assert_eq!(sum_csv_ints(" 10 , 20 "), 30); }
    #[test] fn sum_one()      { assert_eq!(sum_csv_ints("42"),        42); }
}

fn main() {}
