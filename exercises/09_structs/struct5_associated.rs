// =============================================================================
//  struct5 — associated functions, `Self::new`, the `Self` keyword
// =============================================================================
//
// An ASSOCIATED FUNCTION lives inside `impl Foo` but does NOT take a `self`
// parameter. It is called with the path syntax, not the dot syntax:
//
//     impl Foo {
//         fn new() -> Foo { ... }
//     }
//
//     let f = Foo::new();        // ← path syntax, double colon
//
// The convention in Rust is to provide a constructor named `new` that
// returns a fully-initialised instance. There is no built-in `new` keyword;
// `Counter::new` is just a regular function that happens to be named `new`.
//
// THE `Self` KEYWORD
//
// Inside an `impl Foo` block, `Self` (capital S) is an alias for `Foo`.
// It saves typing and — more importantly — means you can rename the type
// later without rewriting every method body:
//
//     impl Counter {
//         fn new() -> Self {            // == `-> Counter`
//             Self { value: 0 }         // == `Counter { value: 0 }`
//         }
//
//         fn starting_at(value: u32) -> Self {
//             Self { value }
//         }
//     }
//
// CONSTRUCTOR PATTERNS
//
//   `Self::new()`            — the canonical zero-config constructor.
//   `Self::with_xxx(...)`    — alternative constructors that accept config.
//   `Self::default()`        — provided by the `Default` trait (later).
//
// You can have as MANY constructors as you like — they are just functions.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build the `Counter` API:
//   - `Counter::new()`                — starts at 0.
//   - `Counter::starting_at(n)`       — starts at n.
//   - `Counter::pair()`               — returns a TUPLE of two Counters,
//                                       both starting at 0. Use `Self`.
//   - `get(&self) -> u32`             — read the value.
//
// Use `Self` (not `Counter`) inside the impl block.

// I AM NOT DONE

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> ??? {
        ??? { value: 0 }
    }

    fn starting_at(value: u32) -> Self {
        Self { ??? }
    }

    fn pair() -> (Self, Self) {
        (Self::???(), Self::???())
    }

    fn get(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn new_starts_at_zero() {
        assert_eq!(Counter::new().get(), 0);
    }
    #[test] fn starting_at_works() {
        assert_eq!(Counter::starting_at(42).get(), 42);
    }
    #[test] fn pair_returns_two_zero_counters() {
        let (a, b) = Counter::pair();
        assert_eq!(a.get(), 0);
        assert_eq!(b.get(), 0);
    }
    #[test] fn new_equals_starting_at_zero() {
        assert_eq!(Counter::new(), Counter::starting_at(0));
    }
}

fn main() {}
