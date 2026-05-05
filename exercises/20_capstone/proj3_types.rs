// =============================================================================
//  proj3_types — domain types
// =============================================================================
//
// Now that we can hand a clean `&str` to the rest of the program, we
// need a place to put the answers. Define the domain:
//
//     struct Stats {
//         lines: usize,
//         words: usize,
//         chars: usize,
//     }
//
//     enum CountUnit { Lines, Words, Chars }
//
// `Stats` accumulates results. `CountUnit` lets the caller say which
// category they're talking about — it'll come back in proj7's CLI flag
// parsing (`--lines` -> CountUnit::Lines).
//
// CONCEPTS IN PLAY
//
//   - chapter 9 structs: a plain data record. We derive Debug + PartialEq
//     so tests can compare values with assert_eq! and print failures.
//   - chapter 10 enums: variants without payloads — a tag-only enum, the
//     simplest enum shape.
//   - chapter 11 traits: deriving works because `usize` already implements
//     Debug + PartialEq + Default.
//
// API YOU IMPLEMENT
//
//     Stats::new()                         -> Self        all-zero stats
//     Stats::add(&mut self, k, n)                         add `n` to category k
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`.  Don't touch the tests.

// I AM NOT DONE

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CountUnit {
    Lines,
    Words,
    Chars,
}

impl Stats {
    pub fn new() -> Self {
        Stats { lines: ???, words: ???, chars: ??? }
    }

    /// Add `n` to whichever category `kind` selects.
    pub fn add(&mut self, kind: CountUnit, n: usize) {
        match kind {
            CountUnit::Lines => self.lines += ???,
            CountUnit::Words => self.??? += n,
            CountUnit::Chars => self.??? += n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn new_is_zero() {
        let s = Stats::new();
        assert_eq!(s, Stats { lines: 0, words: 0, chars: 0 });
    }

    #[test] fn add_lines() {
        let mut s = Stats::new();
        s.add(CountUnit::Lines, 3);
        s.add(CountUnit::Lines, 4);
        assert_eq!(s.lines, 7);
        assert_eq!(s.words, 0);
        assert_eq!(s.chars, 0);
    }

    #[test] fn add_each_category() {
        let mut s = Stats::new();
        s.add(CountUnit::Lines, 1);
        s.add(CountUnit::Words, 2);
        s.add(CountUnit::Chars, 3);
        assert_eq!(s, Stats { lines: 1, words: 2, chars: 3 });
    }

    #[test] fn add_zero_is_noop() {
        let mut s = Stats::new();
        s.add(CountUnit::Words, 0);
        assert_eq!(s, Stats::new());
    }

    #[test] fn unit_is_copy() {
        // CountUnit derives Copy — passing by value is a bit-copy.
        let u = CountUnit::Words;
        let _v = u;     // no move
        let _w = u;     // still usable
    }
}

fn main() {}
