// SOLUTION — collections_quiz

use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, u32> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *m.entry(w.to_string()).or_insert(0) += 1;
    }
    m
}

fn most_common(text: &str) -> Option<(String, u32)> {
    let m = word_count(text);
    let mut best: Option<(String, u32)> = None;
    for (k, v) in &m {
        match &best {
            None => best = Some((k.clone(), *v)),
            Some((_, bv)) if v > bv => best = Some((k.clone(), *v)),
            _ => {}
        }
    }
    best
}

// WHY THIS IS OPTIMAL (for the chapter-6 toolset):
//
//   `text.split_whitespace()` collapses runs of any Unicode whitespace into
//   a single delimiter and yields the in-between `&str` slices. It's the
//   right primitive for "tokenise prose by spaces" — better than `.split(' ')`
//   which would yield empty `""` for double spaces.
//
//   `*m.entry(w.to_string()).or_insert(0) += 1` — the canonical counter
//   pattern from map2. One hash, one lookup, one increment. We `to_string()`
//   the borrowed `&str` because the map owns `String` keys.
//
//   Iterating `&m` yields `(&String, &u32)`. We dereference `*v` to copy the
//   count out (u32 is Copy) and `.clone()` the String when we want to own
//   it for the result tuple.  The `match &best { ... Some((_, bv)) if v > bv ... }`
//   is a borrow-friendly way to peek at the running best without moving it.
//
// ITERATOR PREVIEW (chapter 7):
//
//     fn word_count(text: &str) -> HashMap<String, u32> {
//         let mut m = HashMap::new();
//         text.split_whitespace().for_each(|w| {
//             *m.entry(w.to_string()).or_insert(0) += 1;
//         });
//         m
//     }
//
//     fn most_common(text: &str) -> Option<(String, u32)> {
//         word_count(text)
//             .into_iter()
//             .max_by_key(|(_, v)| *v)
//     }
//
//   `max_by_key` is the one-liner answer to "find the largest by some key".
//   We're staying with explicit loops here so the entry API and HashMap
//   iteration stay in focus.
//
// REAL-WORLD UPGRADES YOU'LL WANT LATER:
//
//   - Lowercase before counting:   w.to_lowercase()
//   - Strip punctuation:           w.trim_matches(|c: char| !c.is_alphanumeric())
//   - Sort by frequency:           collect into Vec<(String, u32)>, sort_by
//   - Deterministic tie-breaks:    BTreeMap, or sort by (count desc, key asc)
//   - Avoid allocating on existing keys: hashbrown's `entry_ref`
