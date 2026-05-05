// SOLUTION — proj7_cli

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
    let mut only_lines = false;
    let mut only_words = false;
    let mut top_k: usize = 0;

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--lines" => only_lines = true,
            "--words" => only_words = true,
            "--top"   => {
                top_k = iter.next().and_then(|n| n.parse().ok()).unwrap_or(0);
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

// WHY THIS IS OPTIMAL (for a single-file, no-deps tool):
//
//   The argv loop walks the iterator manually so a flag like `--top`
//   can consume the NEXT token as its argument. `.next()` advances the
//   iterator, so calling it again inside the match peels off the
//   value. This is the same shape `clap` produces under the hood for
//   non-positional value flags.
//
//   `arg.as_str()` borrows the `String` to a `&str` so we can match
//   against string literals. Matching on `String` directly works in
//   recent Rust but `&str` is the standard convention.
//
//   `.and_then(|n| n.parse().ok())` is the workhorse "optional fallible
//   extract" pattern: Some -> attempt parse -> Some(T) | None. The
//   final `.unwrap_or(0)` collapses any of "no value", "value didn't
//   parse" into the default. Robust without ceremony.
//
// ALTERNATIVES:
//
//   1. Use the `clap` crate for real CLI tools — it gives derive-based
//      argument structs, help text, validation, the lot. We hand-rolled
//      to keep the course std-only.
//
//   2. `let args: Vec<String> = std::env::args().collect();` and index
//      with `args[i]`. Works, but easy to off-by-one. Iterator + match
//      is friendlier to extend.
//
//   3. Shell-style flag combining (`--lines --top 3`) is supported here
//      because each match arm sets independent flags. Mutual exclusion
//      could be enforced at validation time after the loop.
//
// KEY TAKEAWAYS:
//
//   - `std::env::args()` is your zero-deps argv source.
//   - `iter.next()` inside a `while let Some(arg) = iter.next()` loop
//     lets a flag pull its value from the same iterator.
//   - Combine `Option`'s `.and_then` and `.unwrap_or` to write
//     fallible-but-defaulting parsers in one expression.
//
// THE PROJECT IS BUILT. proj_quiz wraps everything into one summarise()
// call and tests the full pipeline.
