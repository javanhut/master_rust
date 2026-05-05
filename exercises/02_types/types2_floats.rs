// =============================================================================
//  types2 — floating-point numbers
// =============================================================================
//
// Two float types: `f32` (single precision) and `f64` (double precision).
// `f64` is the default for unsuffixed float literals and is what you almost
// always want.
//
//     let pi = 3.14159;       // f64
//     let g  = 9.81f32;       // f32
//
// FLOATS DON'T MIX WITH INTS
//
// Rust will NOT silently combine `i32` with `f64`. You must convert:
//
//     let n = 5;
//     let r = 2.0;
//     // r * n;                  // ❌ mismatched types
//        r * n as f64;            // ✅
//
// ASSOCIATED CONSTANTS & METHODS
//
//     f64::MAX          f64::INFINITY        f64::NAN
//     x.sqrt()          x.powi(3)            x.powf(2.5)
//     x.abs()           x.round()            x.floor()  x.ceil()
//     a.min(b)          a.max(b)
//
// COMPARING FLOATS
//
// Floats are TRICKY: NaN is not equal to anything, including itself. For
// "almost equal" comparisons in tests use a tolerance:
//
//     (a - b).abs() < 1e-9
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Implement `circle_area` correctly using f64.
//   - Implement `mean` to return the average of an `i32` slice as `f64`.

// I AM NOT DONE

const PI: f64 = std::f64::consts::PI;

fn circle_area(radius: f64) -> f64 {
    // area = π · r²
    PI * radius.???(2)         // a method that raises to an integer power
}

fn mean(xs: &[i32]) -> f64 {
    // Sum the values (as i64 to avoid overflow), then divide by len.
    // You will need TWO casts here: `as i64` for the values, and `as f64`
    // for the final division.
    let mut total: i64 = 0;
    for &x in xs {
        total += x as ???;
    }
    (total ??? f64) / (xs.len() ??? f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool { (a - b).abs() < 1e-9 }

    #[test] fn area_unit() { assert!(approx(circle_area(1.0), PI)); }
    #[test] fn area_two()  { assert!(approx(circle_area(2.0), 4.0 * PI)); }

    #[test] fn mean_basic()    { assert!(approx(mean(&[1, 2, 3, 4]), 2.5)); }
    #[test] fn mean_negatives() { assert!(approx(mean(&[-1, 1]), 0.0)); }
}

fn main() {}
