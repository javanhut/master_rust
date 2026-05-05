// =============================================================================
//  enum2 — `match` on an enum
// =============================================================================
//
// `match` is Rust's pattern-matching control-flow construct. On an enum it's
// the canonical way to "case on which variant we have AND pull out the
// payload at the same time":
//
//     match light {
//         TrafficLight::Red    => stop(),
//         TrafficLight::Yellow => slow(),
//         TrafficLight::Green  => go(),
//     }
//
// Every arm is `PATTERN => EXPRESSION,`. The match itself is an expression
// — its value is the value of whichever arm fired — so you can assign it:
//
//     let action = match light { ... };
//
// EXHAUSTIVENESS — THE BIG WIN
// ────────────────────────────
// Rust REFUSES to compile a `match` that doesn't cover every possible
// variant. Add a new variant to the enum a year from now and the compiler
// will point at every match that needs updating. This single feature
// catches a massive class of bugs that other languages let you ship.
//
//     match light {
//         TrafficLight::Red => ...,
//         TrafficLight::Green => ...,
//         // ❌ error: non-exhaustive patterns: `Yellow` not covered
//     }
//
// THE WILDCARD `_`
// ────────────────
// If you genuinely don't care about the remaining variants, use `_`:
//
//     match n {
//         0 => "zero",
//         1 => "one",
//         _ => "many",      // catches everything else
//     }
//
// Use `_` SPARINGLY on enums you own — it disables the new-variant
// warning. On values from external crates you can't extend, `_` is fine.
//
// MATCH ARMS RETURN THE SAME TYPE
//   All arms must produce the same type. `match x { 0 => "zero", _ => 1 }`
//   is a type error.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Given the `Coin` enum below, implement `value_in_cents` returning:
//   Penny → 1, Nickel → 5, Dime → 10, Quarter → 25.
//
// Then implement `is_copper` which returns `true` ONLY for `Penny` and uses
// the `_` wildcard for every other variant.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

enum Coin { Penny, Nickel, Dime, Quarter }

fn value_in_cents(c: &Coin) -> u32 {
    match c {
        Coin::Penny   => ???,
        Coin::Nickel  => ???,
        Coin::Dime    => ???,
        Coin::Quarter => ???,
    }
}

fn is_copper(c: &Coin) -> bool {
    match c {
        Coin::Penny => ???,
        ??? => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn values() {
        assert_eq!(value_in_cents(&Coin::Penny),   1);
        assert_eq!(value_in_cents(&Coin::Nickel),  5);
        assert_eq!(value_in_cents(&Coin::Dime),   10);
        assert_eq!(value_in_cents(&Coin::Quarter),25);
    }
    #[test] fn copper() {
        assert!( is_copper(&Coin::Penny));
        assert!(!is_copper(&Coin::Nickel));
        assert!(!is_copper(&Coin::Dime));
        assert!(!is_copper(&Coin::Quarter));
    }
}

fn main() {}
