// SOLUTION — proj4_parse

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

// WHY THIS IS OPTIMAL:
//
//   Three iterators, three counts, no allocation. Each of `lines`,
//   `split_whitespace`, and `chars` returns an iterator that walks the
//   string in place — no intermediate Vecs, no heap. `.count()` then
//   consumes the iterator, returning the number of items it produced.
//
//   The iteration is sequential and unrelated to the others, so this
//   makes THREE passes over the input. For our test inputs that's
//   nothing. For a 10 GB file it would be visible — proj6 will show how
//   to fold all three counts into a single pass with `for c in
//   text.chars()` and a tiny state machine. Pick the right tool for the
//   right size of input.
//
// ALTERNATIVES:
//
//   1. Single-pass, hand-rolled:
//
//          let mut lines = if text.is_empty() { 0 } else { 1 };
//          let mut words = 0;
//          let mut chars = 0;
//          let mut in_word = false;
//          for c in text.chars() {
//              chars += 1;
//              if c == '\n' { lines += 1; in_word = false; continue; }
//              if c.is_whitespace() {
//                  in_word = false;
//              } else if !in_word {
//                  words += 1;
//                  in_word = true;
//              }
//          }
//
//      This is what real `wc` does. Faster on huge inputs, but fiddlier —
//      note the off-by-one on `lines` for files without a trailing
//      newline (POSIX wc and `str::lines()` disagree subtly here).
//
//   2. `text.bytes().count()` instead of `chars()` — gives BYTES, not
//      characters. POSIX `wc -c` reports bytes; `wc -m` reports
//      characters. Different tools, different conventions.
//
//   3. `text.split(char::is_whitespace).filter(|s| !s.is_empty()).count()`
//      is what `split_whitespace` essentially does internally. Use
//      `split_whitespace` — it's tested and clearer.
//
// KEY TAKEAWAYS:
//
//   - Iterator method chains are how Rust expresses "do this thing to
//     each element". Compose them; the compiler will fuse most of the
//     work into a single tight loop.
//   - `lines()` / `split_whitespace()` / `chars()` form the holy
//     trinity for text analysis on `&str`.
//   - When in doubt about chars-vs-bytes, run a Unicode example
//     through both and look at the numbers.
