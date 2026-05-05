// SOLUTION — lt1_intro

fn first_char() -> char {
    let s = vec!['h', 'i'];
    let c: &char = &s[0];
    *c
}

fn main() {
    let c = first_char();
    println!("first char is {c}");
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The fix is structural, not syntactic. The original code had two scopes:
//   the outer function body, and an inner `{ ... }` block where `s` lived.
//   The reference `c` was DECLARED in the outer scope, so it was expected
//   to live there — but its referent `s` died at the end of the inner
//   block. That's the exact pattern Rust forbids.
//
//   By pulling `s` up to the outer scope, both `s` and `c` now share the
//   same enclosing region. The compiler can label them with the same
//   lifetime (call it `'fn`, the function body's region) and prove that
//   every USE of `c` happens inside the region `s` is alive.
//
//   We didn't change any types. We didn't write `'a` anywhere. We just
//   gave the value a long enough scope to back the reference. That is
//   the day-to-day fix for 90% of borrow-checker errors: extend the
//   value's life, or shorten the reference's use.
//
// THE RULE, RESTATED:
//
//   A reference's lifetime must be CONTAINED IN the scope of the value
//   it borrows from. The compiler's only job here is checking that
//   containment. If the containment doesn't hold, the program won't
//   compile — no exceptions, no runtime check, no "it usually works".
//
// ALTERNATIVES:
//
//   1. Don't take the reference at all — just read the value:
//
//          fn first_char() -> char {
//              let s = vec!['h', 'i'];
//              s[0]            // `char` is Copy, so no borrow needed
//          }
//
//      This is cleaner for primitives. We kept the explicit `&` here
//      because the lesson is about REFERENCES, not about avoiding them.
//
//   2. Return the owned value through a different shape entirely (e.g.,
//      take `s` as a parameter so the caller controls its scope). That's
//      the move we'll explore in lt2.
//
// PREVIEW: in the next exercise we'll meet `'a` written in a function
// signature, and you'll see WHY the compiler sometimes can't elide it.
