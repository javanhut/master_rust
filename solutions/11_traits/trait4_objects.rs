// SOLUTION — trait4_objects

trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;
struct Parrot;

impl Animal for Dog {
    fn speak(&self) -> String { String::from("Woof!") }
}

impl Animal for Cat {
    fn speak(&self) -> String { String::from("Meow!") }
}

impl Animal for Parrot {
    fn speak(&self) -> String { String::from("Hello!") }
}

fn count_speakers(zoo: &[Box<dyn Animal>]) -> usize {
    zoo.len()
}

fn all_sounds(zoo: &[Box<dyn Animal>]) -> Vec<String> {
    let mut out = Vec::new();
    for a in zoo {
        out.push(a.speak());
    }
    out
}

// WHY THIS IS OPTIMAL:
//
//   `Vec<Box<dyn Animal>>` is the canonical heterogeneous-collection
//   shape: each element is a fat pointer (data + vtable), all elements
//   are the same SIZE in the Vec, and we keep the freedom to mix concrete
//   animal types. The Box owns its inner value; dropping the Vec drops
//   each Box, which drops its inner Dog/Cat/Parrot.
//
//   Inside `all_sounds`, `a` has type `&Box<dyn Animal>`. Method-call
//   syntax peels both layers automatically: `Box` derefs to `dyn Animal`,
//   then auto-borrow gives us `&dyn Animal` for the `&self` call. This is
//   why dyn Trait feels like normal OO at the call site even though
//   under the hood every call goes through a vtable lookup.
//
// STATIC ALTERNATIVE (and why it's wrong here):
//
//     fn all_sounds<A: Animal>(zoo: &[A]) -> Vec<String> { ... }
//
//   This compiles, but `A` must be ONE concrete type — the slice has to
//   be all-Dogs or all-Cats. It cannot mix. The whole reason we reach
//   for `dyn` is to keep three different types in one collection.
//
// TRADE-OFFS RECAP:
//
//   - Static dispatch (impl Trait, generics): zero-cost calls, monomorphic.
//     Best for hot loops over homogeneous types.
//   - Dynamic dispatch (dyn Trait): vtable lookup per call, one compiled
//     copy regardless of T. Best for plugin layers and heterogeneous
//     collections.
//
// FAT-POINTER SHAPE:
//
//   sizeof(&Dog)            = 1 word  (just the data pointer; Dog is ZST)
//   sizeof(&dyn Animal)     = 2 words (data + vtable)
//   sizeof(Box<dyn Animal>) = 2 words (same shape, owned)
//
//   That second word is a pointer to a vtable: a small table containing
//   `speak`'s function pointer (and drop, size, align). The compiler
//   builds one vtable per (Type, Trait) pair and reuses it for every
//   trait object of that pair.
//
// EQUIVALENT ITERATOR FORM:
//
//   fn all_sounds(zoo: &[Box<dyn Animal>]) -> Vec<String> {
//       zoo.iter().map(|a| a.speak()).collect()
//   }
//
//   Cleaner, but iterators are chapter 7 — the explicit for-push form is
//   easier to read for the lesson.
