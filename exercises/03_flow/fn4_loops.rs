// =============================================================================
//  fn4 — loops
// =============================================================================
//
// Three loop constructs, each with a clear use case:
//
// 1. `loop { ... }`           — INFINITE loop. Exit with `break;`.
//                               Powerful trick: `break VALUE;` makes the
//                               whole `loop { ... }` an expression equal
//                               to `VALUE`. `while`/`for` cannot do this.
//
//        let answer = loop {
//            let guess = ask();
//            if good(guess) { break guess; }
//        };
//
// 2. `while cond { ... }`     — repeat while `cond` is true. Re-evaluated
//                               each iteration.
//
// 3. `for x in iter { ... }`  — iterate over anything that implements
//                               `IntoIterator`. The most common form:
//
//        for i in 0..10      { /* i = 0, 1, ..., 9      */ }
//        for i in 0..=10     { /* i = 0, 1, ..., 10     */ }
//        for s in &["a","b"] { /* s = &"a", &"b"        */ }
//
// `break` and `continue` work in all three. `break` can take a value (only
// in `loop`).  Both can take a LOOP LABEL when nested:
//
//        'outer: for i in 0..n {
//            for j in 0..n {
//                if done(i, j) { break 'outer; }
//            }
//        }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `count_evens(n)`: count even numbers in 0..=n using a `for` loop.
//   - `first_above(threshold, xs)`: scan the slice and return the FIRST
//     value greater than `threshold`, wrapped in `Some`. If none exists,
//     return `None`. Use `loop { ... break VALUE; }` to demonstrate that
//     `break` can return a value from a `loop` expression.

// I AM NOT DONE

fn count_evens(n: u32) -> u32 {
    let mut count = 0;
    for i in 0..=n {
        if i % 2 == 0 {
            count ??? 1;
        }
    }
    count
}

fn first_above(threshold: i32, xs: &[i32]) -> Option<i32> {
    let mut idx = 0;
    let result = loop {
        if idx >= xs.len() {
            break None;
        }
        if xs[idx] > threshold {
            break ???(xs[idx]);
        }
        idx += 1;
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn evens_up_to_10() { assert_eq!(count_evens(10), 6); } // 0,2,4,6,8,10
    #[test] fn evens_zero()     { assert_eq!(count_evens(0),  1); } // 0 itself
    #[test] fn evens_one()      { assert_eq!(count_evens(1),  1); }

    #[test] fn first_above_found() {
        assert_eq!(first_above(3, &[1, 2, 3, 4, 5]), Some(4));
    }
    #[test] fn first_above_missing() {
        assert_eq!(first_above(99, &[1, 2, 3]), None);
    }
    #[test] fn first_above_empty() {
        assert_eq!(first_above(0, &[]), None);
    }
}

fn main() {}
