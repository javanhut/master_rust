// SOLUTION — trait1_define

trait Greet {
    fn name(&self) -> &str;

    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

struct Dog {
    name: String,
}

impl Greet for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

// WHY THIS IS OPTIMAL:
//
//   `Greet` has a tiny REQUIRED surface (just `name`) and a single default
//   method (`greeting`) written in terms of that one required primitive.
//   This is the canonical trait shape: minimise what implementors have to
//   write, maximise what they get for free.
//
//   `Dog::name` returns `&self.name` — a borrow into the struct's field,
//   matching the trait signature `fn name(&self) -> &str`. We never
//   allocate to answer `name()`; the only allocation is inside the default
//   `greeting`'s `format!`, which is unavoidable since it builds a new
//   `String`.
//
// EQUIVALENT BUT WORSE:
//
//   trait Greet {
//       fn name(&self) -> &str;
//       fn greeting(&self) -> String;          // no default
//   }
//   impl Greet for Dog {
//       fn name(&self) -> &str { &self.name }
//       fn greeting(&self) -> String {
//           format!("Hello, {}!", self.name())
//       }
//   }
//   Identical behaviour, but every new implementor has to copy-paste the
//   `greeting` body. That is exactly what default methods exist to avoid.
//
// OVERRIDING THE DEFAULT:
//
//   If `Dog` wanted a custom greeting:
//       impl Greet for Dog {
//           fn name(&self) -> &str { &self.name }
//           fn greeting(&self) -> String {
//               format!("Woof! I am {}.", self.name())
//           }
//       }
//   The compiler always prefers the impl-block body over the default.
