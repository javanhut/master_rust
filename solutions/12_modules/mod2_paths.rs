// SOLUTION — mod2_paths

mod outer {
    pub const BASE: i32 = 10;

    pub mod inner {
        pub fn add_base(x: i32) -> i32 {
            x + super::BASE
        }

        pub fn add_two_base(x: i32) -> i32 {
            x + 2 * crate::outer::BASE
        }
    }
}

use crate::outer::inner::add_base;

fn use_inner(x: i32) -> i32 {
    add_base(x)
}

// WHY THIS IS OPTIMAL:
//
//   `super::BASE` is the right tool for `add_base` because the relationship
//   it expresses is "my parent's constant." If `outer` were renamed
//   tomorrow, `super::` keeps working with zero edits — that's relative
//   addressing earning its keep.
//
//   `crate::outer::BASE` in `add_two_base` is the absolute spelling. It
//   never depends on where the calling module sits in the tree. Use it
//   when you cross several module boundaries or when the relative path
//   would be longer / less obvious than the absolute one.
//
//   The top-level `use crate::outer::inner::add_base;` brings ONE name into
//   the file's scope. Inside `use_inner` we then write `add_base(x)`
//   instead of `outer::inner::add_base(x)` every time — clearer at the
//   call site, and a single edit point if the path moves.
//
// EQUIVALENT PATHS (all reach the same constant):
//
//   self::super::BASE                  // works inside `inner` — `self` is
//                                      // `inner`, `super` of that is `outer`.
//                                      // Verbose; almost never written.
//
//   crate::outer::BASE                 // absolute, also fine.
//
//   In `add_base` you could also write `crate::outer::BASE` — both
//   compile. The point of using one of each in this exercise is to drill
//   that they are interchangeable, not that one is mandatory.
//
// USE ALTERNATIVES:
//
//   use crate::outer::inner;           // then `inner::add_base(x)` at call sites
//   use crate::outer::inner::add_base as ab;     // rename on import
//   use crate::outer::inner::{add_base, add_two_base};   // grouped
//
//   Pick the form that reads best. For two or three names, `use` the
//   functions directly; for many, `use` the parent module to keep call
//   sites self-documenting.
