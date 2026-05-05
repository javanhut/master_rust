// =============================================================================
//  own5 — mutable references (`&mut T`)
// =============================================================================
//
// Shared references let you read. MUTABLE references let you write:
//
//     fn add_bang(s: &mut String) {
//         s.push('!');
//     }
//
//     let mut s = String::from("hi");
//     add_bang(&mut s);     // pass a mutable reference
//     assert_eq!(s, "hi!"); // s is still ours, and it's been modified.
//
// THREE THINGS THAT MUST ALL BE TRUE
//
//   1. The OWNER must itself be `mut`     — `let mut s = ...`.
//   2. You explicitly take `&mut s`       — not just `&s`.
//   3. The function parameter says `&mut` — `s: &mut String`.
//
// Forget any one of those and you'll get a compile error.
//
// THE EXCLUSIVITY RULE  (this is the headline)
//
//   While a `&mut T` exists, NOTHING else may access that value:
//     - no other `&mut T`,
//     - no `&T`,
//     - not even the original owner directly.
//
// This is what eliminates data races at compile time. The compiler proves
// that whoever has `&mut T` has EXCLUSIVE access for the duration of that
// reference's life.
//
// NLL — NON-LEXICAL LIFETIMES
//
// "Duration of a reference's life" is more precise than "the surrounding
// block". Modern Rust (NLL) ends a reference's life at its LAST USE:
//
//     let mut v = vec![1, 2, 3];
//     let first = &v[0];        // shared borrow starts
//     println!("{first}");       // last use — borrow ENDS HERE
//     v.push(4);                 // ✅ allowed: no live borrow anymore
//
// Without NLL the shared borrow would be considered alive until the end
// of the block, and the `v.push(4)` would be rejected.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `append_bang(s: &mut String)`         — push a '!' on to the end.
//   - `double_in_place(n: &mut i32)`        — multiply *n by 2.
//   - `swap_first_two(v: &mut Vec<i32>)`    — swap v[0] and v[1] (assume
//                                             at least 2 elements).
//
// You'll also notice the tests show you the `let mut` + `&mut` dance
// from the caller's side.

// I AM NOT DONE

fn append_bang(s: &mut String) {
    s.???('!');
}

fn double_in_place(n: ???) {
    *n = *n * 2;
}

fn swap_first_two(v: &mut Vec<i32>) {
    // Vec has a built-in `swap(i, j)` method. Use it.
    v.???(0, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn appends() {
        let mut s = String::from("hi");
        append_bang(&mut s);
        assert_eq!(s, "hi!");
    }

    #[test] fn doubles() {
        let mut n = 21;
        double_in_place(&mut n);
        assert_eq!(n, 42);
    }

    #[test] fn swaps() {
        let mut v = vec![1, 2, 3];
        swap_first_two(&mut v);
        assert_eq!(v, vec![2, 1, 3]);
    }

    #[test] fn nll_in_action() {
        // Demonstrates NLL: shared borrow ends at last use, then mutable
        // borrow is fine on the next line.
        let mut v = vec![10, 20, 30];
        let first = &v[0];
        assert_eq!(*first, 10);   // last use of `first`
        v.push(40);                // ✅ no live borrow here
        assert_eq!(v.len(), 4);
    }
}

fn main() {}
