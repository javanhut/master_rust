// SOLUTION — types_quiz

fn stats(xs: &[i32]) -> (i32, i32, f64) {
    assert!(!xs.is_empty(), "stats() requires a non-empty slice");

    let mut mn = xs[0];
    let mut mx = xs[0];
    let mut sum: i64 = 0;

    for &x in xs {
        if x < mn { mn = x; }
        if x > mx { mx = x; }
        sum += x as i64;
    }

    let mean = (sum as f64) / (xs.len() as f64);
    (mn, mx, mean)
}

// WHY THIS IS OPTIMAL FOR A FROM-SCRATCH IMPLEMENTATION:
//   - Single pass through the slice (O(n)).
//   - Seeds mn/mx with `xs[0]`, which is correct AND readable. Seeding
//     with i32::MAX/MIN works numerically but introduces a "what about
//     empty slices?" question we'd rather answer once, up front, with the
//     `assert!`.
//   - Sum accumulated in i64 prevents overflow when many large i32s are
//     added together.
//   - The final mean cast happens at the right moment — after summing,
//     before dividing.
//
// IDIOMATIC NEXT-LEVEL VERSION (preview of iterator chapter):
//
//     fn stats(xs: &[i32]) -> (i32, i32, f64) {
//         let mn = *xs.iter().min().unwrap();
//         let mx = *xs.iter().max().unwrap();
//         let sum: i64 = xs.iter().map(|&x| x as i64).sum();
//         (mn, mx, sum as f64 / xs.len() as f64)
//     }
//
//   Three passes instead of one — but iterators inline so well that the
//   compiler often produces near-identical code. The single-pass loop
//   wins when the slice is huge (cache locality) and when iterator
//   adaptors aren't yet familiar.
//
// REAL-WORLD UPGRADE:
//   In production this function should return `Option<(i32, i32, f64)>`
//   instead of asserting. We'll learn `Option` and the `?` operator soon.
