// =============================================================================
//  proj5_logic — top-K most-frequent words
// =============================================================================
//
// `wc` reports counts. The fancy version reports the LEADERBOARD — the
// top-K most-frequent words.
//
//     fn top_words(text: &str, k: usize) -> Vec<(String, u32)>
//
// Spec:
//
//   - Tokenise via `split_whitespace`.
//   - Count occurrences in a `HashMap<String, u32>`.
//   - Sort by (count DESC, word ASC). Ties break ALPHABETICALLY.
//   - Return the first `k` entries.
//   - If `k` is greater than the number of distinct words, return them
//     all. If `k == 0` or text is empty, return an empty Vec.
//
// CONCEPTS IN PLAY
//
//   - chapter 6 collections: HashMap + the entry API.
//   - chapter 7 iterators: `.into_iter()`, `.take(k)`, `.collect()`.
//   - chapter 16 closures: `sort_by(|a, b| ...)` for the comparator.
//   - chapter 5 strings: `to_string()` to lift `&str` keys into owned
//     `String`s — HashMap keys must be owned (or static).
//
// THE SORT COMPARATOR
//
//     v.sort_by(|(wa, ca), (wb, cb)| {
//         cb.cmp(ca)            // counts DESCENDING — note the order
//             .then_with(|| wa.cmp(wb))   // ties: words ASCENDING
//     });
//
// `Ord::cmp` returns an `Ordering`. `.then_with` chains a secondary
// comparator that's only consulted when the primary returns `Equal`.
// The `cb.cmp(ca)` (b before a) is how you flip a sort to descending
// without a separate `reverse()` call.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`.  Don't touch the tests.

// I AM NOT DONE

use std::collections::HashMap;

pub fn top_words(text: &str, k: usize) -> Vec<(String, u32)> {
    // 1. Build the frequency map.
    let mut counts: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_string()).or_insert(???) += 1;
    }

    // 2. Move it into a Vec we can sort.
    let mut v: Vec<(String, u32)> = counts.into_iter().collect();

    // 3. Sort: counts DESC, words ASC for ties.
    v.sort_by(|(wa, ca), (wb, cb)| {
        cb.cmp(ca).then_with(|| wa.???(wb))
    });

    // 4. Truncate to k.
    v.into_iter().???(k).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty_text() {
        assert!(top_words("", 5).is_empty());
    }

    #[test] fn k_zero() {
        assert!(top_words("a a b", 0).is_empty());
    }

    #[test] fn single_word() {
        assert_eq!(top_words("hi hi hi", 5), vec![("hi".to_string(), 3)]);
    }

    #[test] fn ranks_by_count() {
        let v = top_words("a b c a b a", 3);
        assert_eq!(v, vec![
            ("a".to_string(), 3),
            ("b".to_string(), 2),
            ("c".to_string(), 1),
        ]);
    }

    #[test] fn alphabetical_tiebreak() {
        // "b" and "a" both have count 2 -> alphabetical: a before b.
        let v = top_words("b b a a c", 3);
        assert_eq!(v[0], ("a".to_string(), 2));
        assert_eq!(v[1], ("b".to_string(), 2));
        assert_eq!(v[2], ("c".to_string(), 1));
    }

    #[test] fn k_larger_than_distinct() {
        let v = top_words("a b", 100);
        assert_eq!(v.len(), 2);
    }

    #[test] fn k_truncates() {
        let v = top_words("a a a b b c c c c", 2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], ("c".to_string(), 4));
        assert_eq!(v[1], ("a".to_string(), 3));
    }
}

fn main() {}
