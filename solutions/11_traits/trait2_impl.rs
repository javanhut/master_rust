// SOLUTION — trait2_impl

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
        &self.name
    }
}

impl Greet for Robot {
    fn name(&self) -> &str {
        &self.model
    }

    fn greeting(&self) -> String {
        format!("BEEP. UNIT {} ONLINE.", self.name())
    }
}

// WHY THIS IS OPTIMAL:
//
//   Each impl block does the minimum its type needs:
//
//     - Cat only has to point `name` at its `name` field; the default
//       `greeting` already does the right thing for the friendly case.
//
//     - Robot's user-facing string is genuinely different, so it overrides
//       `greeting`. Note that the override STILL calls `self.name()` — we
//       reuse the required method rather than reaching into `self.model`
//       directly. Future Robot variants (renames, prefixes) will route
//       through `name()` automatically.
//
//   The fact that `Cat` and `Robot` share the trait but diverge only on
//   ONE method is exactly the win default methods give you.
//
// COMMON MISTAKE — impl block for the wrong field:
//
//     impl Greet for Robot {
//         fn name(&self) -> &str { &self.name }   // ← Robot has no `name`!
//     }
//
//   The compiler error here is "no field `name` on type `Robot`". Pick the
//   field that actually carries the identifier; the trait method's NAME
//   doesn't have to match a struct field's name.
//
// ORPHAN-RULE NOTE:
//
//   Both `Greet` and `Cat`/`Robot` are local to this crate, so the impls
//   are uncontroversial. If `Greet` lived in another crate, you could
//   still impl it for your own types. If both `Greet` and `Cat` lived in
//   other crates, the orphan rule would forbid the impl — you'd need a
//   newtype wrapper (`struct MyCat(OtherCrate::Cat)`) to escape.
