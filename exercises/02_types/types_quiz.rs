// =============================================================================
//  types_quiz — chapter 2 capstone
// =============================================================================
//
// Combine integers, floats, tuples, and arrays.
//
// Implement `stats(xs)` that takes a slice of i32 and returns a tuple
// `(min, max, mean_as_f64)`. Treat an empty slice as a panic — we'll learn
// graceful error handling soon. For now, asking for stats of nothing is a
// programmer error.
//
//     stats(&[3, 1, 4, 1, 5, 9, 2, 6])  ==  (1, 9, 3.875)
//
// =============================================================================

// I AM NOT DONE

fn stats(xs: &[i32]) -> (i32, i32, f64) {
    assert!(!xs.is_empty(), "stats() requires a non-empty slice");

    // Initialise mn / mx with the first element. (Why not i32::MAX / MIN?
    // Because then an all-MAX slice would still produce the right answer,
    // but seeding from the first real value reads more clearly and never
    // needs to revisit the question of what to do for empty slices.)
    let mut mn = xs[0];
    let mut mx = xs[0];
    let mut sum: i64 = 0;

    for &x in xs {
        if x < mn { mn = x; }
        if x ??? mx { mx = x; }
        sum += x as i64;
    }

    let mean = (sum as f64) / (??? as f64);
    (mn, mx, mean)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool { (a - b).abs() < 1e-9 }

    #[test] fn single() {
        let (mn, mx, m) = stats(&[42]);
        assert_eq!(mn, 42);
        assert_eq!(mx, 42);
        assert!(approx(m, 42.0));
    }

    #[test] fn pi_digits() {
        let (mn, mx, m) = stats(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(mn, 1);
        assert_eq!(mx, 9);
        assert!(approx(m, 3.875));
    }

    #[test] fn negatives() {
        let (mn, mx, m) = stats(&[-3, -1, -4]);
        assert_eq!(mn, -4);
        assert_eq!(mx, -1);
        assert!(approx(m, -8.0 / 3.0));
    }
}

fn main() {}
