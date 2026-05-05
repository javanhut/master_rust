// =============================================================================
//  intro5 — shadowing
// =============================================================================
//
// SHADOWING is when you use `let` AGAIN with a name that is already bound.
// The new binding hides — "shadows" — the old one for the rest of the scope.
//
//     let x = 5;
//     let x = x + 1;        // x is now 6
//     let x = x.to_string();// x is now a String — TYPE CHANGED
//
// Shadowing vs `mut` — when to use which?
//
//   `mut`        — same name, same type, value changes over time.
//                  Useful when the value is genuinely mutating in place.
//
//   shadowing    — same name, possibly NEW TYPE, conceptually a fresh
//                  variable. Useful when you want to refine / convert a
//                  value, especially right after parsing or validating.
//
// A classic example — read a string and convert it to a number, reusing
// the same name:
//
//     let input = "42";              // input: &str
//     let input: i32 = input.parse().unwrap();  // input: i32 (new binding!)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// `parse_and_double` takes the textual representation of a number, parses it,
// then returns it doubled. Use SHADOWING (not `mut`) to convert from `&str`
// to `i64` while keeping the same variable name `n`.
//
// (`.parse()` is a method on strings; `.unwrap()` says "panic if it failed".
// We'll learn proper error handling later.)

// I AM NOT DONE

fn parse_and_double(n: &str) -> i64 {
    // Step 1: shadow `n` with the parsed integer (annotate the type!).
    let n: i64 = n.???;

    // Step 2: return n * 2 — note: NO semicolon, this is the return value.
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn doubles_seven()      { assert_eq!(parse_and_double("7"), 14); }
    #[test] fn doubles_zero()       { assert_eq!(parse_and_double("0"), 0); }
    #[test] fn doubles_negative()   { assert_eq!(parse_and_double("-12"), -24); }
    #[test] fn doubles_big()        { assert_eq!(parse_and_double("1000000"), 2_000_000); }
}

fn main() {}
