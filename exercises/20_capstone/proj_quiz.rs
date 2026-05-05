// =============================================================================
//  proj_quiz — final integration test
// =============================================================================
//
// You've built every piece. This last quiz wires `count_basic` and
// `top_words` together behind a single function:
//
//     fn summarise(text: &str, k: usize) -> Summary
//
// where:
//
//     struct Summary {
//         stats: Stats,
//         top:   Vec<(String, u32)>,
//     }
//
// `Summary` is what a real CLI would produce as its output struct — pass
// it to a JSON serialiser, a Display impl, or a TUI renderer.
//
// EVERYTHING YOU'VE LEARNED, ONE LAST TIME
//
//   - chapter 4: take `&str`, return owned `Summary` (clean ownership story).
//   - chapter 5: split_whitespace, lines, chars, to_string.
//   - chapter 6: HashMap + entry API for counting.
//   - chapter 7: iterator chains for sort / take / collect.
//   - chapter 8: total functions — no Result needed because input is text.
//   - chapter 9: Stats and Summary.
//   - chapter 10: implicit when CountUnit was used in proj7.
//   - chapter 11: derive Debug + PartialEq for free assertions.
//   - chapter 16: closures inside sort_by.
//   - chapter 17: &str input outliving the Vec<(String,u32)> output.
//
// You are now a Rust programmer who has written, tested, and integrated
// a real tool. This is the last quiz of the course. Take a breath.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`. Don't touch the tests.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub stats: Stats,
    pub top: Vec<(String, u32)>,
}

pub fn count_basic(text: &str) -> Stats {
    Stats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
    }
}

pub fn top_words(text: &str, k: usize) -> Vec<(String, u32)> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_string()).or_insert(0) += 1;
    }
    let mut v: Vec<(String, u32)> = counts.into_iter().collect();
    v.sort_by(|(wa, ca), (wb, cb)| cb.cmp(ca).then_with(|| wa.cmp(wb)));
    v.into_iter().take(k).collect()
}

/// Run BOTH the basic counts and the top-K analysis in one go.
pub fn summarise(text: &str, k: usize) -> Summary {
    Summary {
        stats: ???(text),
        top:   ???(text, k),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty_input() {
        let s = summarise("", 5);
        assert_eq!(s.stats, Stats { lines: 0, words: 0, chars: 0 });
        assert!(s.top.is_empty());
    }

    #[test] fn small_known_input() {
        let s = summarise("a a b\nc c c", 3);
        assert_eq!(s.stats.lines, 2);
        assert_eq!(s.stats.words, 6);
        assert_eq!(s.stats.chars, 11); // "a a b\nc c c" = 11 chars
        assert_eq!(s.top, vec![
            ("c".to_string(), 3),
            ("a".to_string(), 2),
            ("b".to_string(), 1),
        ]);
    }

    #[test] fn k_zero_keeps_stats() {
        // Stats still produced even when no top-K is requested.
        let s = summarise("hello world", 0);
        assert_eq!(s.stats, Stats { lines: 1, words: 2, chars: 11 });
        assert!(s.top.is_empty());
    }

    #[test] fn k_truncates_top() {
        let s = summarise("a a a b b c", 2);
        assert_eq!(s.top.len(), 2);
        assert_eq!(s.top[0], ("a".to_string(), 3));
        assert_eq!(s.top[1], ("b".to_string(), 2));
    }

    #[test] fn alphabetical_tiebreak_in_summary() {
        let s = summarise("z y x z y x", 3);
        // All three tied at 2; alphabetical: x, y, z.
        assert_eq!(s.top, vec![
            ("x".to_string(), 2),
            ("y".to_string(), 2),
            ("z".to_string(), 2),
        ]);
    }

    #[test] fn full_pipeline_is_total() {
        // No matter the input — even nasty whitespace — summarise returns
        // a Summary, never panics.
        let s = summarise("   \n\n\t  ", 10);
        assert_eq!(s.stats.words, 0);
        assert!(s.top.is_empty());
    }
}

fn main() {}
