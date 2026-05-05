// =============================================================================
//  macro4 — macro hygiene
// =============================================================================
//
// HYGIENE is the property that makes a macro behave like a function call
// instead of textual cut-and-paste: identifiers introduced INSIDE the
// macro live in the macro's own scope, separate from any identifiers
// with the same name at the call site. Rust's `macro_rules!` macros are
// hygienic; C's preprocessor macros are NOT.
//
// THE C HORROR STORY (for contrast)
// ─────────────────────────────────
//
// In C you might write:
//
//     #define SWAP(a, b) { int tmp = a; a = b; b = tmp; }
//
// Then call:
//
//     int tmp = 99;
//     int x = 1, y = 2;
//     SWAP(x, y);
//     printf("%d\n", tmp);   // ?? — the macro's `int tmp` clobbered yours.
//
// The C preprocessor does textual substitution. Both `tmp`s end up in
// the same scope — the LATER `int tmp` shadows the earlier one — and
// the program prints something unexpected. C developers work around
// this by giving macro-internal names ugly prefixes (`__SWAP_TMP__`),
// hoping nobody at the call site uses that name.
//
// RUST: HYGIENIC BY DESIGN
// ─────────────────────────
//
// In Rust, an identifier introduced inside a `macro_rules!` expansion
// lives in a SEPARATE syntax context from the caller's identifiers.
// Two `tmp`s — one in the macro, one at the call site — are treated as
// distinct names by the compiler, even though they spell the same.
//
// You can verify it by writing the macro and calling it from a function
// where the same name already exists. Both bindings coexist.
//
// IT WORKS THE OTHER WAY TOO
//
// A macro CANNOT accidentally capture a caller's local. If a macro body
// names `count` directly (without `$count`), that `count` resolves in the
// macro's scope, not the caller's. To USE a caller's variable, you must
// pass it in as a `$x:expr` capture. Captured fragments retain the
// caller's scope, so `$x` resolves where the macro was called.
//
// SUMMARY
//
//     identifier              context
//     ──────────              ────────────────────────────────
//     literal in macro body   macro's own scope
//     `$captured` fragment    caller's scope (passed through)
//
// This separation means you can safely write `let _tmp = ...` inside
// your macro and your callers can keep their own `_tmp` undisturbed.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement a `with_temp_double!` macro that:
//   - Takes one expression `$x:expr`.
//   - Internally binds `let _tmp = $x;` (hygienic — the macro's own _tmp).
//   - Returns `_tmp + _tmp` (so it doubles `$x`).
//
// The TEST then sets up a caller-side `let _tmp = 99;` and verifies that
// the macro's internal `_tmp` does NOT clobber the caller's `_tmp`. If
// hygiene is broken, the test fails.
//
// Use a BLOCK expression `{{ ... }}` so the expansion is one value.
//
// Replace each `???`. Don't change the tests.

// I AM NOT DONE

macro_rules! with_temp_double {
    ( $x:??? ) => {{
        // Macro-internal temporary. Lives in the macro's scope ONLY.
        let _tmp = $x;
        ??? + ???
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_does_not_clobber_caller_tmp() {
        // The caller has its own _tmp. The macro also uses _tmp internally.
        // Hygiene must keep them apart: the caller's _tmp survives the call.
        let _tmp = 99;
        let doubled = with_temp_double!(7);
        assert_eq!(doubled, 14);
        assert_eq!(_tmp, 99); // caller's _tmp untouched
    }

    #[test]
    fn macro_uses_passed_expression() {
        // The captured `$x` evaluates in the CALLER's scope, so it can
        // reference local bindings here.
        let n = 21;
        assert_eq!(with_temp_double!(n), 42);
    }

    #[test]
    fn nested_invocations_dont_collide() {
        // Two expansions in the same scope each get their own _tmp.
        let a = with_temp_double!(5);
        let b = with_temp_double!(10);
        assert_eq!(a + b, 30);
    }
}

fn main() {}
