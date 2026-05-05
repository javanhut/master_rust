// =============================================================================
//  map2 — the `entry` API
// =============================================================================
//
// You'll write this pattern thousands of times in real Rust:
//
//     "increment the counter for this key, creating it as 0 if missing"
//
// The naive version does TWO lookups — `get` then `insert`:
//
//     let v = match counts.get(&k) {
//         Some(n) => *n + 1,
//         None    => 1,
//     };
//     counts.insert(k, v);
//
// The idiomatic version does ONE lookup using the entry API:
//
//     *counts.entry(k).or_insert(0) += 1;
//
// HOW IT WORKS
//
//     m.entry(key)
//         .or_insert(default)        // -> &mut V
//         .or_insert_with(|| build()) // -> &mut V, only builds default if needed
//
// `entry(k)` returns an `Entry<K, V>`, which is one of:
//
//     Entry::Occupied(o)    // key already in the map
//     Entry::Vacant(v)      // key absent
//
// `or_insert(d)` returns `&mut V` — either the existing value or a freshly
// inserted default. You then mutate through that reference.
//
// `or_insert_with(f)` is the same idea but only CALLS `f` when the key is
// missing — useful when constructing the default is expensive.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `bump(counts, key)`:    increment counts[key] by 1, defaulting to 0.
//                             One line. Use `*entry(...).or_insert(0) += 1`.
//   - `add_to_bucket(buckets, key, value)`: append `value` to the Vec stored
//                             at buckets[key], creating an empty Vec if the
//                             key is new. Use `or_insert_with(Vec::new)`.
//
// Do NOT touch the tests.

use std::collections::HashMap;

// I AM NOT DONE

fn bump(counts: &mut HashMap<String, u32>, key: &str) {
    *counts.???(key.to_string()).???(0) += 1;
}

fn add_to_bucket(
    buckets: &mut HashMap<String, Vec<i32>>,
    key: &str,
    value: i32,
) {
    buckets
        .entry(key.to_string())
        .???(Vec::new)
        .push(???);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn bump_new() {
        let mut c: HashMap<String, u32> = HashMap::new();
        bump(&mut c, "a");
        assert_eq!(c.get("a"), Some(&1));
    }
    #[test] fn bump_repeat() {
        let mut c: HashMap<String, u32> = HashMap::new();
        bump(&mut c, "a");
        bump(&mut c, "a");
        bump(&mut c, "b");
        bump(&mut c, "a");
        assert_eq!(c.get("a"), Some(&3));
        assert_eq!(c.get("b"), Some(&1));
        assert_eq!(c.get("c"), None);
    }
    #[test] fn buckets_grow() {
        let mut b: HashMap<String, Vec<i32>> = HashMap::new();
        add_to_bucket(&mut b, "even", 2);
        add_to_bucket(&mut b, "odd",  1);
        add_to_bucket(&mut b, "even", 4);
        add_to_bucket(&mut b, "even", 6);

        assert_eq!(b.get("even"), Some(&vec![2, 4, 6]));
        assert_eq!(b.get("odd"),  Some(&vec![1]));
    }
}

fn main() {}
