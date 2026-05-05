// =============================================================================
//  lt2 — explicit lifetime annotations on functions
// =============================================================================
//
// You wrote one of these in chapter 4 without really learning what it meant:
//
//     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//
// Now we read it carefully.
//
// HOW TO READ A LIFETIME-ANNOTATED SIGNATURE
//
//   `<'a>`        — a lifetime PARAMETER, declared in the angle brackets
//                   exactly like a generic type parameter. The function is
//                   generic over some region of code `'a` chosen by the
//                   CALLER (well, by the compiler at the call site).
//
//   `a: &'a str`  — "a is a reference into a string slice that's valid for
//                   AT LEAST the region `'a`."
//
//   `-> &'a str`  — "the returned reference is valid for the region `'a`,
//                   not longer."
//
// At each call site the compiler picks `'a` to be the INTERSECTION of the
// scopes of whatever the caller passes in. So if you call:
//
//     let s1 = String::from("hello");
//     let result;
//     {
//         let s2 = String::from("hi");
//         result = longest(&s1, &s2);   // 'a here = scope of s2 (the smaller)
//         println!("{result}");          // OK — still inside 'a
//     }
//     // println!("{result}");          // would NOT compile — past 'a
//
// The function does not "remember" anything; it just promises that the
// returned reference is valid wherever both inputs are. The compiler then
// holds the caller to that.
//
// IMPORTANT: ANNOTATIONS DO NOT AFFECT RUNTIME
//
// Lifetimes exist only at compile time. They do not change what the
// generated assembly does, what gets allocated, or how long anything
// lives. The runtime behavior of `longest` would be identical even if
// we could erase the `'a`s — and indeed in the binary they ARE erased.
//
// What annotations do is let the compiler check a CONSTRAINT:
//
//   "The output reference is valid for a region no larger than the
//    intersection of the regions the inputs are valid for."
//
// Without `'a`, the compiler can't tell whether the output borrows from
// `a`, from `b`, or from a static somewhere — three different stories
// with three different rules. So it asks YOU to commit to one.
//
// WHEN YOU MUST ANNOTATE
//
//   - The function returns a reference, AND
//   - There are two-or-more input references, AND
//   - The compiler can't elide (see lt3).
//
// In practice: most functions don't need explicit lifetimes. The ones that
// do are the ones that "thread a borrow through" — taking a reference in
// and giving a (sub-)reference out.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Implement two functions. Both take two `&str`s and return one of them.
// The signatures are given to you — fill in the `<'a>` annotations and
// the bodies.
//
//   1. `longer_of<'a>(a: &'a str, b: &'a str) -> &'a str`
//        Return whichever input has the larger BYTE length.
//        Ties: return `a`.
//
//   2. `prefix_or<'a>(s: &'a str, fallback: &'a str) -> &'a str`
//        Return `s` if `s` starts with the ASCII letter 'p'; otherwise
//        return `fallback`. Use `s.starts_with('p')`.
//
// This file has tests. Make them pass.

// I AM NOT DONE

fn longer_of???(a: &???str, b: &???str) -> &???str {
    if a.len() >= b.len() { a } else { ??? }
}

fn prefix_or???(s: &???str, fallback: &???str) -> &???str {
    if s.starts_with('p') { ??? } else { ??? }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn longer_a_wins() {
        assert_eq!(longer_of("hello", "hi"), "hello");
    }
    #[test] fn longer_b_wins() {
        assert_eq!(longer_of("a", "abcd"), "abcd");
    }
    #[test] fn longer_tie_returns_a() {
        assert_eq!(longer_of("rust", "lang"), "rust");
    }

    #[test] fn prefix_hits() {
        assert_eq!(prefix_or("python", "n/a"), "python");
    }
    #[test] fn prefix_misses() {
        assert_eq!(prefix_or("rust", "n/a"), "n/a");
    }

    // The interesting compile-time check: with `'a` correctly written,
    // the call site picks the SMALLER of the two input scopes for `'a`,
    // and the result is valid in that smaller scope. We exercise that
    // here — the result is consumed inside the inner scope, never escapes.
    #[test] fn shorter_scope_is_fine() {
        let outer = String::from("aaaaa");
        let captured;
        {
            let inner = String::from("bbb");
            let r = longer_of(&outer, &inner);
            captured = r.to_string(); // copy out — `r` itself dies with `inner`
        }
        assert_eq!(captured, "aaaaa");
    }
}

fn main() {}
