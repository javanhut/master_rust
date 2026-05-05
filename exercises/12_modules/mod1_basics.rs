// =============================================================================
//  mod1 — modules: the inline `mod` block
// =============================================================================
//
// A MODULE is a named container for items: functions, types, constants, other
// modules. Modules give you two things at once:
//
//   1. NAMESPACING — `greeter::hello` and `farewell::hello` can coexist.
//   2. PRIVACY    — items inside a module are PRIVATE by default. Only items
//                   marked `pub` are visible to code outside the module.
//
// THE INLINE FORM
//
//     mod greeter {
//         pub fn hello() -> &'static str { "hi" }
//         fn secret()    -> &'static str { "shh" }   // private — outside code
//     }                                              // cannot call this.
//
// Outside the module, you reach in with the path operator `::` :
//
//     greeter::hello()       // OK
//     greeter::secret()      // ERROR — `secret` is private
//
// THE FILE-BASED FORM (real projects)
//
// In a Cargo project you usually write `mod greeter;` at the top of `lib.rs`
// or `main.rs`, and put the module's contents in `src/greeter.rs` (or
// `src/greeter/mod.rs` for a directory module). The semantics are IDENTICAL
// to the inline form — same privacy rules, same path syntax. The only
// difference is where the source text lives.
//
// THIS COURSE compiles each exercise as a single `.rs` file with `rustc`, so
// every example uses the inline `mod NAME { ... }` form. Treat it as a
// stand-in for "this would normally be its own file."
//
// PRIVACY IS PER-ITEM, NOT PER-FIELD-OF-A-FILE
//
// `pub fn`, `pub struct`, `pub const`, `pub mod` — each item opts in to
// publicity individually. Forgetting the `pub` is the #1 reason a fresh
// module appears to "do nothing" from outside.
//
// CALLING FROM TESTS
//
// Test blocks are themselves a child module: `mod tests { ... }`. To reach
// the parent's items, write:
//
//     #[cfg(test)]
//     mod tests {
//         use super::*;          // bring the FILE'S top-level items into scope
//         // now `greeter` is visible here, and we can call greeter::hello()
//     }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build a tiny inline module called `greeter` with:
//
//   - a PUBLIC function `hello() -> &'static str` returning "hello"
//   - a PUBLIC function `shout(name: &str) -> String` returning
//     `"HELLO, NAME!"` (uppercase the name with `.to_uppercase()`)
//   - a PRIVATE helper `bang(s: &str) -> String` that appends '!' — this is
//     used by `shout` but should NOT be reachable from outside `greeter`.
//
// Then write a free function `front_door() -> String` at the file's top
// level that calls `greeter::shout("world")` and returns the result.

// I AM NOT DONE

mod greeter {
    // PUBLIC — outside callers can use this.
    ??? fn hello() -> &'static str {
        "hello"
    }

    // PUBLIC — but it leans on a PRIVATE helper.
    ??? fn shout(name: &str) -> String {
        let upper = name.???();
        bang(&format!("HELLO, {}", upper))
    }

    // PRIVATE — no `pub`. Only callable from inside this module.
    fn bang(s: &str) -> String {
        format!("{}!", s)
    }
}

fn front_door() -> String {
    // Reach into the module with `::` and call the public function.
    ???::???("world")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn hello_says_hello()       { assert_eq!(greeter::hello(), "hello"); }
    #[test] fn shout_uppercases()       { assert_eq!(greeter::shout("rust"), "HELLO, RUST!"); }
    #[test] fn front_door_uses_shout()  { assert_eq!(front_door(), "HELLO, WORLD!"); }
}

fn main() {}
