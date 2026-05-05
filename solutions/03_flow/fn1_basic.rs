// SOLUTION — fn1_basic

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(n: i32) -> i32 {
    n * n
}

// WHY THIS IS OPTIMAL:
//   - One expression each. No `return`, no semicolons, no temporaries.
//   - `n * n` is conventionally faster to read and write than `n.pow(2)`
//     for the simple square case, and the compiler emits the same code.
