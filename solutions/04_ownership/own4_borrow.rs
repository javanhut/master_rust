// SOLUTION — own4_borrow

fn string_length(s: &String) -> usize {
    s.len()
}

fn sum_of_two(a: &i32, b: &i32) -> i32 {
    *a + *b
}

fn count_chars(s: &String) -> usize {
    s.chars().count()
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `&String` is the "I want to look but not take" parameter type. Inside
//   `string_length`, calling `s.len()` works without `*s` because the dot
//   operator auto-dereferences as many times as needed.
//
//   `*a + *b` makes the dereference EXPLICIT so you see what's happening.
//   You could also write `a + b` and rely on `impl Add for &i32`, which the
//   standard library provides — both compile to the same code. The starred
//   form is clearer for a beginner reading along.
//
//   `s.chars().count()` walks the UTF-8 of the String and counts Unicode
//   scalar values. It's O(N) — for ASCII you'd just use `.len()`, but the
//   moment you have multibyte characters those answers diverge:
//
//       "héllo".len()           == 6      (bytes)
//       "héllo".chars().count() == 5      (scalars)
//       "héllo".chars().nth(1)  == Some('é')   ← NOT byte 1
//
// IDIOMATIC: prefer `&str` over `&String`
//
//   In real code these signatures would be:
//
//       fn string_length(s: &str) -> usize  { s.len() }
//       fn count_chars(s: &str) -> usize    { s.chars().count() }
//
//   `&str` is strictly more flexible: it accepts `&String` (via deref
//   coercion: `&String -> &str`) AND string literals like `"hello"` AND
//   substrings. We stick with `&String` here because the chapter is about
//   borrowing in general; `&str` gets its own spotlight in own6.
//
// WHAT YOU CANNOT DO:
//
//   With a `&String`, you cannot push, pop, or otherwise mutate. Try it:
//
//       fn nope(s: &String) { s.push('!'); }
//       // error[E0596]: cannot borrow `*s` as mutable, as it is behind a
//       //              `&` reference
//
//   That's the whole point of `&` — read-only access. `&mut` (next file)
//   is the version that lets you mutate.
