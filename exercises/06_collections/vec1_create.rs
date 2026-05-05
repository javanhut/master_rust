// =============================================================================
//  vec1 — creating a `Vec<T>`
// =============================================================================
//
// `Vec<T>` is Rust's growable, heap-allocated, contiguous list. Where an array
// `[T; N]` bakes its length into the type, a `Vec<T>` carries its length (and
// capacity) at runtime and can grow as needed.
//
// THREE WAYS TO CREATE ONE
//
//     let a: Vec<i32> = Vec::new();          // empty, no allocation yet
//     let b = vec![1, 2, 3];                 // macro: builds a Vec<i32>
//     let c: Vec<i32> = Vec::with_capacity(100); // empty but pre-allocated
//
// `Vec::new()` allocates NOTHING up front. The first `push` triggers a heap
// allocation. Each time the buffer fills, Vec doubles its capacity and copies.
//
// `vec![...]` is just sugar for `Vec::new()` plus pushes. Use it when you know
// the contents at compile time.
//
// `Vec::with_capacity(n)` reserves space for `n` elements right now. The
// length is still 0 — capacity ≠ length. Useful when you know roughly how
// many things you're about to push: it avoids the doubling-and-copying.
//
// LEN vs CAPACITY
//
//     let mut v = Vec::with_capacity(10);
//     v.push(1);
//     assert_eq!(v.len(), 1);        // one element
//     assert!(v.capacity() >= 10);   // room for at least 10
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_empty()`: return an empty `Vec<i32>` using `Vec::new()`.
//   - `make_three()`: return `vec![1, 2, 3]` using the macro.
//   - `make_reserved(n)`: return an empty `Vec<i32>` with capacity AT LEAST n.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn make_empty() -> Vec<i32> {
    Vec::???()
}

fn make_three() -> Vec<i32> {
    ???![1, 2, 3]
}

fn make_reserved(n: usize) -> Vec<i32> {
    Vec::???(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty_is_empty()  { assert_eq!(make_empty().len(), 0); }
    #[test] fn three_len()       { assert_eq!(make_three().len(), 3); }
    #[test] fn three_contents()  { assert_eq!(make_three(), vec![1, 2, 3]); }
    #[test] fn reserved_len_zero() {
        let v = make_reserved(16);
        assert_eq!(v.len(), 0);
        assert!(v.capacity() >= 16);
    }
}

fn main() {}
