// =============================================================================
//  proj4_parse — basic counts via iterators
// =============================================================================
//
// Now we put `Stats` to work. Implement:
//
//     fn count_basic(text: &str) -> Stats
//
// returning the line / word / char counts as a single `Stats`. Three
// iterator one-liners do the job:
//
//     text.lines().count()              -> usize
//     text.split_whitespace().count()   -> usize
//     text.chars().count()              -> usize
//
// THE IMPORTANT SUBTLETIES
//
//   - `text.lines()` does NOT count a trailing-empty line. "a\nb\n"
//     yields just ["a", "b"] — two lines.  This matches POSIX `wc -l`'s
//     behaviour of "number of LFs", because every non-empty line in a
//     well-formed file ends with one.
//
//   - `text.split_whitespace()` treats any run of unicode whitespace as
//     one separator and skips leading/trailing whitespace. It's almost
//     always what you want. (`text.split(' ')` would NOT skip empty
//     strings — `"a  b"` would yield ["a", "", "b"]. Different beast.)
//
//   - `text.chars()` counts UNICODE SCALAR VALUES, not bytes. `text.len()`
//     would give bytes. For "héllo" .chars().count() == 5, .len() == 6.
//     For our purposes (a `wc`-like tool) chars is the friendlier number.
//
// CONCEPTS IN PLAY
//
//   - chapter 7 iterators: every count here is .iter-thing.count(). No
//     loops, no allocation, lazy evaluation.
//   - chapter 5 strings: the difference between chars and bytes.
//   - chapter 9 structs: build a Stats with the named-field syntax.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Replace every `???`. Every test must pass without you touching them.

// I AM NOT DONE

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

pub fn count_basic(text: &str) -> Stats {
    Stats {
        lines: text.???().count(),
        words: text.???().count(),
        chars: text.???().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty_input() {
        assert_eq!(count_basic(""), Stats { lines: 0, words: 0, chars: 0 });
    }

    #[test] fn single_word() {
        assert_eq!(count_basic("hello"), Stats { lines: 1, words: 1, chars: 5 });
    }

    #[test] fn one_line_two_words() {
        let s = count_basic("hello world");
        assert_eq!(s.lines, 1);
        assert_eq!(s.words, 2);
        assert_eq!(s.chars, 11);
    }

    #[test] fn three_lines() {
        // .lines() splits on \n; "a\nb\nc" -> ["a","b","c"].
        let s = count_basic("a\nb\nc");
        assert_eq!(s.lines, 3);
        assert_eq!(s.words, 3);
    }

    #[test] fn whitespace_runs_are_one_separator() {
        // split_whitespace collapses runs.
        let s = count_basic("  hi   there\t\tfriend  ");
        assert_eq!(s.words, 3);
    }

    #[test] fn unicode_chars_not_bytes() {
        // "héllo" — five Unicode scalar values, six UTF-8 bytes.
        let s = count_basic("héllo");
        assert_eq!(s.chars, 5);
    }
}

fn main() {}
