// =============================================================================
//  str_quiz — capstone: word counter & line reverser
// =============================================================================
//
// You've now seen the major string operations: ownership, creation,
// concatenation, iteration, splitting/joining, and parsing. Time to pull
// them together.
//
// You will implement TWO functions on a `&str`:
//
//   1. `count_words_unique(s)` — return the number of DISTINCT
//      whitespace-delimited words, comparing case-INSENSITIVELY.
//      "The cat sat on THE mat" -> 5 distinct: {the, cat, sat, on, mat}.
//      No external crates. A simple Vec-based "have I seen this?" check
//      is fine — we have not covered HashSet yet (it shows up in the
//      collections chapter).
//
//   2. `reverse_lines(s)` — return a String containing the same lines as
//      `s` but in REVERSE order, separated by '\n', with NO trailing '\n'.
//      "a\nb\nc"  -> "c\nb\na"
//      ""         -> ""
//
// STRATEGIES YOU SHOULD REACH FOR
//
//   - `.split_whitespace()` for words.
//   - `.to_lowercase()` for case-insensitive comparison. NOTE: returns
//     a `String` per call.
//   - `.lines()` then collect into `Vec<&str>` so you can reverse it.
//   - `Vec::join("\n")` for the final assembly.
//   - `vec.contains(&value)` for "have I seen it?" — O(n²) overall but
//      perfectly fine for an exercise.
//
// Keep the focus on STRING OPERATIONS — no heroic algorithms required.

// I AM NOT DONE

fn count_words_unique(s: &str) -> usize {
    let mut seen: Vec<String> = Vec::new();
    for word in s.???() {
        let lower = word.???();
        if !seen.???(&lower) {
            seen.push(lower);
        }
    }
    seen.len()
}

fn reverse_lines(s: &str) -> String {
    let mut lines: Vec<&str> = s.???().collect();
    lines.???();
    lines.???("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----- count_words_unique -----
    #[test] fn cwu_empty()        { assert_eq!(count_words_unique(""),                    0); }
    #[test] fn cwu_whitespace()   { assert_eq!(count_words_unique("   \t\n"),             0); }
    #[test] fn cwu_simple()       { assert_eq!(count_words_unique("a b c"),               3); }
    #[test] fn cwu_dupes()        { assert_eq!(count_words_unique("a a a"),               1); }
    #[test] fn cwu_case()         { assert_eq!(count_words_unique("The cat sat on THE mat"), 5); }
    #[test] fn cwu_messy_ws()     { assert_eq!(count_words_unique("  hi   HI\thi  "),     1); }

    // ----- reverse_lines -----
    #[test] fn rl_empty()         { assert_eq!(reverse_lines(""),         ""); }
    #[test] fn rl_one()           { assert_eq!(reverse_lines("only"),     "only"); }
    #[test] fn rl_three()         { assert_eq!(reverse_lines("a\nb\nc"),  "c\nb\na"); }
    #[test] fn rl_trailing_nl() {
        // "a\nb\n" — `lines()` yields "a","b" (no trailing empty)
        assert_eq!(reverse_lines("a\nb\n"), "b\na");
    }
}

fn main() {}
