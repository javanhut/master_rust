// =============================================================================
//  opt1 — Option<T> basics
// =============================================================================
//
// Many languages use `null` to mean "no value". Rust does NOT — `null` is
// notoriously the source of crash-on-deref bugs in every other systems
// language. Instead Rust gives you a regular enum:
//
//     enum Option<T> {
//         Some(T),
//         None,
//     }
//
// `Option<T>` is in the prelude — you don't need to `use` anything. The
// compiler forces you to acknowledge the `None` case before you can touch
// the inner `T`. No surprise null-deref. Ever.
//
// CONSTRUCTING
// ────────────
//     let a: Option<i32> = Some(7);
//     let b: Option<i32> = None;
//
// QUICK QUERIES
// ─────────────
//     a.is_some()   // true
//     a.is_none()   // false
//
// EXTRACTING — the loud ways
// ──────────────────────────
//     a.unwrap()              // returns 7; PANICS if None
//     a.expect("need a num")  // returns 7; PANICS with your message if None
//
// `.unwrap()` and `.expect("...")` say to the reader: "I am sure this is
// `Some` here, and if I'm wrong the program SHOULD crash." Use them in
// tests, prototypes, and after a genuine invariant check. In production
// code you almost always reach for pattern matching or the combinators
// you'll meet two exercises from now.
//
//     // PANIC: called `Option::unwrap()` on a `None` value
//     let x: Option<i32> = None;
//     let n = x.unwrap();          // crashes here
//
// `.expect("msg")` is strictly nicer than `.unwrap()` — when it does panic
// you get a message explaining WHY you thought it was safe.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement four small helpers that practise the four basics:
//
//   - `first_char(s)`            -> Option<char>     (use s.chars().next())
//   - `has_value(opt)`           -> bool             (use is_some)
//   - `force(opt)`               -> i32              (use unwrap)
//   - `force_with_msg(opt)`      -> i32              (use expect)
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn first_char(s: &str) -> Option<char> {
    s.chars().???()
}

fn has_value(opt: Option<i32>) -> bool {
    opt.???()
}

fn force(opt: Option<i32>) -> i32 {
    opt.???()
}

fn force_with_msg(opt: Option<i32>) -> i32 {
    opt.???("expected a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_char_present() { assert_eq!(first_char("hello"), Some('h')); }
    #[test] fn first_char_empty()   { assert_eq!(first_char(""),      None); }

    #[test] fn has_value_some() { assert!( has_value(Some(0))); }
    #[test] fn has_value_none() { assert!(!has_value(None));    }

    #[test] fn force_some() { assert_eq!(force(Some(42)), 42); }

    #[test]
    #[should_panic]
    fn force_none_panics() { let _ = force(None); }

    #[test] fn force_msg_some() { assert_eq!(force_with_msg(Some(7)), 7); }

    #[test]
    #[should_panic(expected = "expected a number")]
    fn force_msg_none_panics() { let _ = force_with_msg(None); }
}

fn main() {}
