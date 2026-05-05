// =============================================================================
//  own2 — Clone vs Copy
// =============================================================================
//
// Last exercise: moves. This exercise: how to KEEP a value usable when you
// also need to hand it off.
//
// Two traits you'll meet again and again:
//
//   `Copy`     — implicit, bitwise copy. No allocation, no work. Used by
//                small stack-only types: integers, floats, bools, chars,
//                shared references `&T`, tuples/arrays of Copy types.
//                You never call anything — the compiler does it for you on
//                assignment, function call, and pattern bind.
//
//   `Clone`    — EXPLICIT, possibly expensive. You write `.clone()`.
//                For `String` it allocates a brand-new heap buffer and
//                memcpy's the bytes over. For `Vec<T>` likewise.
//
// Every `Copy` type is also `Clone`. The reverse is NOT true — `String` is
// `Clone` but not `Copy`. (If `String` were `Copy`, `let b = a` would
// duplicate the heap pointer silently, and you'd be back to double-free
// territory.)
//
// THE PERF COST OF .clone()
//
//   - Cloning a `String` of length N is O(N) and one heap allocation.
//   - Cloning an `Rc<T>` is O(1) (a refcount bump). Same name, different cost.
//   - Cloning an `i32` is free (it's just `Copy` under the hood).
//
// `.clone()` is not a code smell — it's a tool. But if you find yourself
// cloning in a hot loop, that's the spot to look at borrowing instead
// (next exercises).
//
// COPY vs CLONE — a quick demo:
//
//     let a: i32    = 42;
//     let b         = a;          // COPY: a still valid
//     let s1: String = String::from("hi");
//     let s2         = s1.clone(); // CLONE: both valid, two heap buffers
//     let s3         = s1;         // MOVE:  s3 valid, s1 invalidated
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `duplicate_string` takes a `&String` and returns TWO owned copies as a
//     tuple. (Yes, taking `&String` is unusual — `&str` is more idiomatic —
//     but we want the focus on cloning here.)
//
//   - `pair_of_ints` shows the Copy side: take an `i32`, return `(i32, i32)`
//     of (n, n). NO `.clone()` needed because `i32: Copy`.

// I AM NOT DONE

fn duplicate_string(s: &String) -> (String, String) {
    let a = s.???();    // make an owned copy
    let b = s.???();    // and another
    (a, b)
}

fn pair_of_ints(n: i32) -> (i32, i32) {
    // No clone needed — i32 is Copy.
    (n, ???)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn dup_basic() {
        let s = String::from("crab");
        let (a, b) = duplicate_string(&s);
        assert_eq!(a, "crab");
        assert_eq!(b, "crab");
        // The original is still usable — we only borrowed it.
        assert_eq!(s, "crab");
    }

    #[test] fn dup_independent() {
        // After cloning, the two strings are SEPARATE allocations.
        // Mutating one does not affect the other.
        let s = String::from("a");
        let (mut a, b) = duplicate_string(&s);
        a.push('!');
        assert_eq!(a, "a!");
        assert_eq!(b, "a");
    }

    #[test] fn ints_pair() {
        assert_eq!(pair_of_ints(7), (7, 7));
    }
}

fn main() {}
