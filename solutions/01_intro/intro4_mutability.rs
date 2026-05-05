// SOLUTION — intro4_mutability

fn sum_first_n(n: u32) -> u32 {
    let mut total = 0u32;
    for i in 1..=n {
        total += i;
    }
    total
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//   The lesson is `mut` + `+=`. This is the canonical accumulator pattern:
//     - `let mut total` introduces a mutable binding initialised to 0.
//     - `1..=n` is an inclusive range — DON'T use `1..n` here, you'd miss n.
//     - `total += i` is sugar for `total = total + i`.
//
// IDIOMATIC RUST (NEXT-LEVEL — once you know iterators):
//
//     fn sum_first_n(n: u32) -> u32 {
//         (1..=n).sum()
//     }
//
//   - No mutation, no explicit loop, no accumulator.
//   - The compiler infers the sum type from the return type.
//   - In release builds this compiles to the same machine code as the loop.
//   - You'll learn `.sum()` properly in the iterators chapter.
//
// MATHEMATICAL ALTERNATIVE:
//
//     n * (n + 1) / 2          // closed-form Gauss sum
//
//   Constant time. For very large `n` this matters. Note we divide by 2
//   AFTER the multiply to keep precision; `n * (n + 1)` could overflow u32
//   when n is near u32::MAX, so a real implementation would cast to u64.
