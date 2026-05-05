// SOLUTION — str_quiz

fn count_words_unique(s: &str) -> usize {
    let mut seen: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        let lower = word.to_lowercase();
        if !seen.contains(&lower) {
            seen.push(lower);
        }
    }
    seen.len()
}

fn reverse_lines(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().collect();
    lines.reverse();
    lines.join("\n")
}

// WHY THIS WORKS:
//
//   count_words_unique
//       split_whitespace gives us trimmed, collapsed word tokens.
//       to_lowercase normalises for case-insensitive comparison and
//       allocates a fresh String per word — cheap for a quiz, and the
//       only correct way for non-ASCII (`'Ä'.to_lowercase()` is `'ä'`).
//       `Vec::contains(&T)` does a linear scan. O(n*m) total. For a 5-word
//       sentence this is irrelevant. For a 1M-word document we'd reach for
//       std::collections::HashSet<String> — but that arrives in a later
//       chapter, and bringing it in now would distract from the lesson.
//
//   reverse_lines
//       `s.lines()` correctly handles both \n and \r\n. We collect to a
//       Vec so we can mutate (reverse), then `join("\n")` rebuilds a
//       single String with newlines BETWEEN — never a trailing one,
//       which matches the spec.
//
// FUNCTIONAL ALTERNATIVE (no mutable Vec):
//
//     fn reverse_lines(s: &str) -> String {
//         s.lines().rev().collect::<Vec<_>>().join("\n")
//     }
//
//   `Iterator::rev()` requires a `DoubleEndedIterator`, which `lines()`
//   is. We still collect-and-join because there's no `Iterator::join`
//   in std (that's an `itertools` thing).
//
// HASHSET VERSION (for when you reach the collections chapter):
//
//     use std::collections::HashSet;
//     fn count_words_unique(s: &str) -> usize {
//         s.split_whitespace()
//          .map(str::to_lowercase)
//          .collect::<HashSet<_>>()
//          .len()
//     }
//
//   Same answer, O(n) average instead of O(n^2), one fewer mutable binding.
