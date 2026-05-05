// =============================================================================
//  vec4 — iterating: `iter`, `iter_mut`, `into_iter`
// =============================================================================
//
// A `Vec<T>` exposes three flavours of iteration. Pick by how much access
// you need:
//
//     v.iter()        // yields &T        — read-only, Vec untouched
//     v.iter_mut()    // yields &mut T    — mutate elements in place
//     v.into_iter()   // yields T         — CONSUMES the Vec, takes ownership
//
// The `for x in ...` form picks one for you depending on what you write:
//
//     for x in &v       { /* x: &T      */ }   // same as v.iter()
//     for x in &mut v   { /* x: &mut T  */ }   // same as v.iter_mut()
//     for x in v        { /* x: T       */ }   // same as v.into_iter() — v is GONE
//
// PATTERN-DESTRUCTURING THE REFERENCE
//
// When the iterator yields `&i32` and you want a plain `i32`, write `&x`:
//
//     for &x in &v { let y: i32 = x + 1; ... }
//
// Without the `&`, `x` is `&i32` and you must deref with `*x`.
//
// MUTATING IN PLACE
//
//     for x in &mut v { *x *= 2; }   // doubles every element
//
// The `*x` is required because `x` is `&mut i32`, and you assign through
// the reference.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `sum_borrow(v)`: borrow `v` (don't consume it), return the sum.
//                     Use `for &x in v { ... }` with a mutable accumulator.
//   - `double_in_place(v)`: multiply every element by 2 using `iter_mut`
//                           (or `for x in &mut v`).
//   - `sum_consume(v)`: TAKE the Vec by value and return the sum.
//                       Use `for x in v { ... }` (into_iter, x: i32).
//
// Iterator adapters like `.sum()` / `.map()` get the deep treatment in
// chapter 7. Stick with explicit `for` loops here.
//
// Do NOT touch the tests.

// I AM NOT DONE

fn sum_borrow(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for ???x in v {
        total += x;
    }
    total
}

fn double_in_place(v: &mut Vec<i32>) {
    for x in ??? v {
        ???x *= 2;
    }
}

fn sum_consume(v: Vec<i32>) -> i32 {
    let mut total = 0;
    for x in ??? {
        total += x;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn borrow_sum() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(sum_borrow(&v), 10);
        // v still usable here:
        assert_eq!(v.len(), 4);
    }
    #[test] fn double_works() {
        let mut v = vec![1, 2, 3];
        double_in_place(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }
    #[test] fn consume_sum() {
        let v = vec![10, 20, 30];
        assert_eq!(sum_consume(v), 60);
        // v is gone — the next line would not compile:
        // let _ = v.len();
    }
    #[test] fn empty_sum() {
        assert_eq!(sum_borrow(&vec![]), 0);
        assert_eq!(sum_consume(vec![]), 0);
    }
}

fn main() {}
