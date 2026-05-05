// =============================================================================
//  intro3 — variables and immutability
// =============================================================================
//
// In Rust, variable bindings introduced with `let` are IMMUTABLE BY DEFAULT.
// This is the opposite of most other languages and is one of Rust's most
// important design choices. The reason: the compiler can reason much more
// powerfully about code when it knows a value cannot change underneath it.
//
//     let x = 10;
//     x = 11;        // ❌ compile error: cannot assign twice to immutable `x`
//
// TYPE INFERENCE
// ──────────────
// Rust figures out the type from the value on the right (and any later use).
// You only need to write a type when:
//   - the compiler genuinely cannot tell (e.g. `parse`),
//   - or you want to override the default (e.g. force `u8` instead of `i32`).
//
//     let x = 10;          // x: i32  (default integer type)
//     let y = 10u64;       // y: u64  (suffix on the literal)
//     let z: f32 = 1.0;    // z: f32  (type annotation on the binding)
//
// SCOPE
// ─────
// A `let` binding lives until the end of the enclosing `{ ... }` block.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - Make `area_of_square` return the correct area without using `mut`.
//     (You can compute the answer in a single expression.)
//   - Make `tests` pass.

// I AM NOT DONE

fn area_of_square(side: i32) -> i32 {
    // Currently returns a wrong, hard-coded value. Fix it.
    // HINT: the last expression of a block (no `;`) is what the function returns.
    let side = side; // we are not allowed to mutate
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_square() {
        assert_eq!(area_of_square(3), 9);
    }

    #[test]
    fn larger_square() {
        assert_eq!(area_of_square(12), 144);
    }
}

fn main() {} // tests are run instead of main when mode = "test"
