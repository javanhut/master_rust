// =============================================================================
//  iter_quiz — capstone: rewrite chapter 3 in iterator chains
// =============================================================================
//
// In chapter 3 you wrote `count_evens`, `first_above`, and a fizzbuzz with
// explicit `for`/`while`/`loop` constructs. Mutable accumulators, indices,
// and `break`s. They worked. They were also a lot of code.
//
// Same problems, iterator-style. Each function below has a one-or-two-line
// body. The point of this exercise is to FEEL how much the chain shrinks
// the imperative version while staying just as readable.
//
// REWRITE GUIDE
//
//   count_evens(n: u32) -> u32
//       Old: a `for i in 0..=n` loop with a counter.
//       New: `(0..=n).filter(|x| x % 2 == 0).count() as u32`.
//
//   first_above(threshold, xs: &[i32]) -> Option<i32>
//       Old: `loop { match index ... break Some(...); }`.
//       New: `xs.iter().copied().find(|&x| x > threshold)`.
//
//   fizzbuzz(n: u32) -> Vec<String>
//       Old: a `for` loop pushing onto a Vec, with cascading if/else.
//       New: `(1..=n).map(|i| match (i % 3, i % 5) { ... }).collect()`.
//       Use a `match` on the tuple `(i % 3, i % 5)` for the cleanest version:
//
//           (0, 0) => "FizzBuzz".to_string(),
//           (0, _) => "Fizz".to_string(),
//           (_, 0) => "Buzz".to_string(),
//           (_, _) => i.to_string(),
//
//   sum_of_squares_of_evens(xs: &[i32]) -> i32
//       The flagship one-liner: filter, map, sum.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`. Each function should be a SINGLE iterator chain.

// I AM NOT DONE

fn count_evens(n: u32) -> u32 {
    (0..=n).???(|x| x % 2 == 0).count() as u32
}

fn first_above(threshold: i32, xs: &[i32]) -> Option<i32> {
    xs.iter().copied().???(|&x| x > threshold)
}

fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => ???.to_string(),
            (0, _) => ???.to_string(),
            (_, 0) => ???.to_string(),
            (_, _) => i.???(),
        })
        .???()                                          // collect
}

fn sum_of_squares_of_evens(xs: &[i32]) -> i32 {
    xs.iter()
        .copied()
        .???(|&x| x % 2 == 0)                            // keep evens
        .???(|x| x * x)                                  // square them
        .???()                                           // sum the squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn evens_basic() { assert_eq!(count_evens(10), 6); }
    #[test] fn evens_zero()  { assert_eq!(count_evens(0),  1); }
    #[test] fn evens_one()   { assert_eq!(count_evens(1),  1); }

    #[test] fn first_above_found()   { assert_eq!(first_above(3, &[1,2,3,4,5]), Some(4)); }
    #[test] fn first_above_missing() { assert_eq!(first_above(99,&[1,2,3]),     None);    }
    #[test] fn first_above_empty()   { assert_eq!(first_above(0, &[]),          None);    }

    #[test] fn fizzbuzz_15() {
        let v = fizzbuzz(15);
        assert_eq!(v.len(), 15);
        assert_eq!(v[0],  "1");
        assert_eq!(v[1],  "2");
        assert_eq!(v[2],  "Fizz");
        assert_eq!(v[3],  "4");
        assert_eq!(v[4],  "Buzz");
        assert_eq!(v[5],  "Fizz");
        assert_eq!(v[8],  "Fizz");
        assert_eq!(v[9],  "Buzz");
        assert_eq!(v[14], "FizzBuzz");
    }

    #[test] fn squares_basic() {
        // evens are 2, 4, 6 — squares 4 + 16 + 36 = 56
        assert_eq!(sum_of_squares_of_evens(&[1, 2, 3, 4, 5, 6]), 56);
    }
    #[test] fn squares_no_evens() {
        assert_eq!(sum_of_squares_of_evens(&[1, 3, 5]), 0);
    }
    #[test] fn squares_empty() {
        assert_eq!(sum_of_squares_of_evens(&[]), 0);
    }
}

fn main() {}
