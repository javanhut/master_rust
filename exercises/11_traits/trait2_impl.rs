// =============================================================================
//  trait2 — implementing a trait for multiple types
// =============================================================================
//
// A trait gets its value when MORE THAN ONE type implements it. The whole
// point is "code that doesn't care which concrete type it has, as long as
// the type satisfies the contract".
//
// SYNTAX REMINDER
//
//     impl TraitName for TypeName {
//         fn required_method(&self) -> ... { ... }
//     }
//
// You write one `impl ... for ...` block per (Trait, Type) pair. The same
// trait can be implemented for as many types as you like; the same type can
// implement as many traits as you like.
//
// THE ORPHAN RULE (so you're not surprised later)
//
// To `impl SomeTrait for SomeType`, at least ONE of `SomeTrait` or
// `SomeType` must be defined in your crate. You cannot implement
// `std::fmt::Display` for `Vec<T>` — both belong to other crates. This
// keeps trait coherence sane: only one impl can ever exist for a given
// (trait, type) pair globally.
//
// PICKING THE RIGHT RECEIVER
//
// `&self` is the default — it borrows the value, doesn't mutate it, and
// composes with everything. Use `&mut self` if the trait method must
// modify state, and `self` if it must consume the value. For `Greet`,
// `name(&self) -> &str` is right: we're only inspecting the value.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// `Greet` is given (same shape as trait1). Implement it for TWO types:
//
//   - `Cat`, which has a `name: String`. It should NOT override `greeting`.
//
//   - `Robot`, which has a `model: String`. Robot DOES override `greeting`
//     to return `format!("BEEP. UNIT {} ONLINE.", self.name())`. (Yes,
//     uppercase — the test checks the exact string.)
//
// `Robot::name` should just return `&self.model`.

// I AM NOT DONE

trait Greet {
    fn name(&self) -> &str;
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

struct Cat {
    name: String,
}

struct Robot {
    model: String,
}

impl Greet for Cat {
    fn name(&self) -> &str {
        ???
    }
    // Cat uses the default greeting — do not write one here.
}

impl Greet for Robot {
    fn name(&self) -> &str {
        ???
    }

    // Override the default for Robot:
    fn greeting(&self) -> String {
        format!("BEEP. UNIT {} ONLINE.", ???)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn cat_uses_default() {
        let c = Cat { name: String::from("Mittens") };
        assert_eq!(c.greeting(), "Hello, Mittens!");
    }

    #[test] fn robot_overrides_default() {
        let r = Robot { model: String::from("R2D2") };
        assert_eq!(r.name(), "R2D2");
        assert_eq!(r.greeting(), "BEEP. UNIT R2D2 ONLINE.");
    }
}

fn main() {}
