// SOLUTION — own_quiz

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn push_all(dst: &mut String, parts: &[&str]) {
    for p in parts {
        dst.push_str(p);
    }
}

fn count_word(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `longest` uses `>=` to satisfy the "ties → return a" rule. The
//   `'a` lifetime tells the compiler: the result borrows from either `a`
//   or `b`, and is valid for as long as both are. Without that annotation
//   the compiler can't elide — there are two input references and the
//   output is one of them, but which? `'a` makes the answer "either is
//   fine, as long as they outlive the result equally".
//
//   `push_all` walks the slice of `&str`. `for p in parts` gives you
//   `p: &&str` — but `push_str` takes `&str` and Rust auto-derefs the
//   extra layer for you. We mutate `dst` through `&mut String`, the
//   exclusive-borrow form from own5.
//
//   `count_word` is a one-liner thanks to the standard library:
//   `str::matches(pattern)` returns an iterator over non-overlapping
//   occurrences of the pattern, and `.count()` consumes it. Note the
//   non-overlap: "aaaa".matches("aa").count() == 2, not 3.
//
// LIFETIME PRIMER (preview of chapter 17):
//
//   - Every reference has a lifetime. Most of the time it's invisible
//     because of LIFETIME ELISION RULES.
//   - Elision rule for fns: if there's exactly ONE input reference, the
//     output reference borrows from it. That's why `first_word(s: &str)
//     -> &str` doesn't need annotation.
//   - With TWO input references and one output reference, elision can't
//     decide. You write `<'a>` and use it on both inputs and the output
//     to say "they all share one lifetime".
//   - You can also have multiple lifetimes: `<'a, 'b>` — useful when the
//     output is genuinely tied to only ONE of the inputs.
//
// ALTERNATIVES YOU'LL SEE:
//
//   - `push_all` could use iterator chaining:
//
//         dst.extend(parts.iter().copied());
//
//     `String` implements `Extend<&str>`, so `extend` directly works.
//
//   - `count_word` rewritten without the helper:
//
//         haystack.split(needle).count().saturating_sub(1)
//
//     Less obvious; prefer `.matches`.
//
// FINAL CHAPTER 4 CHEAT SHEET:
//
//     T              owned (move on assign unless Copy)
//     &T             shared borrow      — many at once, read-only
//     &mut T         exclusive borrow   — exactly one, read-write
//     &str / &[T]    slice              — borrowed view into owned data
//     .clone()       deep copy          — explicit, may be expensive
//
// You now have the core of Rust's memory model. Everything else builds on it.
