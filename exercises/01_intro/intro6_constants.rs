// =============================================================================
//  intro6 — constants
// =============================================================================
//
// `const` is for values that are KNOWN AT COMPILE TIME and never change.
//
//     const MAX_USERS: u32 = 10_000;
//
// Differences from `let`:
//
//   1. NAME       — convention is SCREAMING_SNAKE_CASE (the compiler will
//                   warn otherwise).
//   2. TYPE       — REQUIRED. The compiler will not infer the type of a
//                   const, because constants are sometimes referenced before
//                   they are assigned.
//   3. SCOPE      — constants can live at module level (outside any fn).
//                   `let` cannot.
//   4. EVALUATION — the right-hand side must be a "const expression": no
//                   I/O, no allocation, no calling non-`const fn` functions.
//                   `60 * 60` works; `vec![1,2,3]` does not.
//   5. INLINING   — the compiler may inline `const` values directly into the
//                   places that use them. There is no single memory location.
//
// `static` is similar but represents an actual memory location, lives for
// the whole program, and CAN (carefully) be mutable. We'll skip `static` for
// now — `const` is what you want 95% of the time.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define a const SECONDS_PER_DAY of type u32 with the correct value.
//   - Implement `days_to_seconds`. (Multiplication should not overflow for
//     the test values — `u32` is fine here.)

// I AM NOT DONE

const SECONDS_PER_HOUR: u32 = 60 * 60;

// TODO: define SECONDS_PER_DAY in terms of SECONDS_PER_HOUR.
const ???

fn days_to_seconds(days: u32) -> u32 {
    days * ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn one_day()    { assert_eq!(days_to_seconds(1),  86_400); }
    #[test] fn one_week()   { assert_eq!(days_to_seconds(7), 604_800); }
    #[test] fn zero_days()  { assert_eq!(days_to_seconds(0), 0); }
}

fn main() {}
