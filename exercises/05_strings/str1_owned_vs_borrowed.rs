// =============================================================================
//  str1 — owned `String` vs borrowed `&str`
// =============================================================================
//
// Rust has TWO main string types and they are NOT interchangeable. Get this
// distinction straight now and the rest of the chapter falls into place.
//
//     String     — OWNED, heap-allocated, growable. You can push to it, mutate
//                  it, hand it off. Think `Vec<u8>` that's guaranteed UTF-8.
//
//     &str       — a BORROWED VIEW into some existing UTF-8 bytes. Just a
//                  pointer + length. You cannot grow it. You cannot own it.
//                  It must be valid for as long as the data it points at.
//
// Mental picture:
//
//                       ┌──────────────────────┐
//                       │   "hello, world"     │   bytes living somewhere
//                       └──────────────────────┘
//                          ▲              ▲
//                          │              │
//                       String           &str
//                       (owns it)     (looks at it)
//
// STRING LITERALS
//
// A bare double-quoted literal in your source code has type `&'static str`.
// It is baked into your program's read-only data section and lives for the
// whole program (`'static` lifetime). You don't allocate, you don't free.
//
//     let greeting: &'static str = "hello";
//
// WHEN TO USE WHICH
//
//     - Function PARAMETERS:    take `&str` by default. It accepts both a
//                               literal AND a `&String` (auto-deref / deref
//                               coercion). Maximum flexibility, zero cost.
//
//     - Function RETURN /
//       struct FIELD that you
//       need to OWN:            use `String`.
//
//     - Building a string up
//       at runtime:             `String` (you need to mutate / grow).
//
// (Chapter 4 covered ownership formally. Here we just reuse it: `&` = borrow,
//  no `&` = own.)
//
// CONVERSIONS
//
//     String -> &str    : `&s` or `s.as_str()`     (cheap, no copy)
//     &str   -> String  : `s.to_string()` / `String::from(s)` / `s.to_owned()`
//                         (allocates and copies — see str2)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `shout`: take a borrowed string slice and return a new OWNED String
//      that is the uppercase version. Pick the right parameter type.
//   - `literal_type_marker`: return the exact same `&'static str` it
//      receives — proves a literal is `'static`.
//   - `first_word_len`: return the length in BYTES of the first whitespace-
//      delimited word of the input. Take a `&str`. (Hint: split_whitespace.)

// I AM NOT DONE

fn shout(s: ???) -> ??? {
    s.to_uppercase()
}

fn literal_type_marker(s: &'static str) -> &'static str {
    ???
}

fn first_word_len(s: &str) -> usize {
    s.split_whitespace().next().map(|w| w.???()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn shout_literal()  { assert_eq!(shout("hi"), "HI"); }
    #[test] fn shout_string()   {
        let owned = String::from("rust");
        assert_eq!(shout(&owned), "RUST");
    }

    #[test] fn literal_passthrough() {
        assert_eq!(literal_type_marker("static!"), "static!");
    }

    #[test] fn fwl_basic()      { assert_eq!(first_word_len("hello world"), 5); }
    #[test] fn fwl_single()     { assert_eq!(first_word_len("rust"), 4); }
    #[test] fn fwl_empty()      { assert_eq!(first_word_len(""), 0); }
    #[test] fn fwl_leading_ws() { assert_eq!(first_word_len("   hi there"), 2); }
}

fn main() {}
