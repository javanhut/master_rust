// SOLUTION — iter4_collect

use std::collections::HashMap;
use std::num::ParseIntError;

fn to_vec(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

fn chars_to_string(cs: &[char]) -> String {
    cs.iter().collect()
}

fn pairs_to_map(pairs: &[(String, i32)]) -> HashMap<String, i32> {
    pairs.iter().cloned().collect()
}

fn parse_all(strs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    strs.iter().map(|s| s.parse::<i32>()).collect()
}


// WHY THIS IS OPTIMAL:
//
//   to_vec — `(1..=n)` is itself an iterator (`RangeInclusive<i32>`), so
//   `.collect()` on it just materialises every element. The function's
//   return type fixes the target as `Vec<i32>` — no annotation needed at
//   the call site.
//
//   chars_to_string — `String` implements `FromIterator<char>` and
//   `FromIterator<&char>`, so `cs.iter().collect()` (yielding `&char`) works
//   directly. The function return type chooses `String` for us.
//
//   pairs_to_map — HashMap<K, V> implements `FromIterator<(K, V)>`. We need
//   OWNED `(String, i32)` tuples to feed it; `.iter()` gives `&(String, i32)`
//   so `.cloned()` produces owned copies. (`.copied()` would only work if
//   both fields were Copy — String is not.)
//
//   parse_all — the `Result<Vec<T>, E>` collect trick. Std implements
//   `FromIterator<Result<T, E>> for Result<Vec<T>, E>` with short-circuit
//   semantics: on the first `Err`, collection STOPS and the whole call
//   returns that error. One method call replaces a manual loop with a
//   bunch of `?` operators inside.
//
// ALTERNATIVES:
//
//   Turbofish vs return-type inference — `(1..=n).collect::<Vec<i32>>()` is
//   identical in behaviour. Annotate at the binding when the binding type
//   is what you care about; turbofish when you want the type RIGHT THERE
//   on the method call.
//
//   `.into_iter()` instead of `.iter().cloned()` — for `pairs_to_map`, if
//   you can take `pairs` BY VALUE (`Vec<(String, i32)>`), then
//   `pairs.into_iter().collect()` skips the clones. We borrow here because
//   the signature borrows.
//
//   For `parse_all`, if you wanted to KEEP the partial successes and a
//   list of errors, `.partition`/`.partition_map` (the latter from `itertools`)
//   are the right tools — `collect::<Result<...>>` is strictly all-or-nothing.
