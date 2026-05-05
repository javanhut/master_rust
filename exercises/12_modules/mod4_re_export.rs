// =============================================================================
//  mod4 — re-exports and the facade pattern
// =============================================================================
//
// `pub use` does two things at once:
//
//   1. Imports a name into the current module (like `use`),
//   2. RE-EXPORTS that name from the current module's public surface.
//
// Example:
//
//     mod internal {
//         pub struct Engine;
//         impl Engine {
//             pub fn new() -> Self { Engine }
//             pub fn run(&self)    { /* ... */ }
//         }
//     }
//
//     pub mod api {
//         pub use crate::internal::Engine;     // <-- re-export
//     }
//
// Now external callers write `api::Engine` even though the type LIVES in
// `internal`. They don't know (or care) where it comes from. Tomorrow you
// can move `Engine` to `internal::v2` and update one `pub use` line — the
// public path `api::Engine` doesn't change.
//
// THE FACADE PATTERN
//
// A real crate is often organised internally by IMPLEMENTATION concern
// (`parser`, `lexer`, `ast`, `eval`) but exposed externally by USER concern
// (a curated, flat `pub mod api` that re-exports the handful of names a
// caller actually needs). The internal modules can refactor freely; the
// facade is the stable surface.
//
//     // Internal: organised for the maintainer.
//     mod parser   { pub struct Parser; pub fn parse(_s: &str) {} }
//     mod lexer    { pub struct Token;  pub fn tokenize(_s: &str) {} }
//
//     // External: organised for the user.
//     pub mod api {
//         pub use crate::parser::{Parser, parse};
//         pub use crate::lexer::{Token, tokenize};
//     }
//
// Callers see a single `api::*` namespace. Maintainers keep their nice
// per-feature folders. Everyone wins.
//
// COMMON GOTCHAS
//
//   - `pub use` only works if the original item is reachable. Re-exporting
//     a private item is a compile error.
//
//   - You can rename on re-export:
//         pub use crate::lexer::Token as LexToken;
//
//   - You can re-export a whole module: `pub use crate::lexer;`.
//
// FILE-BASED NOTE
//
// In a real crate the layout would be `src/parser.rs`, `src/lexer.rs`,
// `src/api.rs` (or `src/api/mod.rs`), with the same `pub use` lines at the
// top of `api`. The runner here compiles a single file with rustc, so we
// keep everything inline — same semantics, fewer files.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build the following structure:
//
//   mod arith
//       pub fn add(a: i32, b: i32) -> i32      // a + b
//
//   mod text
//       pub fn shout(s: &str) -> String        // s.to_uppercase()
//
//   pub mod api
//       — re-exports `arith::add` so callers can use `api::add`
//       — re-exports `text::shout` so callers can use `api::shout`
//
// Then write a top-level free function `demo() -> (i32, String)` that calls
// `api::add(2, 3)` and `api::shout("hi")` and returns both results as a
// tuple.

// I AM NOT DONE

mod arith {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod text {
    pub fn shout(s: &str) -> String {
        s.???()
    }
}

pub mod api {
    // Re-export each name so callers can reach them via `api::add` / `api::shout`
    // without ever naming `arith` or `text`.
    ??? use crate::arith::add;
    ??? use crate::text::shout;
}

fn demo() -> (i32, String) {
    (api::???(2, 3), api::???("hi"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn api_add_works() {
        assert_eq!(api::add(2, 3), 5);
    }

    #[test] fn api_shout_works() {
        assert_eq!(api::shout("hi"), "HI");
    }

    #[test] fn demo_returns_both() {
        assert_eq!(demo(), (5, String::from("HI")));
    }

    // The original paths still work — `pub use` ADDS a name, doesn't remove
    // the original. Most crates eventually mark the internals `pub(crate)`
    // so only the facade is reachable from outside, but for teaching we
    // keep both paths visible.
    #[test] fn original_paths_still_reachable() {
        assert_eq!(arith::add(1, 1), 2);
        assert_eq!(text::shout("yo"), "YO");
    }
}

fn main() {}
