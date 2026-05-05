// SOLUTION — types3_bool_char

fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn digit_value(c: char) -> Option<u32> {
    c.to_digit(10)
}

// WHY THIS IS OPTIMAL:
//
//   to_ascii_lowercase + matches!(...) — collapses the 10 cases (a, e, i,
//   o, u and their uppercase versions) to 5 by normalising case first.
//   The `matches!` macro is the idiomatic way to express "value matches
//   one of these patterns?" as a `bool`. It expands to:
//
//       match c {
//           'a' | 'e' | 'i' | 'o' | 'u' => true,
//           _ => false,
//       }
//
//   `to_digit(10)` does exactly the right thing and returns Option<u32>.
//   `radix = 10` is decimal; you can pass 16 for hex, 2 for binary, etc.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   "aeiouAEIOU".contains(c)
//     - Works, but allocates nothing (good) yet does a linear scan over
//       a string and is conceptually heavier than `matches!`.
//
//   if c == 'a' || c == 'e' || ... { true } else { false }
//     - Verbose; the `if cond { true } else { false }` shape is a code
//       smell — just write `cond`.
//
// SUBTLETY:
//   `is_vowel` matches ONLY ASCII vowels. 'á' (a-acute) returns false.
//   For full Unicode awareness you'd use a Unicode-segmentation library
//   or be explicit about which alphabet you mean. The tests only check
//   ASCII so we keep it simple.
