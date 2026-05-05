// =============================================================================
//  enum3 — methods on enums (`impl` blocks)
// =============================================================================
//
// Just like structs (chapter 9), enums can have methods attached via
// `impl`:
//
//     enum Direction { North, South, East, West }
//
//     impl Direction {
//         fn opposite(&self) -> Direction {
//             match self {
//                 Direction::North => Direction::South,
//                 Direction::South => Direction::North,
//                 Direction::East  => Direction::West,
//                 Direction::West  => Direction::East,
//             }
//         }
//     }
//
//     let d = Direction::North;
//     let o = d.opposite();          // Direction::South
//
// `&self` is shorthand for `self: &Self` — `Self` being the enum type.
// Inside the method, `match self { ... }` matches on the variant.
//
// THREE FLAVOURS OF `self`
//   &self      — read-only borrow (most common)
//   &mut self  — mutating borrow (modify in place)
//   self       — take ownership (consume; returns transformed value)
//
// Pick the lightest one that works. We'll use `&self` here.
//
// ASSOCIATED FUNCTIONS (no `self`)
//   A function in an `impl` block without `self` is called like
//   `Direction::default_dir()` — it's a constructor or helper, not a
//   method on a value.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Given the `Light` enum, implement two methods:
//
//   - `next(&self) -> Light`
//        Red → Green, Green → Yellow, Yellow → Red
//
//   - `is_go(&self) -> bool`
//        true ONLY for Green.
//
// Use `match self` inside each. Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

#[derive(Debug, PartialEq)]
enum Light { Red, Yellow, Green }

impl Light {
    fn next(&self) -> Light {
        match self {
            Light::Red    => ???,
            Light::Green  => ???,
            Light::Yellow => ???,
        }
    }

    fn is_go(&self) -> bool {
        ???
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn cycle() {
        assert_eq!(Light::Red.next(),    Light::Green);
        assert_eq!(Light::Green.next(),  Light::Yellow);
        assert_eq!(Light::Yellow.next(), Light::Red);
    }
    #[test] fn go_only_on_green() {
        assert!(!Light::Red.is_go());
        assert!(!Light::Yellow.is_go());
        assert!( Light::Green.is_go());
    }
}

fn main() {}
