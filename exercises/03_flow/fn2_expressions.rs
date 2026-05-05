// =============================================================================
//  fn2 — expressions vs statements
// =============================================================================
//
// Rust is an EXPRESSION-ORIENTED language. Almost everything that produces
// a value can be used as a value:
//
//   - A BLOCK `{ ... }` is an expression. Its value is the last expression
//     inside it (no semicolon).
//
//         let x = {
//             let a = 2;
//             let b = 3;
//             a + b               // <- block evaluates to 5
//         };
//
//   - `if/else` is an expression — both arms must produce the same type:
//
//         let label = if score >= 60 { "pass" } else { "fail" };
//
//   - `match` (next chapter) is also an expression.
//
//   - `loop { ... break value; }` evaluates to whatever you `break` with.
//
// STATEMENTS DON'T HAVE A USABLE VALUE
//
//     let x = (let y = 5);   // ❌  `let` is a statement, not an expression.
//
// SEMICOLONS
//
// A semicolon turns an expression into a statement (its value becomes `()`).
// This is why `n * 2;` at the end of a function fails to typecheck when the
// return type is `i32`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Use an `if/else` AS AN EXPRESSION to give a binding called `tier`
//     the value `"gold"` if `points >= 100`, `"silver"` if >= 50, otherwise
//     `"bronze"`. (Yes, you can chain — `else if` is allowed.)

// I AM NOT DONE

fn tier_name(points: i32) -> &'static str {
    let tier = if ??? {
        "gold"
    } else if ??? {
        "silver"
    } else {
        "bronze"
    };
    tier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn gold()    { assert_eq!(tier_name(120), "gold"); }
    #[test] fn boundary_gold() { assert_eq!(tier_name(100), "gold"); }
    #[test] fn silver()  { assert_eq!(tier_name(75),  "silver"); }
    #[test] fn boundary_silver() { assert_eq!(tier_name(50), "silver"); }
    #[test] fn bronze()  { assert_eq!(tier_name(10),  "bronze"); }
    #[test] fn zero()    { assert_eq!(tier_name(0),   "bronze"); }
}

fn main() {}
