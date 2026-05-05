// =============================================================================
//  fn1 — defining and calling functions
// =============================================================================
//
// FUNCTION DEFINITION SYNTAX
//
//     fn name(param: Type, other: Type) -> ReturnType {
//         // body
//     }
//
//   - PARAMETERS need explicit types — the compiler does not infer them.
//   - The RETURN ARROW `-> Type` is required if the function returns
//     anything other than the unit `()`. (Forgetting it is a common
//     beginner bug.)
//
// RETURNING A VALUE
//
//   The LAST expression in the body, with NO TRAILING SEMICOLON, is what
//   the function returns. Adding a `;` turns the expression into a
//   statement and the function returns `()` — and the compiler will
//   complain about a type mismatch.
//
//     fn double(n: i32) -> i32 {
//         n * 2          // ✅ this expression IS the return value
//     }
//
//     fn double(n: i32) -> i32 {
//         n * 2;         // ❌ ; makes it a statement; function returns ()
//     }
//
// You can use an explicit `return EXPR;` from anywhere — common for early
// exits, less common at the end of a function.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Implement `add` and `square` as one-line expressions (no `return`).

// I AM NOT DONE

fn add(a: i32, b: i32) -> i32 {
    ???
}

fn square(n: i32) -> i32 {
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn add_works()    { assert_eq!(add(2, 3), 5); }
    #[test] fn add_negative() { assert_eq!(add(-2, 3), 1); }
    #[test] fn square_zero()  { assert_eq!(square(0), 0); }
    #[test] fn square_seven() { assert_eq!(square(7), 49); }
    #[test] fn square_neg()   { assert_eq!(square(-4), 16); }
}

fn main() {}
