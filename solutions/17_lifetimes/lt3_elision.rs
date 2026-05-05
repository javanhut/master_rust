// SOLUTION — lt3_elision

fn trim_leading(s: &str) -> &str {
    s.trim_start()
}

fn pick_owned(a: &str, b: &str) -> String {
    if a.len() >= b.len() { a.to_string() } else { b.to_string() }
}

struct Greeter { name: String }

impl Greeter {
    fn name(&self, _fallback: &str) -> &str {
        &self.name
    }
}

fn main() {
    let g = Greeter { name: String::from("ada") };
    println!("trim:    [{}]", trim_leading("   hello"));
    println!("pick:    [{}]", pick_owned("rust", "lang"));
    println!("name:    [{}]", g.name("anon"));
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   Each function exercises a different rule:
//
//   (A) `trim_leading(s: &str) -> &str`
//       One input reference, one output reference. Rule 2 fires: the
//       output's lifetime is set to `s`'s. `str::trim_start` itself has
//       the signature `fn trim_start(&self) -> &str` — same elision
//       pattern, just on `&self`. The result is a sub-slice of `s`, so
//       borrowing-from-`s` is exactly right.
//
//   (B) `pick_owned(a: &str, b: &str) -> String`
//       Two input references would normally wedge the compiler — Rule 2
//       can't pick a single source for the output. Instead of annotating
//       with `<'a>`, we change the strategy: return an OWNED `String`,
//       which has no lifetime in its type. The cost is one allocation;
//       the gain is a signature with no lifetime obligations at all.
//
//       This is a real engineering choice. If you find yourself fighting
//       lifetimes on a function that returns "either of two inputs",
//       returning `String` (or `Cow<'a, str>`) is often the cleanest fix.
//
//   (C) `Greeter::name(&self, _fallback: &str) -> &str`
//       Rule 3 fires: `&self` is present, so the output reference is
//       assumed to borrow from `self`. We deliver on that assumption by
//       returning `&self.name`. The `_fallback` parameter is a red
//       herring — Rule 3 ignores it.
//
//       This is why getter methods on structs almost never need explicit
//       lifetimes: the elision rules already say "the returned reference
//       borrows from `self`," which is what you want.
//
// THE BIG PICTURE:
//
//   - Single input ref → elision works, no annotation.
//   - Method with `&self` returning a ref → elision works, no annotation.
//   - Multiple input refs returning a ref → elision FAILS; either
//     annotate explicitly (lt2-style) or return an owned value (this
//     exercise's Plan B).
//
// ALTERNATIVES:
//
//   1. (B) annotated explicitly:
//
//          fn pick<'a>(a: &'a str, b: &'a str) -> &'a str {
//              if a.len() >= b.len() { a } else { b }
//          }
//
//      Equivalent to lt2's `longer_of`. Allocates nothing. Use this if
//      the caller controls the inputs' lifetimes and just wants a view.
//
//   2. (A) the fully-explicit form:
//
//          fn trim_leading<'a>(s: &'a str) -> &'a str { s.trim_start() }
//
//      Identical behavior; elision saved you 4 keystrokes. Most Rust
//      code in the wild leans on the elision rules — they're not a
//      hidden trick, they're standard style.
