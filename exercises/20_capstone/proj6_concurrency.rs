// =============================================================================
//  proj6_concurrency — split the work across threads
// =============================================================================
//
// Counting is embarrassingly parallel: each thread counts its own chunk,
// then we sum the partial results. Implement:
//
//     fn count_parallel(chunks: &[&str]) -> Stats
//
// where each `&str` chunk is counted independently and the results are
// merged. The merged total must equal the serial result of concatenating
// the chunks and running `count_basic` over them.
//
// THE PROBLEM `thread::scope` SOLVES
//
//   `std::thread::spawn` requires its closure to be `'static` — it can't
//   borrow stack data. That means you'd have to clone every chunk into
//   an `Arc<String>` or similar.
//
//   `std::thread::scope` (stable since Rust 1.63) lets spawned threads
//   borrow data from the surrounding scope. The compiler guarantees all
//   spawned threads are joined before `scope` returns, so any borrowed
//   reference is valid for the whole duration.
//
//     thread::scope(|s| {
//         let h = s.spawn(|| {
//             // can borrow `chunks[i]` — a &str living on the parent stack
//             count_basic(chunks[i])
//         });
//         let stats: Stats = h.join().unwrap();
//     });
//
//   `s.spawn(...)` returns a `ScopedJoinHandle`. `.join().unwrap()` waits
//   for it and gets the closure's return value.
//
// CONCEPTS IN PLAY
//
//   - chapter 14 concurrency: scoped threads, joining, returning values.
//   - chapter 4 ownership: borrowing slice data into threads safely.
//   - chapter 9 + 10: Stats and the serial-merge pattern.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`. Tests verify correctness against the serial path.

// I AM NOT DONE

use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl Stats {
    pub fn new() -> Self { Stats { lines: 0, words: 0, chars: 0 } }
    pub fn merge(self, other: Stats) -> Stats {
        Stats {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            chars: self.chars + other.chars,
        }
    }
}

pub fn count_basic(text: &str) -> Stats {
    Stats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
    }
}

pub fn count_parallel(chunks: &[&str]) -> Stats {
    thread::???(|s| {
        // Spawn one thread per chunk. Each returns a Stats.
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| s.spawn(move || count_basic(chunk)))
            .collect();

        // Join all and merge.
        let mut total = Stats::new();
        for h in handles {
            total = total.merge(h.???().unwrap());
        }
        total
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty_slice_is_zero() {
        assert_eq!(count_parallel(&[]), Stats::new());
    }

    #[test] fn single_chunk_matches_serial() {
        let chunks = ["hello world\nthis is rust"];
        assert_eq!(count_parallel(&chunks), count_basic(chunks[0]));
    }

    #[test] fn many_chunks_sum_correctly() {
        let chunks = ["a b c", "d e", "f"];
        // Serial: 0+0+0 lines? lines() on "a b c" -> 1. "d e" -> 1. "f" -> 1.
        // (None of these contain a newline, but each is one non-empty line.)
        let total = count_parallel(&chunks);
        assert_eq!(total.lines, 3);
        assert_eq!(total.words, 6);
        // chars = 5 + 3 + 1 = 9
        assert_eq!(total.chars, 9);
    }

    #[test] fn merge_associativity() {
        // The order of joining shouldn't matter for sums.
        let chunks = ["foo bar", "baz qux quux", "  spaced  out  "];
        let par = count_parallel(&chunks);
        let ser = chunks.iter().map(|c| count_basic(c)).fold(Stats::new(), Stats::merge);
        assert_eq!(par, ser);
    }

    #[test] fn many_chunks_dont_panic() {
        // 16 chunks of varying sizes — exercise the join loop.
        let many: Vec<&str> = (0..16).map(|i| if i % 2 == 0 { "alpha beta" } else { "gamma" }).collect();
        let s = count_parallel(&many);
        assert_eq!(s.words, 8 * 2 + 8 * 1);
    }
}

fn main() {}
