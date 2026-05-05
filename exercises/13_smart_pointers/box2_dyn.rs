// =============================================================================
//  box2 — Box<dyn Trait>: trait objects on the heap
// =============================================================================
//
// Chapter 11 introduced trait objects via `&dyn Trait`. That works when
// the borrow is short-lived. When you want OWNED, heterogeneous values
// stored together — for example, a `Vec` of "any animal" — you need
// `Box<dyn Trait>`.
//
// THE PROBLEM
//
// A `Vec<T>` holds elements of one concrete type `T`. You can't write
//
//     let zoo: Vec<???> = vec![Dog { ... }, Cat { ... }];
//
// because `Dog` and `Cat` are different types and a Vec is monomorphic.
// Generics (`Vec<A: Animal>`) wouldn't help either: the type parameter
// gets pinned to one concrete type at instantiation.
//
// THE FIX
//
//     let zoo: Vec<Box<dyn Animal>> = vec![
//         Box::new(Dog { name: "Rex".into() }),
//         Box::new(Cat { name: "Mim".into() }),
//     ];
//
// Each `Box<dyn Animal>` is a TWO-WORD pointer: one word to the data on
// the heap, one word to the VTABLE (a static table of function pointers
// for the trait's methods). The Vec stores those fat pointers; calling
// `animal.speak()` looks the function up in the vtable at runtime
// (DYNAMIC DISPATCH).
//
// `dyn Trait` is an UNSIZED type — its size depends on the concrete
// thing behind it, which is unknown at compile time. That's why you
// always see it behind a pointer: `&dyn Trait`, `&mut dyn Trait`,
// `Box<dyn Trait>`, `Rc<dyn Trait>`, etc. The pointer is sized; the
// pointee can be anything that implements the trait.
//
// COMPARISON
//
//     impl Trait        — static dispatch, one concrete type per call site,
//                         no vtable, fastest. Only one type per return.
//     dyn Trait         — dynamic dispatch via vtable, allows mixed types
//                         in one collection, costs one indirect call.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define a `Dog` struct with `name: String` and a `Cat` struct with
//     `name: String`. (Done for you below.)
//   - Implement `Animal` for both: `Dog` says "<name>: woof", `Cat` says
//     "<name>: meow".
//   - Finish `make_zoo` to return a Vec<Box<dyn Animal>> with one Dog
//     named "Rex" and one Cat named "Mim".
//   - `speak_all(animals)` collects every `speak()` into a Vec<String>.

// I AM NOT DONE

pub trait Animal {
    fn speak(&self) -> String;
}

pub struct Dog { pub name: String }
pub struct Cat { pub name: String }

impl Animal for Dog {
    fn speak(&self) -> String {
        ???
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        ???
    }
}

pub fn make_zoo() -> Vec<Box<dyn Animal>> {
    // Hint: Box::new(Dog { name: "Rex".into() }) is a Box<Dog>.
    // The compiler will COERCE Box<Dog> -> Box<dyn Animal> when
    // assigning into a Vec<Box<dyn Animal>>.
    vec![
        ???,
        ???,
    ]
}

pub fn speak_all(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals.iter().map(|a| a.???()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn dog_speaks() {
        let d = Dog { name: "Rex".into() };
        assert_eq!(d.speak(), "Rex: woof");
    }

    #[test] fn cat_speaks() {
        let c = Cat { name: "Mim".into() };
        assert_eq!(c.speak(), "Mim: meow");
    }

    #[test] fn zoo_has_two() {
        let zoo = make_zoo();
        assert_eq!(zoo.len(), 2);
    }

    #[test] fn speak_all_collects() {
        let zoo = make_zoo();
        let voices = speak_all(&zoo);
        assert_eq!(voices, vec!["Rex: woof", "Mim: meow"]);
    }

    #[test] fn heterogeneous_vec_compiles() {
        // The whole point: different concrete types in one Vec.
        let mixed: Vec<Box<dyn Animal>> = vec![
            Box::new(Cat { name: "A".into() }),
            Box::new(Dog { name: "B".into() }),
            Box::new(Cat { name: "C".into() }),
        ];
        assert_eq!(mixed.len(), 3);
    }
}

fn main() {}
