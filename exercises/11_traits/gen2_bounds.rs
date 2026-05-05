// =============================================================================
//  gen2 — multiple trait bounds & the `where` clause
// =============================================================================
//
// Real generic code usually needs MORE THAN ONE trait from a type parameter.
// Two ways to express that:
//
//     // Inline `+` syntax — fine for one or two short bounds:
//     fn show_clone<T: std::fmt::Display + Clone>(x: T) -> String {
//         let copy = x.clone();
//         format!("{}, {}", x, copy)
//     }
//
//     // `where` clause — strongly preferred once bounds get long or
//     // there are multiple type parameters:
//     fn show_clone<T>(x: T) -> String
//     where
//         T: std::fmt::Display + Clone,
//     {
//         let copy = x.clone();
//         format!("{}, {}", x, copy)
//     }
//
// Both compile to identical code. The `where` form keeps the function
// signature line readable and is the Rust style guide's recommendation
// for anything beyond one tiny bound.
//
// COMMON BOUNDS YOU'LL SEE
//
//     Display      — formats with `{}`     (use std::fmt::Display)
//     Debug        — formats with `{:?}`
//     Clone        — `.clone()` is allowed
//     Copy         — bitwise copy on assignment (implies Clone)
//     PartialEq    — `==`, `!=`
//     PartialOrd   — `<`, `<=`, `>`, `>=`
//     Default      — `T::default()`
//     Hash         — usable in HashMap/HashSet keys
//
// MULTIPLE TYPE PARAMETERS
//
//     fn pair_show<A, B>(a: A, b: B) -> String
//     where
//         A: std::fmt::Display,
//         B: std::fmt::Display,
//     {
//         format!("{a} & {b}")
//     }
//
// Each parameter gets its own row. The body can mix and match them.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - `show_clone<T>(x: T) -> String`
//       Bounds: `T: Display + Clone`. Use the WHERE clause form.
//       Body returns `format!("{}, {}", x, x.clone())`.
//
//   - `pair_show<A, B>(a: A, b: B) -> String`
//       Bounds: `A: Display`, `B: Display`. Use the WHERE clause form.
//       Body returns `format!("{a} & {b}")`.
//
//   - `min_then_show<T>(a: T, b: T) -> String`
//       Bounds: `T: Display + PartialOrd`. WHERE clause form.
//       Pick the smaller of the two with `<` and format it with `{}`.

// I AM NOT DONE

use std::fmt::Display;

fn show_clone<T>(x: T) -> String
where
    T: ??? + ???,
{
    let copy = x.clone();
    format!("{}, {}", x, copy)
}

fn pair_show<A, B>(a: A, b: B) -> String
where
    A: ???,
    B: ???,
{
    format!("{a} & {b}")
}

fn min_then_show<T>(a: T, b: T) -> String
where
    T: ??? + ???,
{
    let smaller = if a < b { a } else { b };
    format!("{}", smaller)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn show_clone_int() {
        assert_eq!(show_clone(7), "7, 7");
    }

    #[test] fn show_clone_string() {
        assert_eq!(show_clone(String::from("hi")), "hi, hi");
    }

    #[test] fn pair_show_mixed() {
        // A and B can be different types — that's the whole point.
        assert_eq!(pair_show(1, "two"), "1 & two");
        assert_eq!(pair_show(3.5, true), "3.5 & true");
    }

    #[test] fn min_then_show_works() {
        assert_eq!(min_then_show(10, 3), "3");
        assert_eq!(min_then_show("banana", "apple"), "apple");
    }
}

fn main() {}
