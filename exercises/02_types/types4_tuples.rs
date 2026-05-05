// =============================================================================
//  types4 — tuples
// =============================================================================
//
// A TUPLE groups a fixed number of values, each with its own type, into a
// single compound value. Parentheses, comma-separated:
//
//     let pair: (i32, &str) = (42, "hello");
//
// Two ways to access fields:
//
//   1. By INDEX with a `.` and a literal number:
//
//          pair.0   // 42
//          pair.1   // "hello"
//
//   2. By DESTRUCTURING with `let`:
//
//          let (n, s) = pair;
//          // n: i32, s: &str
//
// The TWO SPECIAL TUPLES you'll see all the time:
//
//     ()         — the "unit" tuple, has exactly one value `()`.
//                  It's what functions return when they don't return anything.
//     (T,)       — a one-element tuple. The trailing comma is REQUIRED, else
//                  Rust thinks you wrote a parenthesized expression of type T.
//
// You can return a tuple to "return multiple values":
//
//     fn min_max(xs: &[i32]) -> (i32, i32) { ... }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `swap` returns the same pair with elements flipped.
//   - `min_max` returns (smallest, largest) of two values.

// I AM NOT DONE

fn swap(p: (i32, &str)) -> (&str, i32) {
    // Destructure or use field access — both work. Try destructuring.
    let (a, b) = p;
    (???, ???)
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    // Return the smaller value first, then the larger.
    // HINT: `i32` has methods `.min(other)` and `.max(other)`.
    (a.???(b), a.???(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn swap_works()      { assert_eq!(swap((1, "x")), ("x", 1)); }

    #[test] fn min_max_basic()   { assert_eq!(min_max(3, 7), (3, 7)); }
    #[test] fn min_max_reverse() { assert_eq!(min_max(9, 2), (2, 9)); }
    #[test] fn min_max_equal()   { assert_eq!(min_max(4, 4), (4, 4)); }
}

fn main() {}
