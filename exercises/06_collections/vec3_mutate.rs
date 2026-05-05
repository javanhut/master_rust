// =============================================================================
//  vec3 — mutating a `Vec`: push, pop, insert, remove, swap_remove, retain
// =============================================================================
//
// All mutation methods require `&mut self`, so the binding must be `let mut v`.
//
// APPEND / TRIM AT THE END  (cheap — O(1) amortised)
//
//     v.push(x);      // add at the end
//     v.pop();        // -> Option<T>: removes and returns the last element
//
// INSERT / REMOVE IN THE MIDDLE  (expensive — O(n): shifts every later element)
//
//     v.insert(2, x); // place x at index 2, shifting 2..len one slot right
//     v.remove(2);    // -> T: pull index 2 out, shifting later elements left
//
// SWAP_REMOVE  (cheap — O(1), but DOES NOT preserve order)
//
//     v.swap_remove(2); // -> T: replaces index 2 with the last element, then pops
//
// Use `swap_remove` when order doesn't matter. Use `remove` only when it does
// and the Vec is small or the index is near the end.
//
// RETAIN — drop everything that doesn't match a predicate
//
//     v.retain(|x| *x > 0); // keeps only positive values, in-place, O(n)
//
// COST CHEAT-SHEET
//
//     push / pop          O(1) amortised
//     insert / remove     O(n)  (every element after the index moves)
//     swap_remove         O(1)  (order destroyed)
//     retain              O(n)  (single pass, in-place)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `push_zero(v)`:        push 0 onto v.
//   - `pop_last(v)`:         return the popped value (Option<i32>).
//   - `insert_front(v, x)`:  put x at index 0.
//   - `swap_remove_at(v,i)`: remove element at i with swap_remove and return it.
//   - `keep_positives(v)`:   drop every non-positive value (use `retain`).
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn push_zero(v: &mut Vec<i32>) {
    v.???(0);
}

fn pop_last(v: &mut Vec<i32>) -> Option<i32> {
    v.???()
}

fn insert_front(v: &mut Vec<i32>, x: i32) {
    v.???(0, x);
}

fn swap_remove_at(v: &mut Vec<i32>, i: usize) -> i32 {
    v.???(i)
}

fn keep_positives(v: &mut Vec<i32>) {
    v.retain(|x| *x ??? 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn push_works() {
        let mut v = vec![1, 2];
        push_zero(&mut v);
        assert_eq!(v, vec![1, 2, 0]);
    }
    #[test] fn pop_some() {
        let mut v = vec![1, 2, 3];
        assert_eq!(pop_last(&mut v), Some(3));
        assert_eq!(v, vec![1, 2]);
    }
    #[test] fn pop_none() {
        let mut v: Vec<i32> = vec![];
        assert_eq!(pop_last(&mut v), None);
    }
    #[test] fn insert_front_works() {
        let mut v = vec![2, 3];
        insert_front(&mut v, 1);
        assert_eq!(v, vec![1, 2, 3]);
    }
    #[test] fn swap_remove_works() {
        let mut v = vec![10, 20, 30, 40];
        let taken = swap_remove_at(&mut v, 1); // takes 20, replaces with 40
        assert_eq!(taken, 20);
        assert_eq!(v, vec![10, 40, 30]); // order NOT preserved
    }
    #[test] fn retain_works() {
        let mut v = vec![-1, 0, 1, -2, 3];
        keep_positives(&mut v);
        assert_eq!(v, vec![1, 3]);
    }
}

fn main() {}
