// SOLUTION — lt2_function

fn longer_of<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn prefix_or<'a>(s: &'a str, fallback: &'a str) -> &'a str {
    if s.starts_with('p') { s } else { fallback }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   Both signatures share the same shape: two input references, one
//   output reference, the output potentially being either of the inputs.
//   That's the canonical case where the compiler cannot elide and you
//   must say `<'a>` yourself.
//
//   Reading `longer_of<'a>(a: &'a str, b: &'a str) -> &'a str`:
//
//     "Pick a region `'a`. Both inputs are guaranteed valid for at
//      least `'a`, and the result is valid for exactly `'a`."
//
//   The caller's compiler does the picking — it chooses `'a` to be the
//   intersection of the two input scopes, which is just "the smaller of
//   the two." That's why the `shorter_scope_is_fine` test works: the
//   compiler shrinks `'a` down to `inner`'s scope, and we use the result
//   only inside that scope. Past the closing brace of the inner block,
//   any leftover reference would be illegal — but we copied the data
//   out with `.to_string()` first, so nothing dangles.
//
//   `prefix_or` has the same shape; the body is just a different choice
//   of which input to return. Both inputs share the lifetime, so either
//   branch is acceptable.
//
// WHY NOT TWO LIFETIMES?
//
//   You COULD write
//
//       fn longer_of<'a, 'b>(a: &'a str, b: &'b str) -> &??? str { ... }
//
//   but then the return type's lifetime is unconstrained — you can't
//   write `&'a str` (the result might be `b`) or `&'b str` (the result
//   might be `a`). To express "either input", you have to merge them:
//   one shared `'a`. That merging is what `<'a>` plus `&'a` everywhere
//   accomplishes.
//
//   You'd use `<'a, 'b>` when the output is genuinely tied to ONLY ONE
//   of the inputs. Example:
//
//       fn first_word<'a, 'b>(s: &'a str, _sep: &'b str) -> &'a str
//
//   The output borrows from `s` only; `_sep` is read but not stored.
//
// REMINDER: lifetimes are erased at compile time. Both `longer_of` and a
// hypothetical `longer_of_no_lifetimes` (which doesn't exist as valid
// Rust) would generate identical machine code. The annotations are
// pure proof obligations for the borrow checker.
//
// ALTERNATIVES YOU MIGHT SEE:
//
//   1. Returning `String` (owned) instead of `&str`:
//
//          fn longer_of(a: &str, b: &str) -> String {
//              if a.len() >= b.len() { a.to_string() } else { b.to_string() }
//          }
//
//      No lifetimes needed — but you allocate every call. Use this when
//      the caller will outlive the inputs and you need ownership.
//
//   2. Returning a `Cow<'a, str>` to defer the choice of owned-vs-borrowed
//      to runtime. Powerful but heavy for this lesson; we'll meet `Cow`
//      later.
