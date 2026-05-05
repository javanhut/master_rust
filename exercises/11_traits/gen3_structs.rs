// =============================================================================
//  gen3 — generic structs and conditional `impl` blocks
// =============================================================================
//
// Structs can be generic over types too. The classic example is a pair:
//
//     struct Pair<T> {
//         a: T,
//         b: T,
//     }
//
// `T` is a TYPE PARAMETER on the struct. `Pair<i32>` and `Pair<String>`
// are different concrete types, each laid out separately in memory.
//
// IMPL BLOCKS FOR GENERIC STRUCTS
//
// To attach methods, you ALSO declare the type parameter on the impl:
//
//     impl<T> Pair<T> {
//         fn new(a: T, b: T) -> Self {
//             Self { a, b }
//         }
//     }
//
// Read it left-to-right: "for any T, here are methods on Pair<T>". The
// `<T>` after `impl` introduces the parameter; the `<T>` after `Pair` uses
// it. They MUST agree.
//
// CONDITIONAL IMPLS — methods that exist ONLY when T meets a bound
//
//     impl<T: std::fmt::Display> Pair<T> {
//         fn print_pair(&self) {
//             println!("({}, {})", self.a, self.b);
//         }
//     }
//
// `print_pair` only exists for those `Pair<T>` where `T: Display`. A
// `Pair<NonDisplayThing>` won't have the method, and the compiler will
// tell you so at the CALL site, not at the definition. This is exactly
// how `Vec<T>::sort()` is conditional on `T: Ord`.
//
// MULTIPLE IMPL BLOCKS ARE FINE
//
// You can split methods across as many `impl` blocks as you like — one
// per bound is a common idiom. The compiler glues them all together for
// the type.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - Define `Pair<T>` with two fields `a: T, b: T`.
//
//   - In an UNCONDITIONAL `impl<T> Pair<T>` block, write:
//         fn new(a: T, b: T) -> Self     — build a Pair.
//         fn into_tuple(self) -> (T, T)  — consume Pair, return (a, b).
//
//   - In a CONDITIONAL `impl<T: Display> Pair<T>` block, write:
//         fn show(&self) -> String       — return format!("({}, {})", a, b).
//
// `T: Display` is a bound on the impl, not on the struct itself — so
// `Pair<T>` exists for any T, but `.show()` only exists when T: Display.

// I AM NOT DONE

use std::fmt::Display;

struct Pair<T> {
    a: ???,
    b: ???,
}

impl<???> Pair<???> {
    fn new(a: T, b: T) -> Self {
        Self { ??? }
    }

    fn into_tuple(self) -> (T, T) {
        (???, ???)
    }
}

impl<T: ???> Pair<T> {
    fn show(&self) -> String {
        format!("({}, {})", ???, ???)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn new_and_into_tuple_int() {
        let p: Pair<i32> = Pair::new(1, 2);
        assert_eq!(p.into_tuple(), (1, 2));
    }

    #[test] fn new_and_into_tuple_string() {
        let p: Pair<String> = Pair::new(String::from("hi"), String::from("bye"));
        let (a, b) = p.into_tuple();
        assert_eq!(a, "hi");
        assert_eq!(b, "bye");
    }

    #[test] fn show_when_display() {
        let p = Pair::new(3, 4);
        assert_eq!(p.show(), "(3, 4)");
    }

    #[test] fn show_for_strings() {
        // String implements Display, so the conditional impl applies.
        let p = Pair::new(String::from("x"), String::from("y"));
        assert_eq!(p.show(), "(x, y)");
    }
}

fn main() {}
