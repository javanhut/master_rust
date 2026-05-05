// SOLUTION — map2_entry

use std::collections::HashMap;

fn bump(counts: &mut HashMap<String, u32>, key: &str) {
    *counts.entry(key.to_string()).or_insert(0) += 1;
}

fn add_to_bucket(
    buckets: &mut HashMap<String, Vec<i32>>,
    key: &str,
    value: i32,
) {
    buckets
        .entry(key.to_string())
        .or_insert_with(Vec::new)
        .push(value);
}

// WHY THIS IS OPTIMAL:
//
//   `*counts.entry(k).or_insert(0) += 1` is THE canonical Rust counter
//   pattern. It does exactly one hash lookup. `entry(k)` either finds the
//   bucket or marks the empty slot; `or_insert(0)` writes 0 into the slot if
//   it's empty and returns `&mut V` either way; the leading `*` derefs that
//   reference so `+= 1` mutates the value in place.
//
//   The naive `match counts.get(&k) { ... } ; counts.insert(k, ...)` does
//   TWO hashes and TWO lookups, plus an unnecessary `to_string` clone in
//   the path that already had the key.
//
//   `or_insert_with(Vec::new)` instead of `or_insert(Vec::new())`:
//   `or_insert(...)` evaluates its argument unconditionally — even when the
//   key already exists. For a `Vec::new()` that's just a pointer, no harm.
//   But for any default that allocates or computes (e.g. `vec![0; 1000]`),
//   `or_insert_with` calls the closure only if the key was missing, which
//   matters once defaults aren't free.
//
//   Passing `Vec::new` (not `Vec::new()`) — we pass the FUNCTION itself,
//   and `or_insert_with` calls it. Same as `|| Vec::new()` but shorter.
//
// WHY THE `key.to_string()`?
//
//   The map owns `String` keys, so `entry` needs a `String`. Cloning is the
//   cost of "I might need to insert this key". If you're inserting the same
//   string a billion times and want to avoid the clone on existing keys,
//   look at `raw_entry` (nightly) or `entry_ref` (in `hashbrown`); for normal
//   code, `to_string()` is fine.
//
// PATTERNS YOU'LL USE FOREVER:
//
//   Word counter:        *counts.entry(word).or_insert(0u32) += 1;
//   Group-by:            groups.entry(k).or_insert_with(Vec::new).push(v);
//   Memoise:             *cache.entry(input).or_insert_with(|| compute(input))
