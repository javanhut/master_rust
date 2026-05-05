// SOLUTION — macro2_macro_rules

macro_rules! say_hello {
    () => {
        println!("hello")
    };
}

macro_rules! square {
    ( $x:expr ) => {
        ($x * $x)
    };
}

// WHY THIS IS OPTIMAL:
//
//   - `say_hello!()` matches the empty pattern `()`. The expansion is a
//     plain `println!` call. The macro adds nothing the caller could not
//     have written by hand — but it demonstrates the simplest possible
//     `macro_rules!` rule.
//
//   - `square!($x:expr)` uses the `expr` fragment specifier so the input
//     can be ANY expression — a literal, an arithmetic combination, a
//     function call, a variable. The expansion is `($x * $x)`.
//
// WHY THE OUTER PARENTHESES MATTER:
//
//   Rust macros expand TOKENS, not values. If `square!` were defined as
//   `$x * $x` and you called `square!(2 + 3)`, the expansion would be
//   `2 + 3 * 2 + 3` — multiplication binds tighter than addition, so the
//   result is `11`, not `25`. The fix is to wrap the whole expansion in
//   parentheses so the macro acts as a single grouped expression.
//
//   The same reasoning applies to wrapping `$x` itself if you intended to
//   use it more than once: `($x) * ($x)` is even safer if `$x` could be
//   something gnarly. But because `expr` is already PRE-PARSED (not raw
//   text), Rust knows where it begins and ends — internally it is treated
//   as one node — so a single outer set of parens is enough for all
//   well-formed expressions. This is one of the major upgrades over C's
//   textual macros, where you had to wrap EVERYTHING.
//
// THE FRAGMENT SPECIFIER MENU (alternatives to `expr`):
//
//     ident   — `square!(x)` would take a NAME, not an expression. You
//               could not pass `2 + 3`. Useful when generating bindings.
//     ty      — `square!(i32)` would take a type. Useful for type-level
//               macros (e.g. building a typed-id newtype).
//     tt      — accepts ANY single token tree. Most permissive but defers
//               error messages to the call site of the expansion.
//     literal — only literals (`42`, `"hi"`). Rules out `x + 1`.
//     pat     — for matching patterns inside generated `match` arms.
//     block   — `{ ... }` blocks. Useful for `lazy_init!({ ... })`-style
//               macros.
//
//   The general rule: pick the MOST RESTRICTIVE specifier that still
//   accepts what you need. That tightens diagnostics and makes the macro
//   harder to misuse.
//
// COMMON MISTAKES:
//
//   - Forgetting the trailing `;` on a rule (each arm of a macro_rules!
//     match block ends with one).
//   - Reaching for `tt` when `expr` would do — you lose the better error
//     messages and hygiene that come with `expr`.
//   - Forgetting the outer parens around the expansion. `square!(a + b)`
//     bites you immediately.
