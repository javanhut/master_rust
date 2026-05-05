// =============================================================================
//  intro4 — mutability with `let mut`
// =============================================================================
//
// Sometimes you really do want to change a value over time — accumulating a
// sum, mutating a counter, etc. Add `mut` after `let`:
//
//     let mut score = 0;
//     score += 1;        // ✅ now allowed
//     score = 42;        // ✅ allowed too — same TYPE required
//
// Things to internalise:
//
// 1.  `mut` is part of the BINDING, not the type. You can rebind a `mut`
//     variable but you cannot change its type with `=`.
//
// 2.  The new value must have the SAME type as the old one.
//
//         let mut x = 0;
//         x = "hi";      // ❌ expected integer, found &str
//
// 3.  Use mutability sparingly. If you find yourself reaching for `mut` a
//     lot, ask whether you can compute the new value in one shot instead
//     (Rust often makes that nicer than you'd expect).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `sum_first_n` so it returns 1 + 2 + ... + n using a mutable
// accumulator and a `for` loop.  (We'll meet `iterator::sum()` later, which
// is the idiomatic version — but right now the goal is to internalise `mut`.)
//
// Replace ALL the `???` in the function body. Do NOT touch the tests.

// I AM NOT DONE

fn sum_first_n(n: u32) -> u32 {
    let ??? total = 0u32;
    for i in 1..=n {
        total ??? i;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sum_zero() { assert_eq!(sum_first_n(0), 0); }
    #[test] fn sum_one()  { assert_eq!(sum_first_n(1), 1); }
    #[test] fn sum_ten()  { assert_eq!(sum_first_n(10), 55); }
    #[test] fn sum_hund() { assert_eq!(sum_first_n(100), 5050); }
}

fn main() {}
