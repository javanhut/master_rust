// SOLUTION — trait3_derive

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Tag {
    name: String,
}

// WHY THIS IS OPTIMAL:
//
//   One derive line, six trait impls, zero hand-written code. Each derived
//   impl defers field-by-field to the corresponding impl on `String`,
//   which already implements all six. No allocation, no surprises, the
//   compiler audits the structure.
//
// WHY EACH ONE IS NEEDED:
//
//   - Debug    — `{:?}` formatting in `format!("{:?}", t)`.
//   - Clone    — `t.clone()` to duplicate in the test.
//   - PartialEq — `assert_eq!(t, u)` and `s.contains(&Tag { ... })`.
//   - Eq       — `HashSet` requires its key type to be `Eq` (a stronger
//                promise than `PartialEq`: equality is reflexive). Without
//                it, `HashSet<Tag>` itself doesn't compile.
//   - Hash     — also required for `HashSet<Tag>`, so the set can pick a
//                bucket for the value.
//   - Default  — `Tag::default()` returns `Tag { name: String::default() }`
//                which is `Tag { name: String::new() }` — the empty string.
//
// IF YOU FORGET Eq:
//
//     #[derive(Debug, Clone, PartialEq, Hash, Default)]
//     struct Tag { name: String }
//     let _: HashSet<Tag> = HashSet::new();
//     // error[E0277]: the trait bound `Tag: Eq` is not satisfied
//
//   PartialEq alone isn't enough for hash-based collections — they need
//   the marker that says equality is total.
//
// IF YOU FORGET Hash:
//
//   Same shape of error, this time complaining that `Tag: Hash` isn't met.
//   You can have `Eq` without `Hash` (e.g. for an `Ord`-based BTreeSet),
//   but for `HashSet`/`HashMap` you need both.
//
// WHY NOT ALSO `Copy`?
//
//   `Copy` requires that EVERY field is `Copy`. `String` is not `Copy`
//   (it owns a heap allocation), so deriving `Copy` here would fail.
//   `Clone` is the right choice for any struct with owned heap data.
