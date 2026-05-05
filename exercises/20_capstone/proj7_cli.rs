// =============================================================================
//  proj7_cli — wire it all together with an argv parser
// =============================================================================
//
// Time to assemble the tool. This file:
//
//   - Defines a sample input as `const SAMPLE: &str` (so we don't depend
//     on stdin and the runner can compile-and-execute deterministically).
//   - Reads `std::env::args()` and parses three flags:
//
//         --lines        report only the line count
//         --words        report only the word count
//         --top N        report the top-N most common words
//
//   - With no flags, prints a full report (lines, words, chars).
//
// The runner executes `main`; exit code 0 ⇒ PASS. We don't pass any args
// to the binary during verification, so the default-no-flags path runs.
//
// THE TINY ARGV PARSER
//
//   `std::env::args()` returns an iterator yielding `String`. The first
//   item is the program name; skip it with `.skip(1)`. We walk the rest
//   manually because `--top` consumes a second token (its value), which
//   is not expressible with simple `.contains(&"--top")`.
//
//     let mut iter = std::env::args().skip(1);
//     while let Some(arg) = iter.next() {
//         match arg.as_str() {
//             "--lines" => only_lines = true,
//             "--words" => only_words = true,
//             "--top"   => top_k = iter.next().and_then(|n| n.parse().ok()).unwrap_or(0),
//             _ => {}     // ignore unknown
//         }
//     }
//
// CONCEPTS REUNITED
//
//   - chapter 7: iterator chains on the args.
//   - chapter 8: `.parse().ok()` to fall back without panicking.
//   - chapter 9 + 10: Stats, CountUnit (could dispatch on it).
//   - chapter 12 modules: we'd split this across files in real life, but
//     here it's one file so you can see the whole assembly at a glance.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`. Make `main` exit 0 on the no-args path so the
// runner reports PASS.

// I AM NOT DONE

use std::collections::HashMap;

const SAMPLE: &str = "the quick brown fox jumps over the lazy dog\n\
                      the fox is quick and the dog is lazy\n\
                      brown brown fox\n";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
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

fn main() {
    // Default: full report. Flags can switch to a category-only mode.
    let mut only_lines = false;
    let mut only_words = false;
    let mut top_k: usize = 0;

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.???() {
            "--lines" => only_lines = true,
            "--words" => only_words = true,
            "--top"   => {
                top_k = iter.next().and_then(|n| n.parse().ok()).???(0);
            }
            _ => {}
        }
    }

    let stats = count_basic(SAMPLE);

    if only_lines {
        println!("{}", stats.lines);
    } else if only_words {
        println!("{}", stats.words);
    } else {
        println!("lines: {}", stats.lines);
        println!("words: {}", stats.words);
        println!("chars: {}", stats.chars);
    }

    if top_k > 0 {
        println!("--- top {top_k} words ---");
        for (w, c) in top_words(SAMPLE, top_k) {
            println!("{c:>4}  {w}");
        }
    }
}
