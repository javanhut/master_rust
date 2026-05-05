// =============================================================================
//  iter5 — combining and slicing iterators: zip, enumerate, chain, take, skip
// =============================================================================
//
// A small tour of adapters that stitch iterators together or chop them up.
// All lazy, all cheap, all stack on each other.
//
// ENUMERATE — pair each item with its index
//
//     for (i, c) in "abc".chars().enumerate() {
//         println!("{i}: {c}");      // 0: a / 1: b / 2: c
//     }
//
//   `.enumerate()` yields `(usize, Item)`. Indices start at 0 and count
//   the items the underlying iterator actually produces — so if you put
//   `.filter(...)` before `.enumerate()`, the index is the position in the
//   filtered stream, not the original.
//
// ZIP — pair items from two iterators
//
//     let names  = ["Alice", "Bob", "Carol"];
//     let scores = [10, 20, 30, 40];
//     for (n, s) in names.iter().zip(scores.iter()) { /* ... */ }
//
//   `.zip` stops as soon as EITHER side runs out — three pairs above, the
//   trailing `40` is dropped silently.
//
// CHAIN — concatenate two iterators
//
//     let a = [1, 2];
//     let b = [3, 4, 5];
//     let all: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
//     // [1, 2, 3, 4, 5]
//
//   The two iterators must yield the SAME item type.
//
// TAKE / SKIP / STEP_BY — cropping
//
//     (1..).take(5)              // first 5 elements: 1,2,3,4,5
//     (1..10).skip(3)            // 4,5,6,7,8,9
//     (0..20).step_by(5)         // 0,5,10,15
//
//   `take` is the standard way to make an INFINITE iterator finite. `(1..)`
//   counts forever; `.take(N)` makes it stop.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `index_of_max(xs)`: return the index of the largest element of `xs`
//     using `.iter().enumerate().max_by_key(|(_, v)| **v).map(|(i, _)| i)`.
//     If `xs` is empty, return `None`.
//   - `dot_product(a, b)`: classic dot product. Sum of `a[i]*b[i]`. Use
//     `a.iter().zip(b.iter()).map(...).sum()`. Stop at the shorter slice.
//   - `head_then_tail(head, tail)`: concatenate two slices into one Vec
//     using `.chain()`.
//   - `evens_then_first_three(n)`: take only the first three even numbers
//     from `0, 1, 2, ...` (use `(0..).step_by(2).take(3).collect()`).

// I AM NOT DONE

fn index_of_max(xs: &[i32]) -> Option<usize> {
    xs.iter()
        .???()                                  // pair each item with its index
        .max_by_key(|(_, v)| ???)               // largest by value
        .map(|(i, _)| i)
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().???(b.iter())                      // pair items 1:1
        .map(|(x, y)| ???)
        .sum()
}

fn head_then_tail(head: &[i32], tail: &[i32]) -> Vec<i32> {
    head.iter().???(tail.iter()).copied().collect()
}

fn evens_then_first_three() -> Vec<i32> {
    (0..).???(2).???(3).collect()                // step_by then take
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn idx_basic()  { assert_eq!(index_of_max(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(5)); }
    #[test] fn idx_first()  { assert_eq!(index_of_max(&[9, 1, 2]),                Some(0)); }
    #[test] fn idx_empty()  { assert_eq!(index_of_max(&[]),                       None);    }

    #[test] fn dot_basic()  { assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32); }
    #[test] fn dot_short_b(){ assert_eq!(dot_product(&[1, 2, 3, 4], &[10, 10]), 30); }
    #[test] fn dot_empty()  { assert_eq!(dot_product(&[], &[1, 2, 3]),       0);  }

    #[test] fn chain_basic() {
        assert_eq!(head_then_tail(&[1, 2], &[3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }
    #[test] fn chain_empties() {
        assert_eq!(head_then_tail(&[], &[1]),  vec![1]);
        assert_eq!(head_then_tail(&[1], &[]),  vec![1]);
        assert_eq!(head_then_tail(&[], &[]),   Vec::<i32>::new());
    }

    #[test] fn evens3() { assert_eq!(evens_then_first_three(), vec![0, 2, 4]); }
}

fn main() {}
