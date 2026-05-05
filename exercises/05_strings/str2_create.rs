// =============================================================================
//  str2 — creating Strings (and why there are five ways to do it)
// =============================================================================
//
// Newcomers hit this within their first hour:
//
//     let a = "hello";                  // &'static str
//     let b = String::from("hello");    // String
//     let c = "hello".to_string();      // String
//     let d = "hello".to_owned();       // String
//     let e: String = "hello".into();   // String
//
// All five end up holding the same 5 bytes "hello". Why does the language
// expose so many spellings? Because each comes from a different TRAIT or
// associated function, and Rust never picks favourites in std.
//
//     String::from(x)    — `From<&str> for String`   — explicit conversion.
//     "x".to_string()    — `ToString` trait          — works on any Display.
//     "x".to_owned()     — `ToOwned` trait           — works on any borrowed
//                                                      type that has an owned
//                                                      counterpart (also slices,
//                                                      Path, OsStr, ...).
//     "x".into()         — calls `Into<String>`      — type-driven (the LHS
//                                                      annotation steers it).
//     String::new()      — empty heap String, no allocation until you push.
//
// Style guidance you'll see in the wild:
//   - Rust style guide & most projects: `String::from("x")` for clarity, or
//     `"x".to_string()` because it's shorter. Both are fine; pick one and be
//     consistent.
//   - `to_owned()` shines in GENERIC code (`T: ToOwned`) — you don't always
//     know you're dealing with a string slice.
//   - `into()` shines when the target type is obvious from context.
//
// `String::with_capacity(n)` pre-allocates room for `n` bytes. Use it when you
// know roughly how much you'll push, to avoid reallocations.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_from`:        use `String::from`.
//   - `make_to_string`:   use `.to_string()`.
//   - `make_to_owned`:    use `.to_owned()`.
//   - `make_empty_with_capacity`: pre-allocate room for at least 64 bytes,
//      DON'T push anything yet. Return the empty String.
//
// All four functions must produce equal-valued Strings for the literal "hi"
// (except `make_empty_with_capacity`, which is empty by design).

// I AM NOT DONE

fn make_from() -> String {
    ???::from("hi")
}

fn make_to_string() -> String {
    "hi".???()
}

fn make_to_owned() -> String {
    "hi".???()
}

fn make_empty_with_capacity() -> String {
    String::???(64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn from_eq()      { assert_eq!(make_from(), "hi"); }
    #[test] fn to_string_eq() { assert_eq!(make_to_string(), "hi"); }
    #[test] fn to_owned_eq()  { assert_eq!(make_to_owned(), "hi"); }

    #[test] fn empty_is_empty() {
        let s = make_empty_with_capacity();
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }
    #[test] fn capacity_reserved() {
        let s = make_empty_with_capacity();
        assert!(s.capacity() >= 64);
    }
}

fn main() {}
