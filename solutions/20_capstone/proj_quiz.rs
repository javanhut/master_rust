// SOLUTION — proj_quiz

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

pub fn summarise(text: &str, k: usize) -> Summary {
    Summary {
        stats: count_basic(text),
        top:   top_words(text, k),
    }
}

// WHY THIS IS OPTIMAL:
//
//   `summarise` is a thin orchestration function — it does no work
//   itself, just calls the two pure pieces and packs them into a
//   `Summary`. That's exactly the job of an integration layer:
//
//     - keep computation in leaf functions you can test in isolation,
//     - keep the caller-facing entry point trivial,
//     - put the data shape (Summary) where everyone agrees on it.
//
//   `Summary` derives `PartialEq + Eq` so tests can compare whole
//   summaries with `assert_eq!`. It does NOT derive `Copy` (the inner
//   `Vec<String,u32>` is heap-owned and not Copy) — but it derives
//   nothing it can't, which is the right discipline.
//
//   We make TWO passes over the text — one in `count_basic`, one
//   in `top_words`. For huge inputs you'd fuse them. For our scale
//   the clarity is worth more than the saved cycles.
//
// ALTERNATIVES:
//
//   1. Single-pass fused version:
//
//          let mut stats = Stats::new();
//          let mut counts: HashMap<String, u32> = HashMap::new();
//          for line in text.lines() {
//              stats.lines += 1;
//              for w in line.split_whitespace() {
//                  stats.words += 1;
//                  *counts.entry(w.to_string()).or_insert(0) += 1;
//              }
//          }
//          stats.chars = text.chars().count();   // still a second pass
//
//      Faster, harder to read, harder to test. Worth it on `wc`-of-a-
//      gigabyte-file scale; not worth it here.
//
//   2. Add a `Display` impl on `Summary` so `println!("{summary}")`
//      pretty-prints the whole report. That's chapter 11's reward —
//      types that format themselves.
//
//   3. Return `Result<Summary, _>` if the underlying functions ever
//      grew failure modes. They don't (every step is a total function
//      on `&str`), so a plain `Summary` is correct.
//
// CONGRATULATIONS — this is the last solution of the course.
//
// You've practised:
//
//   - safe ownership and borrowing
//   - pattern matching, control flow, error handling
//   - the std collections and the iterator family
//   - data modelling with structs, enums, and traits
//   - smart pointers and concurrency primitives
//   - closures, lifetimes, async, macros
//   - and finally — assembling all of it into a real, useful tool.
//
// Open a fresh `cargo new` and build something. Reach for `clap`,
// `serde`, `tokio`, `rayon` when you need them. The standard library
// has carried you this far; the ecosystem is the next mile.
//
// Welcome to Rust.
