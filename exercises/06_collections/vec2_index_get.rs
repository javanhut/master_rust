// =============================================================================
//  vec2 — indexing, `get`, `len`, `is_empty`
// =============================================================================
//
// Two ways to look at element `i` of a `Vec<T>`:
//
//     v[i]        // returns T (by value if Copy, else this borrows / moves
//                 // depending on context). PANICS at runtime if i >= len.
//
//     v.get(i)    // returns Option<&T>.
//                 //   Some(&value) if i < len
//                 //   None         otherwise
//                 // Never panics — you handle the missing case.
//
// Use `v[i]` when you've already proven `i` is in range (e.g. you just
// computed `i` from `0..v.len()`). Use `v.get(i)` whenever the index might be
// out of bounds and a panic would be wrong.
//
// LENGTH AND EMPTINESS
//
//     v.len()        // usize, current number of elements
//     v.is_empty()   // bool, equivalent to v.len() == 0 but reads better
//
// Clippy will scold you for writing `v.len() == 0` — `is_empty` is the
// idiomatic form, and on some collections it's measurably faster.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `safe_first(v)`: return the first element copied out, or None if empty.
//                     Use `.get(...)` and `.copied()` (turns Option<&i32>
//                     into Option<i32>).
//   - `length(v)`: return `v.len()`.
//   - `is_blank(v)`: return whether `v` is empty — use `is_empty`.
//   - `unsafe_at(v, i)`: return `v[i]` — yes this can panic; that's the point
//                        of contrast.
//
// Do NOT touch the tests.

// I AM NOT DONE

fn safe_first(v: &Vec<i32>) -> Option<i32> {
    v.???(0).???()
}

fn length(v: &Vec<i32>) -> usize {
    v.???()
}

fn is_blank(v: &Vec<i32>) -> bool {
    v.???()
}

fn unsafe_at(v: &Vec<i32>, i: usize) -> i32 {
    v[???]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_some()  { assert_eq!(safe_first(&vec![10, 20]), Some(10)); }
    #[test] fn first_none()  { assert_eq!(safe_first(&vec![]), None); }
    #[test] fn len_three()   { assert_eq!(length(&vec![1, 2, 3]), 3); }
    #[test] fn len_zero()    { assert_eq!(length(&vec![]), 0); }
    #[test] fn blank_yes()   { assert!(is_blank(&vec![])); }
    #[test] fn blank_no()    { assert!(!is_blank(&vec![1])); }
    #[test] fn at_works()    { assert_eq!(unsafe_at(&vec![7, 8, 9], 2), 9); }

    #[test]
    #[should_panic]
    fn at_panics_oob() {
        let _ = unsafe_at(&vec![1, 2, 3], 99);
    }
}

fn main() {}
