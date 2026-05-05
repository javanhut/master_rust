// SOLUTION — opt1_basics

fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn has_value(opt: Option<i32>) -> bool {
    opt.is_some()
}

fn force(opt: Option<i32>) -> i32 {
    opt.unwrap()
}

fn force_with_msg(opt: Option<i32>) -> i32 {
    opt.expect("expected a number")
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `s.chars().next()` is the textbook way to ask "the first char, if
//   any". `.next()` on any iterator returns `Option<Item>` — `None`
//   when the iterator is exhausted, which for an empty string is
//   immediately. We get the right `Option` shape for free.
//
//   `is_some` / `is_none` are zero-cost predicates — they compile down
//   to a tag check on the enum discriminant. Use them whenever you only
//   need the answer to "is there a value?" and not the value itself.
//
//   `unwrap` is the blunt instrument: extracts the `T` from `Some(T)`,
//   PANICS on `None`. The panic message is a generic
//   "called `Option::unwrap()` on a `None` value" — fine for tiny
//   scripts and tests, useless for diagnosing a real crash a user hit.
//
//   `expect("msg")` is `unwrap` with a custom panic message. The rule
//   of thumb: if you ever feel like writing `.unwrap()` in real code,
//   write `.expect("why I think this is Some")` instead. Future-you
//   reading the panic in a logfile will thank present-you.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   first_char via `s.chars().nth(0)`
//     - Works, but `.next()` is more direct and slightly cheaper —
//       `.nth(n)` does an internal `for _ in 0..n` skip; for n=0 that's
//       no skips, but `.next()` is the clearer intent.
//
//   first_char via `s.bytes().next().map(|b| b as char)`
//     - WRONG for non-ASCII. UTF-8 multi-byte chars need decoding;
//       `.chars()` does that, `.bytes()` does not.
//
//   has_value via `match opt { Some(_) => true, None => false }`
//     - Functionally identical, three lines instead of one. `is_some`
//       exists precisely so you don't write that.
//
// SUBTLETY:
//   `.unwrap()` and `.expect()` MOVE the inner value out of the `Option`.
//   On `Option<&T>` they hand back a `&T`; on `Option<String>` they
//   hand back the `String` and consume the original `Option`. If you
//   need the value AND the original `Option`, reach for `.as_ref()`
//   first (you'll meet it later).
