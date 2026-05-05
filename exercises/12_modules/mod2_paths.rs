// =============================================================================
//  mod2 — paths: crate, super, self, and `use`
// =============================================================================
//
// Every item in a Rust program has a PATH that names it uniquely. There are
// two flavours of path:
//
//   ABSOLUTE — starts at the crate root.
//
//       crate::outer::inner::thing
//
//   RELATIVE — starts at the current module.
//
//       self::sibling                  (the current module)
//       super::neighbour               (the parent module)
//       inner::thing                   (a child module)
//
// CRATE ROOT
//
// The `crate` keyword always means "the top of THIS crate" (the file being
// compiled, in our single-file world). It's how nested code reaches
// top-level items unambiguously.
//
// SUPER
//
// `super` means "one module up." A child can poke at the parent's items
// (subject to privacy) without knowing the parent's full path:
//
//     mod parent {
//         pub fn hi() {}
//         mod child {
//             pub fn call_parent() { super::hi(); }   // up one level
//         }
//     }
//
// SELF
//
// `self` means "this module right here." It's mostly used in `use`
// statements for clarity: `use self::helpers::foo;`.
//
// NESTED MODULES
//
// You can nest modules as deep as you like:
//
//     mod a {
//         pub mod b {
//             pub fn thing() -> i32 { 42 }
//         }
//     }
//
// To use `thing` from outside, the full path is `a::b::thing`. If the
// child module isn't `pub`, the parent's items aren't reachable through it
// even if THEY are `pub` — privacy applies at every step of the path.
//
// `use` — IMPORT NAMES INTO SCOPE
//
//     use crate::a::b::thing;       // now you can write `thing()` directly.
//     use crate::a::b;              // now you can write `b::thing()`.
//     use crate::a::b::{thing, other};   // multiple at once
//     use crate::a::b::thing as t;       // rename on import
//
// `use` only creates a LOCAL ALIAS — it doesn't change visibility. If
// `thing` were private, the `use` would fail at compile time.
//
// FILE-BASED NOTE
//
// In real projects, `mod a { mod b { ... } }` would correspond to
// `src/a/mod.rs` (or `src/a.rs`) declaring `pub mod b;`, with `b`'s
// contents in `src/a/b.rs`. The path syntax is identical — `crate::a::b`
// either way.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build the following structure:
//
//     mod outer
//         pub const BASE: i32 = 10
//         pub mod inner
//             pub fn add_base(x: i32) -> i32      // returns x + BASE,
//                                                 // reaching BASE via `super::`
//             pub fn add_two_base(x: i32) -> i32  // returns x + 2*BASE,
//                                                 // reaching BASE via `crate::outer::`
//
// At the top level, write:
//
//     fn use_inner(x: i32) -> i32
//         — uses `use crate::outer::inner::add_base;` so the body can call
//           `add_base(x)` directly with no `outer::inner::` prefix.

// I AM NOT DONE

mod outer {
    pub const BASE: i32 = 10;

    pub mod inner {
        // Reach the parent's BASE via `super`.
        pub fn add_base(x: i32) -> i32 {
            x + ???::BASE
        }

        // Reach the same BASE via the absolute path from the crate root.
        pub fn add_two_base(x: i32) -> i32 {
            x + 2 * ???::outer::BASE
        }
    }
}

// Bring `add_base` directly into scope so the body of `use_inner` doesn't
// need to spell out the full path each time.
use crate::???::???::add_base;

fn use_inner(x: i32) -> i32 {
    add_base(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn add_base_via_super()  { assert_eq!(outer::inner::add_base(5), 15); }
    #[test] fn add_two_base_via_crate() { assert_eq!(outer::inner::add_two_base(5), 25); }
    #[test] fn use_brings_into_scope()  { assert_eq!(use_inner(7), 17); }
}

fn main() {}
