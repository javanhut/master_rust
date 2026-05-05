// =============================================================================
//  iter1 — what an Iterator actually IS
// =============================================================================
//
// In chapter 6 you wrote `for x in &v { ... }` without thinking too hard about
// it. Time to look behind the curtain.
//
// THE TRAIT
//
// `Iterator` is a trait. The whole thing fits in five lines:
//
//     trait Iterator {
//         type Item;
//         fn next(&mut self) -> Option<Self::Item>;
//         // (lots of provided methods built on top of `next`)
//     }
//
// That's it. An iterator is anything you can ask for the NEXT element,
// returning `Some(item)` while there's more, then `None` once it's exhausted.
// Every fancy combinator — `map`, `filter`, `sum`, `collect` — is built on
// top of `next` and provided for free.
//
// `for` IS JUST SUGAR
//
// This loop:
//
//     for x in iter { body(x); }
//
// desugars (essentially) to:
//
//     let mut it = iter.into_iter();
//     loop {
//         match it.next() {
//             Some(x) => { body(x); }
//             None    => break,
//         }
//     }
//
// Once you see `for` as "call `next` until `None`", iterators stop feeling
// magical.
//
// GETTING AN ITERATOR
//
// On a slice `&[T]` (or `&Vec<T>`):
//
//     v.iter()       // yields &T
//     v.iter_mut()   // yields &mut T
//     v.into_iter()  // yields T  (consumes)
//
// And ranges are iterators directly:
//
//     (0..5)         // yields 0, 1, 2, 3, 4
//     (0..=5)        // yields 0, 1, 2, 3, 4, 5
//
// CALLING `next` MANUALLY
//
//     let v = [10, 20];
//     let mut it = v.iter();
//     assert_eq!(it.next(), Some(&10));
//     assert_eq!(it.next(), Some(&20));
//     assert_eq!(it.next(), None);
//
// Two things to notice:
//   - `it` must be `mut` — calling `next` mutates internal state.
//   - `v.iter()` yields `&T`, so the items are `Some(&10)`, not `Some(10)`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `first_two(xs)`: return the first two elements as a tuple
//     `(Option<i32>, Option<i32>)` by calling `.next()` MANUALLY twice on the
//     slice's iterator. Use `.copied()` (or deref) so the tuple holds `i32`,
//     not `&i32`. Hint: chain `.iter().copied()` then call `.next()` twice.
//   - `count_with_next(xs)`: count the elements of `xs` by spinning a manual
//     `loop { match it.next() { ... } }`. No `for`, no `.count()`.

// I AM NOT DONE

fn first_two(xs: &[i32]) -> (Option<i32>, Option<i32>) {
    let mut it = xs.iter().???;        // turn &i32 items into i32 items
    let a = it.???;                    // first call to next
    let b = it.???;                    // second call to next
    (a, b)
}

fn count_with_next(xs: &[i32]) -> usize {
    let mut it = xs.iter();
    let mut n = 0usize;
    loop {
        match it.???() {
            ???(_) => n += 1,
            ??? => break,
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_two_full()  { assert_eq!(first_two(&[10, 20, 30]), (Some(10), Some(20))); }
    #[test] fn first_two_one()   { assert_eq!(first_two(&[7]),          (Some(7), None));      }
    #[test] fn first_two_empty() { assert_eq!(first_two(&[]),           (None, None));         }

    #[test] fn count_basic() { assert_eq!(count_with_next(&[1, 2, 3, 4]), 4); }
    #[test] fn count_empty() { assert_eq!(count_with_next(&[]),           0); }
    #[test] fn count_one()   { assert_eq!(count_with_next(&[42]),         1); }
}

fn main() {}
