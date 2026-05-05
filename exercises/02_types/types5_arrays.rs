// =============================================================================
//  types5 — arrays
// =============================================================================
//
// An ARRAY is a fixed-length, same-type sequence laid out contiguously in
// memory. The TYPE is `[T; N]` — both element type AND length are part of
// the type:
//
//     let a: [i32; 3] = [1, 2, 3];
//     let zeros: [u8; 4] = [0; 4];     // shorthand: [value; length]
//
// LENGTH IS COMPILE-TIME-KNOWN
//
//     [i32; 3]   and   [i32; 4]    are DIFFERENT types.
//
// If you want a runtime-resizable list, use `Vec<T>` (later chapter).
// If you want to write code that works on arrays of ANY length, use a SLICE,
// which is the type `&[T]` — a pointer + length, length not in the type:
//
//     fn sum(xs: &[i32]) -> i32 { ... }
//     sum(&[1, 2, 3]);          // pass an array reference, coerces to slice
//     sum(&vec![1, 2, 3, 4]);   // also works — Vec coerces to slice too
//
// INDEXING is `arr[i]`. It panics if `i >= len`. For a non-panicking lookup
// use `arr.get(i)` which returns `Option<&T>`.
//
// LOOPING
//
//     for x in arr.iter() { ... }     // x: &T
//     for x in &arr       { ... }     // same — by reference
//     for &x in &arr      { ... }     // x: T   (pattern destructures)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define `WEEK` as a length-7 array of strings.
//   - Implement `sum_slice` for any-length i32 slices.

// I AM NOT DONE

const WEEK: [&str; ???] = [
    "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
];

fn sum_slice(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {
        total ??? x;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn week_len()    { assert_eq!(WEEK.len(), 7); }
    #[test] fn week_first()  { assert_eq!(WEEK[0], "Mon"); }
    #[test] fn week_last()   { assert_eq!(WEEK[6], "Sun"); }

    #[test] fn sum_empty()   { assert_eq!(sum_slice(&[]), 0); }
    #[test] fn sum_one()     { assert_eq!(sum_slice(&[42]), 42); }
    #[test] fn sum_many()    { assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15); }
    #[test] fn sum_negative(){ assert_eq!(sum_slice(&[-1, -2, 3]), 0); }
}

fn main() {}
