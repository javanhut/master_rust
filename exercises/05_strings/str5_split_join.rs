// =============================================================================
//  str5 — splitting, trimming, and the slice-collect-join idiom
// =============================================================================
//
// Most "string processing" you write in real code is about SPLITTING input
// into pieces, doing something to the pieces, and JOINING the results.
//
// SPLITTERS (all return iterators of `&str` views into the original string —
// no allocation per piece):
//
//     s.split(',')                — split on a single char
//     s.split("--")               — split on a string pattern
//     s.split(char::is_whitespace) — split on any char matching a predicate
//     s.splitn(3, ',')            — at MOST 3 pieces; remainder is the last
//     s.split_whitespace()        — split on runs of any Unicode whitespace,
//                                   AND drop empty leading/trailing pieces
//                                   (this is what you usually want)
//     s.lines()                   — split on \n, also handles \r\n.
//
// TRIMMERS:
//
//     s.trim()             — remove leading & trailing whitespace
//     s.trim_start()       — leading only
//     s.trim_end()         — trailing only
//     s.trim_matches(c)    — strip leading & trailing chars matching c
//
// JOINING:
//
//     ["a","b","c"].join(", ")    // "a, b, c"
//
//     The `join` method is provided by `[T]` where `T: AsRef<str>` (and a
//     handful of similar overloads). Both `&[&str]` and `Vec<String>` work.
//
// THE SLICE → ITERATOR → COLLECT → JOIN IDIOM
//
//     let upper_csv: String = "hello world rust"
//         .split_whitespace()                   // iterator of &str
//         .map(|w| w.to_uppercase())            // iterator of String
//         .collect::<Vec<_>>()                  // Vec<String>
//         .join(", ");                          // String "HELLO, WORLD, RUST"
//
//   Why collect into a Vec first? Because `join` needs to know the total
//   count to preallocate; iterators don't always know their length. There
//   is also `Itertools::join` in the `itertools` crate that skips the Vec.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `word_count`: return the number of whitespace-delimited words.
//                   Treat ANY whitespace, ignore leading/trailing whitespace,
//                   collapse runs of whitespace.
//   - `clean`:      trim leading/trailing whitespace AND return an OWNED
//                   String of the result.
//   - `csv_upper`:  given "a,b,c", return "A,B,C". Use split + map + collect
//                   + join. (Don't worry about empty fields here.)
//   - `nth_line`:   return the N-th line (0-indexed) as Some(&str) or None.

// I AM NOT DONE

fn word_count(s: &str) -> usize {
    s.???().count()
}

fn clean(s: &str) -> String {
    s.???().to_string()
}

fn csv_upper(s: &str) -> String {
    s.???(',')
        .map(|piece| piece.to_uppercase())
        .collect::<Vec<_>>()
        .???(",")
}

fn nth_line(s: &str, n: usize) -> Option<&str> {
    s.???().nth(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn wc_simple()     { assert_eq!(word_count("hello world"),       2); }
    #[test] fn wc_messy()      { assert_eq!(word_count("  one  two\tthree\n"), 3); }
    #[test] fn wc_empty()      { assert_eq!(word_count("   \t\n"),           0); }

    #[test] fn clean_basic()   { assert_eq!(clean("   hi   "),  "hi"); }
    #[test] fn clean_inner()   { assert_eq!(clean("  a b c "),  "a b c"); } // inner ws preserved

    #[test] fn upper_three()   { assert_eq!(csv_upper("a,b,c"), "A,B,C"); }
    #[test] fn upper_one()     { assert_eq!(csv_upper("hi"),    "HI"); }

    #[test] fn nth_first()     { assert_eq!(nth_line("a\nb\nc", 0), Some("a")); }
    #[test] fn nth_middle()    { assert_eq!(nth_line("a\nb\nc", 1), Some("b")); }
    #[test] fn nth_oob()       { assert_eq!(nth_line("a\nb\nc", 9), None); }
    #[test] fn nth_crlf()      { assert_eq!(nth_line("a\r\nb",  1), Some("b")); }
}

fn main() {}
