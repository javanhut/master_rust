// SOLUTION — intro5_shadowing

fn parse_and_double(n: &str) -> i64 {
    let n: i64 = n.parse().unwrap();
    n * 2
}

// WHY THIS IS OPTIMAL:
//   - `n` arrives as `&str`, leaves the function's brain as `i64`. Reusing
//     the name keeps the call-site logic on a single noun ("the input").
//   - The annotation `: i64` is essential — without it the compiler can't
//     pick a `parse` target.
//
// CONTRAST WITH MUTABILITY:
//   You CANNOT do this with `let mut`:
//
//       let mut n = "42";
//       n = n.parse::<i64>().unwrap();   // ❌ expected &str, found i64
//
//   `let mut` keeps the type the same. Shadowing is the right tool when
//   the type CHANGES.
//
// ERROR HANDLING NOTE:
//   `unwrap()` panics on a parse failure ("hello".parse::<i64>() panics).
//   In real code you'd write `?` or `.map_err(...)`. We'll cover that in
//   the error-handling chapter — for now, `unwrap` is the convenient
//   "I know this won't fail" escape hatch for tests and example code.
