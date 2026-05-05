// =============================================================================
//  lt3 — the three lifetime elision rules
// =============================================================================
//
// In chapter 4 you wrote functions like
//
//     fn first_word(s: &str) -> &str { ... }
//
// and the compiler accepted them without complaint, even though both `s`
// and the return value are references. No `'a` in sight. Why?
//
// LIFETIME ELISION RULES
//
// The compiler runs three rules, in order, on any function or method
// signature where you've omitted lifetimes. If all references end up with
// a lifetime after the rules run, you don't have to write any. If any are
// still missing, the compiler asks you to spell them out.
//
//   RULE 1 — Each input reference gets its OWN, fresh lifetime parameter.
//
//       fn foo(a: &i32, b: &i32, c: &str)
//       // is treated as
//       fn foo<'a, 'b, 'c>(a: &'a i32, b: &'b i32, c: &'c str)
//
//   RULE 2 — If there is EXACTLY ONE input reference, every output
//            reference gets that same lifetime.
//
//       fn foo(s: &str) -> &str
//       // becomes
//       fn foo<'a>(s: &'a str) -> &'a str
//
//   RULE 3 — If there's a `&self` or `&mut self` among the inputs, the
//            lifetime of `self` is given to every output reference. (Methods
//            "borrow from self" by default — usually correct.)
//
//       fn whatever(&self, s: &str) -> &str
//       // becomes
//       fn whatever<'s, 'other>(&'s self, s: &'other str) -> &'s str
//
// If none of the rules can fill in an output reference's lifetime, you get
// the famous error[E0106]: missing lifetime specifier. That's the compiler
// saying "I tried, I can't decide between your inputs — you tell me."
//
// =============================================================================
//  THREE WORKED EXAMPLES — predict before reading the answer
// =============================================================================
//
// Example A:
//
//     fn first_word(s: &str) -> &str { ... }
//
//   • Rule 1 gives `s: &'a str`.
//   • Rule 2 fires: one input ref → output gets `'a`.
//   • Final: `fn first_word<'a>(s: &'a str) -> &'a str`. ✔ elided OK.
//
// Example B:
//
//     fn longest(a: &str, b: &str) -> &str { ... }
//
//   • Rule 1 gives `a: &'a str, b: &'b str` — TWO different lifetimes.
//   • Rule 2 doesn't apply (there are TWO input references, not one).
//   • Rule 3 doesn't apply (no `&self`).
//   • The output reference still has no lifetime. ✘ COMPILE ERROR.
//   You must annotate: `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`.
//
// Example C:
//
//     impl Foo {
//         fn name(&self, default: &str) -> &str { ... }
//     }
//
//   • Rule 1 gives `&'a self, default: &'b str`.
//   • Rule 2 doesn't apply (two input refs).
//   • Rule 3 fires: `&self` is present, so the output lifetime is `'a`.
//   • Final: `fn name<'a, 'b>(&'a self, default: &'b str) -> &'a str`.
//   The compiler chose self over `default`. If you actually wanted
//   `default`'s lifetime, you have to write it yourself.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Below are three signatures, each missing the body. They all rely on
// elision (or fail to). For each one, fill in the body so the file
// compiles cleanly. You should NOT add `<'a>` anywhere — if a signature
// can't be made to compile without one, switch the return type to OWNED
// (`String`) instead. (We're testing your understanding of when elision
// covers you, not how to add `'a`s.)
//
// All three answers below use only string-slice methods you've seen.
//
// This is "compile mode" — passing means the file compiles cleanly.
// There are no tests.

// I AM NOT DONE

// (A) Rule 2 fires — one input ref, output borrows from it. NO `'a` needed.
fn trim_leading(s: &str) -> &str {
    ???
}

// (B) Two input references, output is one of them. Elision FAILS, so we
//     dodge it: change the return type to a fresh OWNED `String`. No
//     references in the output → no lifetime question to answer.
fn pick_owned(a: &str, b: &str) -> ??? {
    if a.len() >= b.len() { ??? } else { ??? }
}

// (C) Rule 3 fires — `&self` present, output gets self's lifetime.
//     NO `'a` needed.
struct Greeter { name: String }

impl Greeter {
    fn name(&self, _fallback: &str) -> &str {
        ???   // return a borrow into `self.name`
    }
}

fn main() {
    let g = Greeter { name: String::from("ada") };
    println!("trim:    [{}]", trim_leading("   hello"));
    println!("pick:    [{}]", pick_owned("rust", "lang"));
    println!("name:    [{}]", g.name("anon"));
}
