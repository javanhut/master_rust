// SOLUTION — macro4_hygiene

macro_rules! with_temp_double {
    ( $x:expr ) => {{
        let _tmp = $x;
        _tmp + _tmp
    }};
}

// WHY THIS IS OPTIMAL:
//
//   The macro names `_tmp` directly inside its body. In a hygienic macro
//   system that name lives in the macro's OWN syntax context — so even if
//   the caller has a `let _tmp = 99;` in scope, the two names are
//   considered distinct identifiers by the compiler. The caller's `_tmp`
//   is not shadowed; the macro's `_tmp` is invisible from outside.
//
//   `$x:expr` is a fragment captured from the caller's tokens. Captured
//   fragments KEEP their original syntax context, so `$x` resolves names
//   in the caller's scope — which is why `with_temp_double!(n)` can see
//   the caller's local `n`.
//
// THE TWO HALVES OF HYGIENE:
//
//   1. NAMES INTRODUCED IN THE MACRO BODY do not leak to the caller.
//      Our `let _tmp` does not shadow the caller's `_tmp`.
//
//   2. NAMES THE MACRO MENTIONS (without `$`) cannot accidentally pick
//      up the caller's bindings. If we wrote `count` literally inside the
//      macro and the caller happened to have a local `count`, the macro's
//      `count` would still resolve in the macro's own scope (and fail to
//      compile if no `count` exists there). To use a caller's value, you
//      MUST receive it via a `$capture`.
//
// CONTRAST WITH C:
//
//     #define SWAP(a, b) { int tmp = a; a = b; b = tmp; }
//
//   ...is broken in C because the textual `tmp` collides with any
//   caller-side `tmp`. C programmers work around it with ugly prefixes
//   like `__SWAP_TMP__` and hope. Rust eliminates the entire class of
//   bug at the language level.
//
// THE BLOCK EXPRESSION `{{ ... }}`:
//
//   The OUTER `{ }` is the macro's expansion delimiter (every rule wraps
//   its expansion in `{ ... }` or `( ... )` or `[ ... ]`). The INNER
//   `{ ... }` is a Rust BLOCK EXPRESSION — it lets the macro produce a
//   value (the last expression `_tmp + _tmp`). Without the inner block
//   you'd need to give the expansion a different shape.
//
// LIMITS OF HYGIENE (worth knowing):
//
//   `macro_rules!` is hygienic for IDENTIFIERS but not for everything —
//   most notably, lifetimes and labels are only partially hygienic, and
//   PROCEDURAL macros have to opt into hygiene more carefully. For
//   day-to-day declarative macros the rule of thumb is simple: name
//   internal bindings whatever you like, and let `$captures` carry the
//   caller's intent in.
//
// COMMON MISTAKES:
//
//   - Trying to "share" a name between the caller and the macro body
//     by writing it bare. That's the C mindset; Rust will not see them
//     as the same name.
//   - Forgetting the inner `{ ... }` and ending up with an expansion
//     that's a sequence of statements rather than an expression.
