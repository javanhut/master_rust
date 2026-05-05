// =============================================================================
//  collections_quiz — capstone: word-frequency counter
// =============================================================================
//
// Time to put `Vec`, `HashMap`, and the `entry` API together.
//
// Implement `word_count(text: &str) -> HashMap<String, u32>`:
//
//   - Split `text` on ASCII whitespace.  Use `text.split_whitespace()`,
//     which yields `&str` chunks separated by any whitespace.
//   - For each word, increment its count in the map.  Use the canonical
//     entry pattern: `*m.entry(w.to_string()).or_insert(0) += 1`.
//
// Then implement `most_common(text: &str) -> Option<(String, u32)>`:
//
//   - Build the frequency map.
//   - Walk it once with a `for (k, v) in &m` loop, tracking the best entry
//     seen so far in a mutable `Option<(String, u32)>`.
//   - Return that.  Ties may resolve to either winner — the test handles it.
//
// Keep it explicit `for` loops; iterator chains land in chapter 7.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`.  Do NOT touch the tests.

use std::collections::HashMap;

// I AM NOT DONE

fn word_count(text: &str) -> HashMap<String, u32> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *m.???(w.to_string()).or_insert(???) += 1;
    }
    m
}

fn most_common(text: &str) -> Option<(String, u32)> {
    let m = word_count(text);
    let mut best: Option<(String, u32)> = None;
    for (k, v) in ??? {
        match &best {
            None => best = Some((k.clone(), *v)),
            Some((_, bv)) if v > bv => best = Some((k.clone(), *v)),
            _ => {}
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn count_basic() {
        let m = word_count("the quick brown fox the lazy dog the");
        assert_eq!(m.get("the"),   Some(&3));
        assert_eq!(m.get("quick"), Some(&1));
        assert_eq!(m.get("dog"),   Some(&1));
        assert_eq!(m.get("cat"),   None);
    }
    #[test] fn count_empty() {
        let m = word_count("");
        assert_eq!(m.len(), 0);
    }
    #[test] fn count_whitespace_runs() {
        // split_whitespace collapses runs of any whitespace
        let m = word_count("  hi\thi\nhi   ");
        assert_eq!(m.get("hi"), Some(&3));
        assert_eq!(m.len(), 1);
    }
    #[test] fn most_common_basic() {
        let (w, c) = most_common("a a a b b c").unwrap();
        assert_eq!(w, "a");
        assert_eq!(c, 3);
    }
    #[test] fn most_common_empty() {
        assert!(most_common("").is_none());
    }
    #[test] fn most_common_tie() {
        // Either "a" or "b" is acceptable, both with count 2.
        let (w, c) = most_common("a a b b c").unwrap();
        assert!(w == "a" || w == "b");
        assert_eq!(c, 2);
    }
}

fn main() {}
