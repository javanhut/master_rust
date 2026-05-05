// SOLUTION — types2_floats

const PI: f64 = std::f64::consts::PI;

fn circle_area(radius: f64) -> f64 {
    PI * radius.powi(2)
}

fn mean(xs: &[i32]) -> f64 {
    let mut total: i64 = 0;
    for &x in xs {
        total += x as i64;
    }
    (total as f64) / (xs.len() as f64)
}

// WHY THIS IS OPTIMAL:
//
//   powi(2) — integer exponent, fast (one multiply). Compare:
//     powf(2.0)  — float exponent. Slower; uses logarithms internally.
//     r * r      — same speed as powi, slightly less self-documenting for
//                  exponents > 2.
//
//   We accumulate into `i64` because adding many `i32`s into another `i32`
//   is a great way to overflow silently in release builds. `i64` gives us
//   plenty of headroom.
//
//   We cast to `f64` at the LAST possible moment, then divide. Doing the
//   division in `i64` first would lose all the fractional information.
//
// IDIOMATIC NEXT-LEVEL VERSION:
//
//     fn mean(xs: &[i32]) -> f64 {
//         let total: i64 = xs.iter().map(|&x| x as i64).sum();
//         total as f64 / xs.len() as f64
//     }
//
//   Same logic, no mutation. Iterators are coming up — this is what the
//   manual loop "wants to be" once you know `.sum()`.
//
// EDGE CASE TO BEWARE OF:
//   `mean(&[])` will panic on division by 0.0 → infinity actually doesn't
//   panic for f64; you get NaN ((0 as f64) / (0 as f64) == NaN). The test
//   suite doesn't exercise empty slices; in production you'd return
//   `Option<f64>` or take a `NonEmpty` slice type.
