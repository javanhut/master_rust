// =============================================================================
//  lt6 — lifetime subtyping & covariance
// =============================================================================
//
// "Subtyping" sounds like an OOP word, but in Rust it shows up most often
// for LIFETIMES. The rule is short:
//
//   ▶ A REFERENCE THAT LIVES LONGER CAN BE USED WHERE A SHORTER REFERENCE
//     IS REQUIRED.
//
// In symbols: if `'long: 'short` (read "`'long` outlives `'short`"), then
// `&'long T` can be used wherever `&'short T` is expected. We say that
// `&T` is COVARIANT in its lifetime parameter.
//
// THE INTUITION
//
// If a function says "give me a `&'short T`", it's really saying "give me
// a reference that's valid for at least the next `'short` worth of code."
// A reference that's valid for `'static` (forever) trivially satisfies that.
// One that's valid for `'long`, where `'long ⊇ 'short`, also satisfies it.
// You can hand a long-lived thing to someone asking for a short-lived one
// — they'll just use it for less time than it could've supported.
//
// THE OPPOSITE DIRECTION DOES NOT WORK
//
// You CANNOT pass a `&'short T` where `&'long T` is required. The caller
// expects to use it for `'long`'s entire region; if the reference dies
// partway, that's a use-after-free. The compiler stops you.
//
// THE CLASSIC EXAMPLE: A CALLBACK
//
// Suppose you have a function that calls a callback `fn(&str)`:
//
//     fn run(callback: impl Fn(&str)) {
//         callback("hardcoded");          // pass a &'static str
//         let s = String::from("dynamic");
//         callback(&s);                    // pass a &'short_local str
//     }
//
// The callback's parameter type is `&str` — really `for<'any> &'any str`,
// "any lifetime works." Both call sites supply some specific lifetime, and
// both are accepted because `Fn(&str)` doesn't pin the lifetime down.
//
// More relevant for THIS exercise: a function that REQUIRES a
// short-lifetime callback can be CALLED with a function that supports
// `'static`. The longer-supporting function trivially satisfies the
// shorter-requiring caller.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// You'll implement a small higher-order function and then call it with
// inputs of mixed lifetimes to see the covariance in action.
//
//   1. `apply_to_str(s: &str, f: impl Fn(&str) -> usize) -> usize`
//        Just calls `f(s)` and returns the result. The point is the
//        signature: `f` accepts a reference of *any* lifetime that fits.
//
//   2. `count_bytes(s: &str) -> usize`
//        Returns `s.len()`. Implements `Fn(&str) -> usize`.
//
// Then the tests will hand `apply_to_str` both a `&'static str` (a
// literal) AND a short-lived `&str` borrowed from a local `String`. Both
// must work — that's covariance.

// I AM NOT DONE

fn apply_to_str(s: &str, f: impl Fn(&str) -> usize) -> usize {
    ???
}

fn count_bytes(s: &str) -> usize {
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn static_str_input() {
        // The literal is &'static str. apply_to_str's parameter is just
        // &str (any lifetime that fits). Subtyping: 'static ⊇ anything,
        // so &'static str satisfies the shorter requirement.
        let n = apply_to_str("hello", count_bytes);
        assert_eq!(n, 5);
    }

    #[test] fn short_lived_input() {
        let owned = String::from("dynamic");
        // &owned is some &'a str where 'a is the local scope. Still
        // satisfies apply_to_str's "any lifetime that fits" signature.
        let n = apply_to_str(&owned, count_bytes);
        assert_eq!(n, 7);
    }

    #[test] fn closure_capturing_static_data() {
        // A closure that captures a `&'static str` from outside.
        // The closure's type implements Fn(&str) -> usize regardless
        // of the lifetime of `s` it's called with — covariance again.
        let prefix: &'static str = "prefix=";
        let with_prefix = |s: &str| -> usize { prefix.len() + s.len() };
        assert_eq!(apply_to_str("xy", with_prefix), 9);

        let owned = String::from("zzzz");
        assert_eq!(apply_to_str(&owned, with_prefix), 11);
    }

    #[test] fn count_bytes_works_directly() {
        assert_eq!(count_bytes("rust"), 4);
        assert_eq!(count_bytes(""), 0);
    }
}

fn main() {}
