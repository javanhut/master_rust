// =============================================================================
//  iter7 — `flatten` and `flat_map`
// =============================================================================
//
// Sometimes one element of your iterator IS itself an iterator (or a
// collection that can become one). You want a flat stream of their elements,
// not a stream of nested things.
//
// FLATTEN — for "iterator of iterators"
//
//     let nested = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
//     let flat: Vec<i32> = nested.into_iter().flatten().collect();
//     // [1, 2, 3, 4, 5, 6]
//
//   `.flatten()` requires that each item is itself `IntoIterator`. Vec, &[T],
//   arrays, ranges, even `Option<T>` (treated as an iterator of 0 or 1
//   elements!) — all qualify.
//
//   Bonus party trick:
//
//       vec![Some(1), None, Some(3)].into_iter().flatten().collect::<Vec<_>>();
//       // [1, 3]   — None contributed nothing, Some contributed its inner value.
//
// FLAT_MAP — map THEN flatten, in one step
//
//     iter.flat_map(|x| f(x))
//
//   Use it when each input element produces a (possibly empty) GROUP of
//   output elements:
//
//       let words = ["one two", "three"];
//       let all: Vec<&str> = words
//           .iter()
//           .flat_map(|line| line.split_whitespace())
//           .collect();
//       // ["one", "two", "three"]
//
//   That's `map` (line -> iterator of words) followed by `flatten`
//   (concatenate them all), but the chain expresses it in one call.
//
// MENTAL MODEL
//
//   `map`      — one in, one out.
//   `filter`   — one in, zero or one out.
//   `flat_map` — one in, ZERO OR MORE out.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `flatten_vecs(vs)`: take `Vec<Vec<i32>>` (by value), return one flat
//     Vec<i32>. Use `.into_iter().flatten().collect()`.
//   - `keep_somes(opts)`: given `Vec<Option<i32>>`, return a Vec<i32> with
//     all the `None`s stripped out. Same `.into_iter().flatten().collect()`
//     pattern — Option implements IntoIterator!
//   - `words_of(text)`: given a `&str` containing several lines, return
//     every word across all lines as `Vec<&str>`. Use `.lines()` to split
//     into lines, `.flat_map(...)` with `.split_whitespace()` per line.

// I AM NOT DONE

fn flatten_vecs(vs: Vec<Vec<i32>>) -> Vec<i32> {
    vs.into_iter().???().collect()                    // flatten
}

fn keep_somes(opts: Vec<Option<i32>>) -> Vec<i32> {
    opts.into_iter().???().collect()                  // flatten works on Option too
}

fn words_of(text: &str) -> Vec<&str> {
    text.lines().???(|line| line.split_whitespace()).collect()   // flat_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn flatten_basic() {
        let v = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
        assert_eq!(flatten_vecs(v), vec![1, 2, 3, 4, 5, 6]);
    }
    #[test] fn flatten_empties_inside() {
        let v = vec![vec![], vec![1], vec![], vec![2, 3]];
        assert_eq!(flatten_vecs(v), vec![1, 2, 3]);
    }
    #[test] fn flatten_top_empty() {
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten_vecs(v), Vec::<i32>::new());
    }

    #[test] fn somes_basic() {
        assert_eq!(keep_somes(vec![Some(1), None, Some(3), None]), vec![1, 3]);
    }
    #[test] fn somes_all_none() {
        assert_eq!(keep_somes(vec![None, None]), Vec::<i32>::new());
    }

    #[test] fn words_basic() {
        let t = "one two\nthree four five\n\nsix";
        assert_eq!(words_of(t), vec!["one","two","three","four","five","six"]);
    }
    #[test] fn words_empty() {
        assert_eq!(words_of(""), Vec::<&str>::new());
    }
    #[test] fn words_only_whitespace() {
        assert_eq!(words_of("   \n  \t  \n"), Vec::<&str>::new());
    }
}

fn main() {}
