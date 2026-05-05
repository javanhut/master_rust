// =============================================================================
//  str4 — iterating: chars, bytes, char_indices (and why `s[0]` is forbidden)
// =============================================================================
//
// THE BIG IDEA: a Rust string is UTF-8. UTF-8 is a VARIABLE-WIDTH encoding —
// one Unicode scalar value can occupy 1, 2, 3, or 4 bytes:
//
//     'a'   -> 1 byte  (0x61)
//     'é'   -> 2 bytes (0xc3 0xa9)
//     '€'   -> 3 bytes (0xe2 0x82 0xac)
//     '🦀'  -> 4 bytes (0xf0 0x9f 0xa6 0x80)
//
// So `s[0]` is ambiguous: do you mean byte 0, or character 0? Those are not
// the same thing. Rust refuses to guess and DOES NOT IMPLEMENT `Index<usize>`
// for `str`. You'll see:
//
//     "héllo"[0]    // ❌ compile error: cannot index into str
//
// You must EXPLICITLY pick what you want:
//
//     s.chars()         — iterator of `char` (Unicode scalar values).
//                         O(n) to walk; O(1) per step. NO random access.
//
//     s.bytes()         — iterator of `u8` (raw UTF-8 bytes).
//                         O(1) random access via `s.as_bytes()[i]`.
//
//     s.char_indices()  — iterator of (byte_offset, char). The byte offset
//                         is the index into the underlying bytes where this
//                         char starts. You'll need this for slicing on
//                         char boundaries:  &s[start..end].
//
//     s.len()           — LENGTH IN BYTES. Always. Constant time.
//     s.chars().count() — number of chars. Linear scan; there's no shortcut.
//
// SLICING SAFELY
//
//     &s[a..b]          — byte indices, must land on UTF-8 boundaries or
//                         the program PANICS at runtime. Use .char_indices()
//                         to discover valid boundaries.
//
// "Character" gets fuzzy at the user-perceived level (a flag emoji is many
// chars; "é" can be one char or "e + combining accent"). Rust's `char` is a
// UNICODE SCALAR — `unicode-segmentation` crate handles graphemes. Here we
// stick to chars and bytes.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `byte_len`:   return the number of BYTES.
//   - `char_count`: return the number of CHARS.
//   - `count_a`:    count occurrences of the char 'a' (case-sensitive).
//                    Must use .chars().
//   - `first_char_byte_offset`: given a string and an N, return the byte
//      offset where the N-th char begins (0-indexed). Return None if the
//      string has fewer than N+1 chars. Use .char_indices().

// I AM NOT DONE

fn byte_len(s: &str) -> usize {
    s.???()
}

fn char_count(s: &str) -> usize {
    s.chars().???()
}

fn count_a(s: &str) -> usize {
    let mut n = 0;
    for c in s.???() {
        if c == 'a' {
            n += 1;
        }
    }
    n
}

fn first_char_byte_offset(s: &str, n: usize) -> Option<usize> {
    s.???().nth(n).map(|(idx, _c)| idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn bytes_ascii()    { assert_eq!(byte_len("hello"), 5); }
    #[test] fn bytes_utf8()     { assert_eq!(byte_len("héllo"), 6); } // é is 2 bytes
    #[test] fn bytes_crab()     { assert_eq!(byte_len("🦀"),    4); }

    #[test] fn chars_ascii()    { assert_eq!(char_count("hello"), 5); }
    #[test] fn chars_utf8()     { assert_eq!(char_count("héllo"), 5); }
    #[test] fn chars_crab()     { assert_eq!(char_count("🦀"),    1); }

    #[test] fn count_a_basic()  { assert_eq!(count_a("banana"),    3); }
    #[test] fn count_a_none()   { assert_eq!(count_a("rust"),      0); }
    #[test] fn count_a_case()   { assert_eq!(count_a("Aaaa"),      3); } // big A doesn't count

    #[test] fn offset_zero()    { assert_eq!(first_char_byte_offset("hello", 0), Some(0)); }
    #[test] fn offset_after_é() {
        // "héllo": h@0, é@1 (2 bytes), l@3, l@4, o@5
        assert_eq!(first_char_byte_offset("héllo", 2), Some(3));
    }
    #[test] fn offset_oob()     { assert_eq!(first_char_byte_offset("hi", 5), None); }
}

fn main() {}
