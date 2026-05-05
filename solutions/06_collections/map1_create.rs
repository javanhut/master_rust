// SOLUTION — map1_create

use std::collections::HashMap;

fn make_scores() -> HashMap<String, i32> {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("alice".to_string(), 90);
    m.insert("bob".to_string(), 75);
    m
}

fn score_of(m: &HashMap<String, i32>, name: &str) -> Option<i32> {
    m.get(name).copied()
}

fn has_player(m: &HashMap<String, i32>, name: &str) -> bool {
    m.contains_key(name)
}

fn kick(m: &mut HashMap<String, i32>, name: &str) -> Option<i32> {
    m.remove(name)
}

// WHY THESE ARE OPTIMAL:
//
//   `HashMap::new()` allocates nothing up front, just like `Vec::new()`.
//   The first insert triggers the actual hash-table allocation.
//
//   `m.get(name).copied()` — note the parameter is `&str`, but the keys are
//   `String`. This works because of `Borrow<str> for String`: HashMap's
//   `get` accepts anything that borrows as the key type. So passing `&str`
//   to a `HashMap<String, _>` is zero-cost — no allocation, no `.to_string()`
//   on the lookup side. This is one of HashMap's nicest ergonomic touches.
//
//   `m.contains_key(name)` is the right tool for a yes/no check. If you
//   need both "is it there?" and "what's the value?", just call `get` once
//   and match the Option — `contains_key` followed by `get` is two lookups.
//
//   `m.remove(name)` returns the removed value as `Option<V>`. If you don't
//   care about the old value, you can ignore it: `let _ = m.remove(name);`.
//
// ALTERNATIVES YOU'LL SEE:
//
//   - `HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)])` — build
//     from an array of tuples in one expression.
//   - `m.insert("x".to_string(), 1).is_none()` — `insert` returns the old
//     value, so you can detect "was this a new key?" without a prior lookup.
//
// PERFORMANCE NOTES:
//
//   The default `HashMap` uses SipHash, designed to be DoS-resistant. For
//   trusted, performance-critical code, the `ahash` or `fxhash` crates give
//   2–5x throughput on small keys. Std stays conservative on purpose.
