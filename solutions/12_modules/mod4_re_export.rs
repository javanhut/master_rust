// SOLUTION — mod4_re_export

mod arith {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod text {
    pub fn shout(s: &str) -> String {
        s.to_uppercase()
    }
}

pub mod api {
    pub use crate::arith::add;
    pub use crate::text::shout;
}

fn demo() -> (i32, String) {
    (api::add(2, 3), api::shout("hi"))
}

// WHY THIS IS OPTIMAL:
//
//   The internal layout (`arith`, `text`) reflects how the maintainer
//   thinks. The facade (`api`) reflects how the user thinks: a single
//   namespace with the curated public surface. `pub use` is the bridge.
//
//   Why `pub use crate::arith::add` instead of `pub fn add(a, b) { arith::add(a, b) }`?
//   — Re-exports COST NOTHING. The compiler knows it's just an alias; no
//     wrapper function is generated. They also preserve the original
//     item's signature, attributes, and docs verbatim — wrappers always
//     drift over time.
//
//   The "original paths still work" test demonstrates that `pub use`
//   ADDS a name; it doesn't take anything away. To truly hide
//   `arith` / `text`, you'd mark them `pub(crate)` (or leave them
//   non-`pub`) so only the facade exposes them outside the crate.
//
// EQUIVALENT BUT NOISIER:
//
//   pub mod api {
//       use crate::arith;
//       use crate::text;
//       pub fn add(a: i32, b: i32) -> i32 { arith::add(a, b) }
//       pub fn shout(s: &str) -> String   { text::shout(s) }
//   }
//     — works, but you've now written a thunk for every function and
//       you'll need to maintain it as signatures evolve. `pub use` is
//       both shorter and refactor-safe.
//
// RENAMING ON RE-EXPORT:
//
//   pub use crate::text::shout as upper;
//     — exposes it as `api::upper(...)`. Useful when your internal name
//       is technical and your public name should read more naturally.
//
// FILE-BASED EQUIVALENT:
//
//   In a real crate `arith` and `text` would be `src/arith.rs` and
//   `src/text.rs`; `api` would be `src/api.rs` (or `src/api/mod.rs`)
//   containing exactly the two `pub use` lines you see above. The
//   facade pattern is identical — only the file boundaries change.
