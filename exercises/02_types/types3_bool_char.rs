// =============================================================================
//  types3 — bools and chars
// =============================================================================
//
// `bool`
// ─────
// Two values, `true` and `false`. Operators:
//
//     &&   short-circuit AND
//     ||   short-circuit OR
//     !    NOT
//
// Rust does NOT auto-convert numbers to bool. `if 1 { ... }` is a compile
// error. You must say `if x != 0`.
//
// `char`
// ─────
// A `char` is a SINGLE Unicode "scalar value" — 4 bytes, not 1. Single
// quotes:
//
//     let letter: char = 'A';
//     let crab:   char = '🦀';
//
// Note: `'A'` (single quotes) is a `char`, while `"A"` (double quotes) is
// a string slice — `&str`. Different types.
//
// USEFUL `char` METHODS
//
//     c.is_alphabetic()      'A' → true,  '7' → false
//     c.is_alphanumeric()
//     c.is_ascii_digit()
//     c.is_whitespace()
//     c.to_ascii_uppercase()
//     c.to_digit(10)         Option<u32>: '7' → Some(7), 'a' → None
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `is_vowel`: true for any of a/e/i/o/u (uppercase or lowercase).
//   - `digit_value`: convert an ASCII digit char to its integer value.

// I AM NOT DONE

fn is_vowel(c: char) -> bool {
    // HINT: convert to lowercase first with `c.to_ascii_lowercase()`,
    // then `match` against the vowels.
    let c = c.???();
    matches!(c, ???)
}

fn digit_value(c: char) -> Option<u32> {
    // Use the standard `char::to_digit` method with radix 10.
    c.to_digit(???)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn vowels() {
        for c in "aeiouAEIOU".chars() { assert!(is_vowel(c), "{c} should be a vowel"); }
    }

    #[test] fn consonants() {
        for c in "bcdfgxyzBCDFG".chars() { assert!(!is_vowel(c), "{c} is not a vowel"); }
    }

    #[test] fn digits() {
        assert_eq!(digit_value('0'), Some(0));
        assert_eq!(digit_value('7'), Some(7));
        assert_eq!(digit_value('9'), Some(9));
    }

    #[test] fn not_a_digit() {
        assert_eq!(digit_value('a'), None);
        assert_eq!(digit_value('!'), None);
    }
}

fn main() {}
