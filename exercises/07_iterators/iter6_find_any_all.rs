// =============================================================================
//  iter6 — short-circuit terminators: find, position, any, all, min/max_by_key
// =============================================================================
//
// Some terminators stop EARLY — as soon as they have their answer. That makes
// them much faster than walking the whole iterator and checking afterwards.
// Most take a closure that returns `bool`.
//
//     .find(|&&x| pred(x))   -> Option<Item>      // first match (or None)
//     .position(|&x| pred(x))-> Option<usize>     // INDEX of first match
//     .any(|&x| pred(x))     -> bool              // does ANY match?
//     .all(|&x| pred(x))     -> bool              // do they ALL match?
//
//   `find` returns the element. `position` returns its index. Both stop at
//   the first hit; on no match they return `None`/`None`.
//
//   `any` returns true at the first true. `all` returns false at the first
//   false. On an EMPTY iterator: `.any(...)` is `false`, `.all(...)` is
//   `true` (vacuous truth). Memorise that pair — it bites people.
//
// REFERENCE GOTCHA — read carefully
//
//   `find` takes the item by REFERENCE (just like `filter`). Closure
//      parameter is `&Self::Item`. Pattern: `|&&x|` over `&i32` items.
//
//   `position`, `any`, `all` take the item BY VALUE. Closure parameter
//      is `Self::Item` directly. Pattern: `|&x|` over `&i32` items.
//
// One fewer `&` for any/all/position. Annoying inconsistency, easy to fix
// when the compiler complains:
//
//     let v = [1, 2, 3, 4];
//     v.iter().find(|&&x| x > 2);    // Some(&3)   — reference closure
//     v.iter().any(|&x|  x == 3);    // true       — value closure
//     v.iter().position(|&x| x == 3);// Some(2)    — value closure
//
// MIN / MAX (and friends with closures)
//
//     iter.min()         iter.max()                // requires Ord
//     iter.min_by_key(|x| f(x))                    // smallest by KEY
//     iter.max_by_key(|x| f(x))                    // largest by KEY
//     iter.min_by(|a, b| a.cmp(b))                 // custom compare
//
//   `_by_key` is what you reach for 90% of the time. The closure maps each
//   item to a comparable key (number, string, tuple…), and the call returns
//   the original item with the chosen key.
//
//   On EMPTY iterators all of these return `None`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `first_negative(xs)`: return the first negative number in `xs` as
//     `Option<i32>`. Use `.find(...)` then `.copied()`.
//   - `index_of(xs, target)`: return the index of the first occurrence of
//     `target`, or `None`. Use `.position`.
//   - `has_zero(xs)`: `true` if `xs` contains a 0. Use `.any`.
//   - `all_positive(xs)`: `true` if every element is > 0. Use `.all`.
//     Empty slice should return `true` (vacuous).
//   - `longest_word(words)`: given `&[&str]`, return the longest as
//     `Option<&str>`. Use `.iter().max_by_key(|w| w.len()).copied()`.

// I AM NOT DONE

fn first_negative(xs: &[i32]) -> Option<i32> {
    xs.iter().???(|&&x| x < 0).???()                 // find then copied
}

fn index_of(xs: &[i32], target: i32) -> Option<usize> {
    xs.iter().???(|&x| x == target)                  // position (NOTE: only one &)
}

fn has_zero(xs: &[i32]) -> bool {
    xs.iter().???(|&x| x == 0)                       // any  (takes Item by value!)
}

fn all_positive(xs: &[i32]) -> bool {
    xs.iter().???(|&x| x > 0)                        // all  (takes Item by value!)
}

fn longest_word<'a>(words: &'a [&'a str]) -> Option<&'a str> {
    words.iter().???(|w| w.len()).???()              // max_by_key then copied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_neg_found()   { assert_eq!(first_negative(&[3, 2, -1, -5]), Some(-1)); }
    #[test] fn first_neg_missing() { assert_eq!(first_negative(&[1, 2, 3]),       None);    }
    #[test] fn first_neg_empty()   { assert_eq!(first_negative(&[]),              None);    }

    #[test] fn idx_found()    { assert_eq!(index_of(&[10, 20, 30, 20], 20), Some(1)); }
    #[test] fn idx_missing()  { assert_eq!(index_of(&[10, 20, 30],      99), None);    }

    #[test] fn has_zero_yes() { assert!( has_zero(&[1, 2, 0, 3])); }
    #[test] fn has_zero_no()  { assert!(!has_zero(&[1, 2, 3]));    }
    #[test] fn has_zero_empty() { assert!(!has_zero(&[]));         }

    #[test] fn all_pos_yes() { assert!( all_positive(&[1, 2, 3])); }
    #[test] fn all_pos_no()  { assert!(!all_positive(&[1, -2, 3])); }
    #[test] fn all_pos_empty() { assert!( all_positive(&[]));     }     // vacuous

    #[test] fn longest_basic() {
        assert_eq!(longest_word(&["hi", "rusty", "go"]), Some("rusty"));
    }
    #[test] fn longest_empty() {
        let v: &[&str] = &[];
        assert_eq!(longest_word(v), None);
    }
}

fn main() {}
