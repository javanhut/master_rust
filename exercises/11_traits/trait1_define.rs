// =============================================================================
//  trait1 — defining a trait (required vs. default methods)
// =============================================================================
//
// A TRAIT is a named set of behaviours. "If your type wants to claim it
// implements `Greet`, it must provide these methods." Traits are how Rust
// expresses shared interfaces — the rough analogue of interfaces in Java/C#
// or abstract base classes in Python, but checked at compile time and with
// zero runtime overhead by default.
//
// SHAPE OF A TRAIT
//
//     trait Greet {
//         // Required method — only the SIGNATURE is given.
//         // Every implementor must supply a body.
//         fn name(&self) -> &str;
//
//         // Default method — has a BODY here. Implementors may keep it
//         // OR override it.
//         fn greeting(&self) -> String {
//             format!("Hello, {}!", self.name())
//         }
//     }
//
// Two key ideas:
//
//   - REQUIRED methods are the trait's contract. No default body is given,
//     so implementors are forced to provide one. Use these for the few
//     primitives that genuinely vary per type.
//
//   - DEFAULT methods are written ONCE in terms of the required ones. Every
//     implementor gets them for free, but can override when it has a faster
//     or more specific implementation. This is how traits scale: a small
//     number of required methods, a large number of default conveniences.
//
// CALLING TRAIT METHODS
//
// Trait methods use the same dot syntax as inherent methods:
//
//     let d: Dog = ...;
//     d.name();        // calls the impl on Dog
//     d.greeting();    // calls the default body — which calls d.name()
//
// You only need to bring the trait INTO SCOPE (`use some::path::Greet;`)
// when its methods aren't already in the prelude.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Define the `Greet` trait below.
//
//   - Required method:  `name(&self) -> &str`
//                       (no body — just the signature followed by `;`)
//
//   - Default method:   `greeting(&self) -> String`
//                       Body returns `format!("Hello, {}!", self.name())`.
//
// Then implement `Greet` for `Dog` so the tests pass. The Dog impl should
// NOT override `greeting` — the whole point is that it inherits the default.

// I AM NOT DONE

trait Greet {
    // Required: no body, just the signature ending in `;`
    fn name(&self) -> ???;

    // Default: a body the implementor may override.
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.???())
    }
}

struct Dog {
    name: String,
}

impl ??? for Dog {
    fn name(&self) -> &str {
        ???
    }
    // No `greeting` here on purpose — Dog uses the default.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn dog_name() {
        let d = Dog { name: String::from("Rex") };
        assert_eq!(d.name(), "Rex");
    }

    #[test] fn dog_uses_default_greeting() {
        let d = Dog { name: String::from("Rex") };
        assert_eq!(d.greeting(), "Hello, Rex!");
    }
}

fn main() {}
