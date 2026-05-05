// SOLUTION — str1_owned_vs_borrowed

fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn literal_type_marker(s: &'static str) -> &'static str {
    s
}

fn first_word_len(s: &str) -> usize {
    s.split_whitespace().next().map(|w| w.len()).unwrap_or(0)
}

// WHY THIS IS OPTIMAL:
//
//   `shout(s: &str) -> String`
//       Take by `&str` for MAXIMUM flexibility — callers can pass a literal
//       OR `&some_string` (deref coercion turns `&String` into `&str`).
//       Returning `String` because `to_uppercase` must allocate fresh bytes;
//       there is no way to return a borrow into local data.
//
//   `literal_type_marker(s: &'static str) -> &'static str`
//       The whole point is to prove a string literal really is 'static.
//       Returning `s` compiles ONLY because the input lifetime is 'static.
//       Try changing the input type to `&str` and the body still compiles —
//       Rust would just elide the lifetime to a shorter one.
//
//   `first_word_len`
//       split_whitespace().next() gives Option<&str>. We map to its byte
//       length and unwrap_or(0) for the empty / all-whitespace case.
//
// COMMON BAD ALTERNATIVES:
//
//   fn shout(s: String) -> String  { ... }
//       Forces every caller to give up ownership or clone. A literal must
//       become an allocated String just to call you. Almost always wrong.
//
//   fn shout(s: &String) -> String { ... }
//       The double-indirection antipattern. `&String` is strictly less
//       flexible than `&str` — clippy will warn. Always prefer `&str`.
//
// NOTE ON `len`:
//   `s.len()` on a `&str` returns BYTES, not characters. That's what's
//   under test here (all ASCII), but for "héllo" .len() == 6, not 5.
//   We address this properly in str4.
