// SOLUTION — box2_dyn

pub trait Animal {
    fn speak(&self) -> String;
}

pub struct Dog { pub name: String }
pub struct Cat { pub name: String }

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{}: woof", self.name)
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{}: meow", self.name)
    }
}

pub fn make_zoo() -> Vec<Box<dyn Animal>> {
    vec![
        Box::new(Dog { name: "Rex".into() }),
        Box::new(Cat { name: "Mim".into() }),
    ]
}

pub fn speak_all(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals.iter().map(|a| a.speak()).collect()
}

// WHY THIS IS OPTIMAL:
//
//   `Box<dyn Animal>` is the canonical owned trait object. Each entry in
//   the Vec is a fat pointer — (data ptr, vtable ptr) — so we can mix
//   `Dog` and `Cat` (and any future implementor) in one collection.
//
//   The conversion `Box<Dog> -> Box<dyn Animal>` happens by UNSIZING
//   COERCION at the `vec!` macro's element slot, because the Vec's
//   target element type is `Box<dyn Animal>`. You don't need to write
//   `as Box<dyn Animal>` — the compiler handles it.
//
//   `format!("{}: woof", self.name)` borrows `self.name` (a String) via
//   its `Display` impl. We allocate a fresh owned String for the return.
//
//   `speak_all` takes `&[Box<dyn Animal>]` and returns `Vec<String>`.
//   Inside the closure, `a` has type `&Box<dyn Animal>`; calling
//   `a.speak()` auto-derefs through the Box and dispatches dynamically
//   via the vtable.
//
// DYNAMIC DISPATCH — WHAT IT COSTS
//
//   Every call to `a.speak()` is an INDIRECT call: load the vtable
//   pointer, load the function pointer, jump. That's a few extra cycles
//   per call and (more importantly) blocks inlining. For most code this
//   is invisible; for hot inner loops, prefer `impl Trait` / generics so
//   the compiler can monomorphize and inline.
//
// ALTERNATIVES:
//
//   1. An ENUM dispatch:
//
//          enum AnyAnimal { Dog(Dog), Cat(Cat) }
//          impl AnyAnimal {
//              fn speak(&self) -> String { match self { ... } }
//          }
//
//      Static dispatch, no heap allocation per element, no vtable —
//      faster and simpler when you control all variants. But it CLOSES
//      the world: adding `Hamster` means editing the enum. `Box<dyn
//      Trait>` keeps the world OPEN — third-party crates can implement
//      Animal for their own types.
//
//   2. `Rc<dyn Animal>` / `Arc<dyn Animal>` when the trait object needs
//      to be shared (multiple owners) instead of owned uniquely.
//
//   3. `&dyn Animal` when you don't need ownership — cheaper, but borrow
//      lifetime constrains where the slice can travel.
