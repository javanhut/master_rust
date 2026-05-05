// =============================================================================
//  iter4 — `collect`: from iterator into a real collection
// =============================================================================
//
// `collect` is the universal terminator that materialises an iterator into
// something concrete: a `Vec`, a `String`, a `HashMap`, a `HashSet`, …
//
// Anything implementing the `FromIterator` trait can be the target.
//
// TWO WAYS TO TELL IT WHAT TO BUILD
//
// `collect` is generic over its return type, so YOU must say which type
// you want — there's no default. Two equally idiomatic syntaxes:
//
//     let v: Vec<i32>   = (1..=3).collect();          // annotate the binding
//     let v             = (1..=3).collect::<Vec<i32>>();  // turbofish
//
// The turbofish (`::<...>`) is just generic-argument syntax stuck on the
// method name. Use whichever reads better in context.
//
// WHEN A `_` PLACEHOLDER IS ENOUGH
//
//     let v = (1..=3).collect::<Vec<_>>();
//
// The `_` says "you figure it out": the compiler will fill in `i32` from the
// iterator's item type. Real-world code uses `Vec<_>` constantly.
//
// SOME TARGETS WORTH KNOWING
//
//     Vec<T>        — most common, ordered.
//     String        — collect an iterator of `char`s OR `&str`s into one
//                     String:  ['h','i'].iter().collect::<String>() == "hi"
//     HashMap<K, V> — collect an iterator of `(K, V)` pairs.
//     HashSet<T>    — collect with deduplication.
//
//                     use std::collections::{HashMap, HashSet};
//                     let m: HashMap<&str, i32> = [("a", 1), ("b", 2)]
//                         .into_iter().collect();
//
// THE `Result<Vec<_>, _>` MAGIC
//
// One trick worth memorising. If your iterator yields `Result<T, E>`, you
// can collect into `Result<Vec<T>, E>`:
//
//     let parsed: Result<Vec<i32>, _> =
//         ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
//
// `collect` SHORT-CIRCUITS on the first `Err` — you get `Err(e)` back, not a
// vec full of partial successes. If everything was `Ok`, you get
// `Ok(Vec<T>)`. This idiom replaces a manual loop with a `?`-on-each-item:
//
//     ["1", "oops", "3"].iter().map(|s| s.parse::<i32>())
//                       .collect::<Result<Vec<i32>, _>>()
//     // -> Err(ParseIntError { ... })
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `to_vec(it_range)`: collect `1..=n` into `Vec<i32>`.
//   - `chars_to_string(cs)`: given a slice of `char`, collect into `String`.
//   - `pairs_to_map(pairs)`: given `&[(String, i32)]`, build a HashMap.
//     Use `.iter().cloned().collect()`.
//   - `parse_all(strs)`: parse a slice of `&str` into `Vec<i32>`. Return a
//     `Result<Vec<i32>, std::num::ParseIntError>` and collect once. The whole
//     point is that ONE bad string short-circuits the whole pipeline.

use std::collections::HashMap;
use std::num::ParseIntError;

// I AM NOT DONE

fn to_vec(n: i32) -> Vec<i32> {
    (1..=n).???()                                  // collect the range
}

fn chars_to_string(cs: &[char]) -> String {
    cs.iter().???()                                // collect into String
}

fn pairs_to_map(pairs: &[(String, i32)]) -> HashMap<String, i32> {
    pairs.iter().???().collect()                   // need owned (K, V) tuples
}

fn parse_all(strs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    strs.iter().map(|s| s.parse::<i32>()).???      // ONE collect, into Result<Vec<_>, _>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn vec_basic() { assert_eq!(to_vec(4), vec![1, 2, 3, 4]); }
    #[test] fn vec_zero()  { assert_eq!(to_vec(0), Vec::<i32>::new()); }

    #[test] fn string_basic() {
        assert_eq!(chars_to_string(&['r', 'u', 's', 't']), String::from("rust"));
    }
    #[test] fn string_empty() {
        assert_eq!(chars_to_string(&[]), String::new());
    }

    #[test] fn map_basic() {
        let pairs = vec![("a".to_string(), 1), ("b".to_string(), 2)];
        let m = pairs_to_map(&pairs);
        assert_eq!(m.get("a"), Some(&1));
        assert_eq!(m.get("b"), Some(&2));
        assert_eq!(m.len(), 2);
    }

    #[test] fn parse_ok() {
        assert_eq!(parse_all(&["1", "2", "3"]).unwrap(), vec![1, 2, 3]);
    }
    #[test] fn parse_short_circuits() {
        assert!(parse_all(&["1", "oops", "3"]).is_err());
    }
    #[test] fn parse_empty() {
        assert_eq!(parse_all(&[]).unwrap(), Vec::<i32>::new());
    }
}

fn main() {}
