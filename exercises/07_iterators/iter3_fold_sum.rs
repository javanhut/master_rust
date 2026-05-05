// =============================================================================
//  iter3 — reducing: `sum`, `product`, `count`, `fold`
// =============================================================================
//
// "Reducing" means collapsing an iterator's stream of values down to ONE
// final value. Rust gives you a few specialised reducers and one general one.
//
// THE SPECIALISED REDUCERS
//
//     iter.sum::<T>()        // adds them up
//     iter.product::<T>()    // multiplies them together
//     iter.count()           // how many items? (returns usize)
//
//   Examples:
//
//       let s: i32 = (1..=4).sum();         // 10
//       let p: i32 = (1..=4).product();     // 24
//       let n      = (1..=4).count();       // 4
//
//   For `sum` and `product` you usually annotate the result type because
//   the trait can't tell which numeric type you want. Annotate the binding
//   (`let s: i32 = ...`) or use the turbofish (`.sum::<i32>()`).
//
// FOLD — THE GENERAL FORM
//
// All of the above are special cases of `fold`:
//
//     iter.fold(initial, |acc, item| new_acc)
//
// Walk the iterator, threading an accumulator through:
//
//     let s = (1..=4).fold(0, |acc, x| acc + x);     // 10  — same as sum
//     let p = (1..=4).fold(1, |acc, x| acc * x);     // 24  — same as product
//     let n = (1..=4).fold(0, |acc, _| acc + 1);     // 4   — same as count
//
// Fold's closure does NOT receive a reference like `filter`'s does — it gets
// the item by whatever the iterator yields. If your iterator yields `&i32`,
// pattern-destructure with `|acc, &x|`.
//
// WHEN TO REACH FOR FOLD
//
//   - The accumulator type differs from the item type
//     (e.g. building a `String` from `char`s).
//   - The combination logic is non-trivial — branching, conditional updates,
//     custom state structs.
//   - You want one pass and the existing `sum`/`max`/`count` aren't enough.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `sum_via_sum(xs)`: sum a slice using `.iter().sum()`.
//   - `product_of(xs)`: multiply every element using `.iter().product()`.
//   - `sum_via_fold(xs)`: same answer as `sum_via_sum` but written with
//     `.fold(0, |acc, x| ...)`. Reimplementing sum from scratch.
//   - `count_via_fold(xs)`: count elements using `.fold(0usize, ...)`.
//     (Of course `.count()` exists; we're flexing fold here.)

// I AM NOT DONE

fn sum_via_sum(xs: &[i32]) -> i32 {
    xs.iter().???()
}

fn product_of(xs: &[i32]) -> i32 {
    xs.iter().???()
}

fn sum_via_fold(xs: &[i32]) -> i32 {
    xs.iter().fold(???, |acc, &x| ???)
}

fn count_via_fold(xs: &[i32]) -> usize {
    xs.iter().fold(0usize, |acc, _| ???)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sum_basic() { assert_eq!(sum_via_sum(&[1, 2, 3, 4]), 10); }
    #[test] fn sum_empty() { assert_eq!(sum_via_sum(&[]),           0);  }

    #[test] fn product_basic() { assert_eq!(product_of(&[1, 2, 3, 4]), 24); }
    #[test] fn product_empty() { assert_eq!(product_of(&[]),           1);  }
    #[test] fn product_zero()  { assert_eq!(product_of(&[1, 0, 3]),    0);  }

    #[test] fn fold_sum_matches_sum() {
        for xs in [&[][..], &[1, 2, 3], &[-5, 5, 10, -1]] {
            assert_eq!(sum_via_fold(xs), sum_via_sum(xs));
        }
    }

    #[test] fn fold_count() {
        assert_eq!(count_via_fold(&[]),           0);
        assert_eq!(count_via_fold(&[1, 2, 3, 4]), 4);
    }
}

fn main() {}
