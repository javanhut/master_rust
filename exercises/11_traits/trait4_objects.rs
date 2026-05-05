// =============================================================================
//  trait4 — trait objects: `dyn Trait` for runtime polymorphism
// =============================================================================
//
// So far traits + generics give us STATIC DISPATCH: every call site is
// resolved at compile time, the compiler stamps out one specialised copy
// per concrete type. Fast, zero overhead, but every call site burns the
// concrete type into the binary.
//
// Sometimes you want the OPPOSITE — a runtime collection of values whose
// concrete types differ but that all implement the same trait:
//
//     let zoo: Vec<???> = vec![dog, cat, parrot];   // all `Animal`s
//
// You CANNOT write `Vec<Animal>` directly — `Animal` is a trait, not a
// type, and the compiler doesn't know how big each element is. (A `Dog`
// and a `Parrot` may have completely different sizes.)
//
// The fix is a TRAIT OBJECT: a value of type `dyn Animal`. Trait objects
// are unsized (`dyn Animal` could be any size at runtime), so we always
// access them through a pointer — typically `Box<dyn Animal>` for owned
// or `&dyn Animal` for borrowed.
//
//     let zoo: Vec<Box<dyn Animal>> = vec![
//         Box::new(Dog { ... }),
//         Box::new(Cat { ... }),
//         Box::new(Parrot { ... }),
//     ];
//
// Each element is a fat pointer: data pointer + VTABLE pointer. Calling
// `a.speak()` looks up `speak` in the vtable at runtime and calls through
// it. That's DYNAMIC DISPATCH.
//
// STATIC vs DYNAMIC — which do I pick?
//
//     STATIC (`impl Trait` / generics)
//       + zero-cost: calls are direct, often inlined.
//       + you can use any associated method, even generic ones.
//       - one compiled copy per concrete T → larger binaries.
//       - every call site must know the concrete type.
//       Use when: hot loops, libraries, single-type contexts.
//
//     DYNAMIC (`dyn Trait`)
//       + one copy of the function regardless of how many T's call it.
//       + heterogeneous collections (`Vec<Box<dyn T>>`) become possible.
//       + great for plugin-style architectures.
//       - per-call vtable indirection (cheap, but not free, no inlining
//         across the call).
//       - traits must be OBJECT-SAFE (next exercise).
//       Use when: heterogeneous collections, plugin layers, dyn-typed UIs.
//
// SYNTAX RECAP
//
//     fn print_one(a: &dyn Animal)        { a.speak(); }    // borrowed
//     fn print_box(a: Box<dyn Animal>)    { a.speak(); }    // owned
//     fn print_each(zoo: &[Box<dyn Animal>]) {
//         for a in zoo { a.speak(); }
//     }
//
// You write `dyn Animal`, NOT `Animal`, in modern Rust. The keyword is
// mandatory and reminds the reader "this is a trait object".
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// `Animal` is given. `Dog`, `Cat`, and `Parrot` are given.
//
//   - Implement `Animal::speak` for all three so `Dog::speak` returns
//     `"Woof!"`, `Cat::speak` returns `"Meow!"`, `Parrot::speak` returns
//     `"Hello!"`.
//
//   - Write `count_speakers(zoo: &[Box<dyn Animal>]) -> usize`. It returns
//     `zoo.len()`. The point is just that the parameter type compiles —
//     a slice of boxed trait objects. (Hint: `slice.len()`.)
//
//   - Write `all_sounds(zoo: &[Box<dyn Animal>]) -> Vec<String>`. For each
//     boxed animal, push the result of `a.speak()` into the result Vec.

// I AM NOT DONE

trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;
struct Parrot;

impl ??? for Dog {
    fn speak(&self) -> String { String::from("???") }
}

impl ??? for Cat {
    fn speak(&self) -> String { String::from("???") }
}

impl ??? for Parrot {
    fn speak(&self) -> String { String::from("???") }
}

fn count_speakers(zoo: &[Box<dyn Animal>]) -> usize {
    zoo.???()
}

fn all_sounds(zoo: &[Box<dyn Animal>]) -> Vec<String> {
    let mut out = Vec::new();
    for a in zoo {
        // `a` is `&Box<dyn Animal>`. Auto-deref + auto-borrow lets you
        // call `a.speak()` directly.
        out.push(a.???());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_zoo() -> Vec<Box<dyn Animal>> {
        vec![
            Box::new(Dog),
            Box::new(Cat),
            Box::new(Parrot),
        ]
    }

    #[test] fn dog_says_woof()     { assert_eq!(Dog.speak(),    "Woof!");  }
    #[test] fn cat_says_meow()     { assert_eq!(Cat.speak(),    "Meow!");  }
    #[test] fn parrot_says_hello() { assert_eq!(Parrot.speak(), "Hello!"); }

    #[test] fn count_three() {
        assert_eq!(count_speakers(&build_zoo()), 3);
    }

    #[test] fn collected_sounds() {
        assert_eq!(
            all_sounds(&build_zoo()),
            vec!["Woof!", "Meow!", "Hello!"],
        );
    }
}

fn main() {}
