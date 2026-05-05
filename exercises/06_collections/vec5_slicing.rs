// =============================================================================
//  vec5 — slicing a `Vec`
// =============================================================================
//
// A SLICE is a borrowed view into a contiguous run of elements. The type is
// `&[T]` (or `&mut [T]`). Internally it's a (pointer, length) pair — no
// ownership, no allocation.
//
//     let v = vec![10, 20, 30, 40, 50];
//     let s: &[i32] = &v[1..4];   // 20, 30, 40 — borrowed view into v
//
// Range syntax in indexing:
//
//     &v[1..4]    // exclusive end:  indices 1, 2, 3
//     &v[1..=3]   // inclusive end:  indices 1, 2, 3
//     &v[..3]     //   open start:  0..3
//     &v[2..]     //   open end  :  2..len
//     &v[..]      //   full slice :  same as &v
//
// As long as the slice exists, the Vec is BORROWED — you can't push to it.
// (Pushing might reallocate and invalidate the slice's pointer; the borrow
// checker won't let you.)
//
// HANDY SLICE METHODS
//
//     s.first()         -> Option<&T>   // first element
//     s.last()          -> Option<&T>   // last element
//     s.len()           -> usize
//     s.chunks(n)       -> iterator over fixed-size sub-slices &[T] of length n
//                          (the LAST chunk may be shorter if len isn't a multiple)
//
// `chunks` is great for "process this Vec in groups of N":
//
//     for c in v.chunks(2) { /* c: &[i32], 1 or 2 elements */ }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `middle(v)`:       return `&v[1..v.len()-1]` as a slice.
//                        (Assume the test only calls it on len >= 2.)
//   - `head(v)`:         return v.first().copied() — Option<i32>.
//   - `tail(v)`:         return v.last().copied()  — Option<i32>.
//   - `chunk_sums(v, n)`:return a Vec<i32> where element i is the sum of
//                        the i-th chunk of size n.
//
// Do NOT touch the tests.

// I AM NOT DONE

fn middle(v: &Vec<i32>) -> &[i32] {
    &v[1..v.len() ??? 1]
}

fn head(v: &Vec<i32>) -> Option<i32> {
    v.???().copied()
}

fn tail(v: &Vec<i32>) -> Option<i32> {
    v.???().copied()
}

fn chunk_sums(v: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for c in v.???(n) {
        let mut s = 0;
        for &x in c {
            s += x;
        }
        out.push(s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn middle_works() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(middle(&v), &[2, 3, 4]);
    }
    #[test] fn head_some() { assert_eq!(head(&vec![7, 8, 9]), Some(7)); }
    #[test] fn head_none() { assert_eq!(head(&vec![]), None); }
    #[test] fn tail_some() { assert_eq!(tail(&vec![7, 8, 9]), Some(9)); }
    #[test] fn tail_none() { assert_eq!(tail(&vec![]), None); }
    #[test] fn chunks_even() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(chunk_sums(&v, 2), vec![3, 7, 11]);
    }
    #[test] fn chunks_uneven() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(chunk_sums(&v, 2), vec![3, 7, 5]); // last chunk is just [5]
    }
}

fn main() {}
