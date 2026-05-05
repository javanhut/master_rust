// SOLUTION — mod1_basics

mod greeter {
    pub fn hello() -> &'static str {
        "hello"
    }

    pub fn shout(name: &str) -> String {
        let upper = name.to_uppercase();
        bang(&format!("HELLO, {}", upper))
    }

    fn bang(s: &str) -> String {
        format!("{}!", s)
    }
}

fn front_door() -> String {
    greeter::shout("world")
}

// WHY THIS IS OPTIMAL:
//
//   `pub` is OPT-IN. `hello` and `shout` are part of the module's contract,
//   so they get `pub`. `bang` is an implementation detail — leaving it
//   private means we can rename or delete it without breaking outside
//   callers. That's the whole point of modules: drawing a line between
//   "what we promise" and "how we get the job done."
//
//   Private items are still visible WITHIN the same module, which is why
//   `shout` can call `bang` directly without any path qualification.
//
//   `name.to_uppercase()` allocates a fresh `String`. We hand a `&str` slice
//   of that into `format!` which builds the final string. One owned string
//   in, one owned string out — no surprises.
//
// EQUIVALENT BUT LESS IDIOMATIC:
//
//   pub(crate) fn hello() -> ... { ... }
//     — visible inside the current crate but not exported beyond it. We'll
//       use this in mod3. For a single-file teaching example it's
//       indistinguishable from `pub`, but in a real library it matters.
//
//   fn shout(name: &str) -> String {
//       let mut s = String::from("HELLO, ");
//       s.push_str(&name.to_uppercase());
//       s.push('!');
//       s
//   }
//     — sidesteps the private `bang` helper entirely. Fewer moving parts;
//       the cost is losing the demonstration that PRIVATE items still work
//       perfectly well as building blocks for PUBLIC ones.
//
// FILE-BASED EQUIVALENT:
//
//   In a real Cargo project this same module would live at `src/greeter.rs`
//   and `main.rs` / `lib.rs` would contain only `mod greeter;`. Same
//   privacy rules, same `greeter::shout(...)` call site. The runner here
//   compiles a single file with rustc, so we keep everything inline.
