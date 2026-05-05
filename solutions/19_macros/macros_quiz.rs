// SOLUTION — macros_quiz

macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

// WHY THIS IS OPTIMAL:
//
//   The classic `hashmap!` macro pulls together every idea from this
//   chapter:
//
//   - `macro_rules!` rule, ONE arm.
//   - `$k:expr` and `$v:expr` capture the key/value as full expressions
//     (so callers can pass anything, not just literals).
//   - `=>` is a literal token in the pattern — pattern matchers can use
//     any token they like as a separator, not just `,` or `;`.
//   - `$( ... ),*` accepts zero-or-more comma-separated pairs, so
//     `hashmap!{}` produces an empty map.
//   - `$(,)?` accepts an optional trailing comma — the same ergonomic
//     feature that `vec!`, `println!`, and `assert_eq!` give callers.
//   - The inner `{ ... }` is a Rust block expression that yields the
//     constructed `HashMap` as its value, while the outer `{ ... }` is
//     the macro's expansion delimiter.
//   - `::std::collections::HashMap` — note the LEADING `::`. That is the
//     ABSOLUTE-PATH form, anchored at the crate root. It means the macro
//     works even if the caller has shadowed `std` or never imported
//     `HashMap`. Library-quality macros always use absolute paths.
//   - Hygiene means our `let mut m = ...` does not collide with any
//     `m` the caller may have in scope — even nested `hashmap!` calls
//     work without interference.
//
// EXPANSION FOR ONE EXAMPLE:
//
//   Source:
//
//       hashmap! {
//           "a" => 1,
//           "b" => 2,
//       }
//
//   After macro expansion (paraphrased):
//
//       {
//           let mut m = ::std::collections::HashMap::new();
//           m.insert("a", 1);
//           m.insert("b", 2);
//           m
//       }
//
// ALTERNATIVES YOU MAY SEE:
//
//   1. Pre-sizing with `with_capacity` — the standard `maplit::hashmap!`
//      macro counts the entries and reserves capacity:
//
//          let mut m = ::std::collections::HashMap::with_capacity(
//              <[()]>::len(&[$( () ),*])
//          );
//
//      Clever (and zero-cost), but harder to read. Worth knowing as an
//      idiom: `<[()]>::len(&[$( () ),*])` counts repetitions at compile
//      time by building an array of unit values and asking its length.
//
//   2. Returning `BTreeMap` instead of `HashMap` — same shape, swap the
//      type. Many projects ship one macro per map type.
//
//   3. Using `From<[(K, V); N]>` (Rust 1.56+) — for FIXED-SIZE inputs you
//      can just write `HashMap::from([("a", 1), ("b", 2)])`. No macro
//      needed. The macro still wins for nicer syntax and a heterogeneous
//      list of expressions.
//
// COMMON MISTAKES:
//
//   - Forgetting `$(,)?` and getting yelled at for `hashmap!{ "a" => 1, }`.
//   - Using `HashMap::new()` (relative path) instead of
//     `::std::collections::HashMap::new()` — works only if the caller
//     imported `HashMap`. Bad hygiene.
//   - Reaching for `tt` instead of `expr` for the key/value. You'd lose
//     pre-parsing and get worse error messages.
//
// CHAPTER 19 TAKEAWAYS:
//
//   1. Macros expand BEFORE type-checking. They rewrite tokens.
//   2. Use `macro_rules!` for the 90% of cases that don't need to walk
//     a real AST. Fragment specifiers (`expr`, `ident`, `ty`, `tt`,
//     `block`, ...) make the patterns precise.
//   3. Repeaters `$( )*` / `$( )+` / `$( )?` give variable arity. Mirror
//      the same shape on the expansion side.
//   4. Hygiene means your macro's internal names don't leak into the
//      caller. Use clean local bindings without fear.
//   5. Procedural macros (separate crate, `syn` + `quote`) are the
//      heavier hammer for real AST transforms — `#[derive(...)]`,
//      `#[tokio::main]`, `sqlx::query!`. Reach for them when
//      `macro_rules!` runs out of road.
