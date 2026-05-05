// =============================================================================
//  macro2 — `macro_rules!`: writing your first declarative macros
// =============================================================================
//
// Rust has TWO kinds of macros:
//
//   - DECLARATIVE macros, written with `macro_rules!`. They look like a
//     `match` over input tokens — you list patterns and the code each one
//     expands to. Pure source-to-source rewriting; no Rust crates needed.
//   - PROCEDURAL macros, written as Rust functions in a separate
//     `proc-macro` crate. They take a `TokenStream` in and emit one out.
//     That's how `#[derive(Debug)]`, `#[tokio::main]`, `serde_derive`, and
//     friends work. We meet those in macro5.
//
// This file is about declarative macros — the everyday workhorse.
//
// SHAPE
// ─────
//
//     macro_rules! NAME {
//         ( PATTERN )      => { EXPANSION };
//         ( OTHER PATTERN) => { OTHER EXPANSION };
//     }
//
// Patterns are matched against the token stream that follows the macro
// invocation. The first pattern that matches wins — order matters.
//
// FRAGMENT SPECIFIERS (you'll memorise these)
//
// Inside a pattern, `$name:KIND` captures a chunk of input under `$name`.
// The most useful KINDs:
//
//     expr     — any expression: `1 + 2`, `foo()`, `if x { 1 } else { 2 }`.
//     ident    — a single identifier: `foo`, `MyType`. NOT a path.
//     ty       — a type: `i32`, `Vec<u8>`, `&'a mut T`.
//     pat      — a pattern: `Some(x)`, `(a, b)`.
//     stmt     — a statement.
//     block    — a `{ ... }` block expression.
//     literal  — `42`, `"hi"`, `'x'`, `true`.
//     path     — `std::vec::Vec`, `crate::foo::bar`.
//     tt       — a SINGLE TOKEN TREE: one identifier, one literal, one
//                punctuation token, OR one balanced `(...)` / `[...]` /
//                `{...}` group. Most permissive; use as a last resort.
//
// You then refer to a captured chunk inside the expansion as `$name`.
// Critically, the chunk is reinserted as TOKENS — not as a string. So
// fragments interact correctly with the surrounding code (no quoting
// nightmares like in C macros).
//
// HYGIENE (preview, full story in macro4)
//
// Identifiers introduced INSIDE a `macro_rules!` expansion live in the
// macro's own scope, not the caller's. So a macro that does
// `let _tmp = 0;` won't clobber a caller's `_tmp`. C's textual macros
// can't promise this.
//
// SEMICOLONS
//
// Each rule ends with `;` (the last one too — the trailing `;` is fine).
// The `=>` is mandatory.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   1. Define `say_hello!()` — takes NO arguments, expands to
//      `println!("hello")`.
//
//   2. Define `square!(x)` — takes one expression and expands to `($x * $x)`.
//      The parentheses matter: `square!(2 + 3)` should expand to
//      `((2 + 3) * (2 + 3)) == 25`, not `2 + 3 * 2 + 3 == 11`.
//
// Replace each `???`. Don't change the tests.

// I AM NOT DONE

macro_rules! say_hello {
    // Pattern: empty input. Expansion: a `println!` call.
    () => {
        ???
    };
}

macro_rules! square {
    // Pattern: capture one expression as `$x`. Expansion: ($x * $x).
    ( $x:??? ) => {
        ???
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn say_hello_compiles_and_runs() {
        // We can't easily capture stdout here, but the macro must EXPAND
        // to valid code — that alone proves the rule fires.
        say_hello!();
    }

    #[test]
    fn square_basic() {
        assert_eq!(square!(4), 16);
    }

    #[test]
    fn square_groups_its_argument() {
        // If `square!` forgets the parentheses around `$x`, this becomes
        // `2 + 3 * 2 + 3 == 11` and the test fails.
        assert_eq!(square!(2 + 3), 25);
    }

    #[test]
    fn square_works_on_negative() {
        assert_eq!(square!(-7), 49);
    }
}

fn main() {}
