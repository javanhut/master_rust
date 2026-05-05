// SOLUTION — str4_iterate

fn byte_len(s: &str) -> usize {
    s.len()
}

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn count_a(s: &str) -> usize {
    let mut n = 0;
    for c in s.chars() {
        if c == 'a' {
            n += 1;
        }
    }
    n
}

fn first_char_byte_offset(s: &str, n: usize) -> Option<usize> {
    s.char_indices().nth(n).map(|(idx, _c)| idx)
}

// WHY THIS IS OPTIMAL:
//
//   byte_len -> s.len()
//       `str::len` is O(1) — it's literally the length field of the slice
//       header. No scanning, no allocation. The "number of CHARACTERS" is
//       not an O(1) thing in UTF-8, hence the deliberate naming.
//
//   char_count -> s.chars().count()
//       MUST scan. There is no faster way; UTF-8 is variable-width. If you
//       need char count repeatedly, cache it.
//
//   count_a — explicit `.chars()` because we are matching a `char`. Using
//       `.bytes()` would also work for the literal ASCII byte b'a', but
//       silently breaks the moment somebody passes 'á' or any non-ASCII.
//       Always reach for `.chars()` when you're thinking about characters.
//
//   first_char_byte_offset -> char_indices().nth(n)
//       `.chars()` alone gives you `char` only — no offset. `char_indices()`
//       gives you (byte_offset, char) pairs. `.nth(n)` walks until the n-th
//       and returns Option, exactly the semantics we want. This is the
//       canonical way to map a "logical char index" to a safe slicing index.
//
// IDIOMATIC FUNCTIONAL FORM:
//
//     fn count_a(s: &str) -> usize {
//         s.chars().filter(|&c| c == 'a').count()
//     }
//
//   Same machine code in release. The for-loop version is here so the
//   blanks are obvious before you've internalised iterator combinators.
//
// REMINDER:
//   `&s[i..j]` slicing uses BYTE indices. If `i` is not on a char boundary,
//   it panics. That's why `.char_indices()` exists — get a known-good
//   byte offset, then slice with confidence.
