// =============================================================================
//  trait3 — `#[derive(...)]`: free trait impls the compiler writes for you
// =============================================================================
//
// You've seen `#[derive(Debug)]` already. `derive` is how you ask the
// compiler to GENERATE a trait impl for your type. The body is
// mechanically determined by the type's structure — field by field for
// structs, variant by variant for enums.
//
// THE BIG SIX (the ones you reach for daily)
//
//     #[derive(Debug)]      // {:?} formatting
//     #[derive(Clone)]      // .clone() — explicit duplication
//     #[derive(Copy)]       // bitwise copy on assignment (also requires Clone)
//     #[derive(PartialEq)]  // == and !=
//     #[derive(Eq)]         // marker: PartialEq is total (no NaN-style holes)
//     #[derive(Hash)]       // can be hashed — required for HashMap/HashSet keys
//     #[derive(Default)]    // T::default()
//
// All of them require that EVERY FIELD also implements the same trait.
// `#[derive(Clone)]` on a struct containing a `String` works because
// `String: Clone`. Try it on a struct containing a raw pointer or a
// non-Clone closure and the derive will fail.
//
// PARTIALEQ vs EQ
//
//   `PartialEq` says "==/!= are defined" but allows partial equivalences:
//   `f64::NAN != f64::NAN` is famously true, so f64 is PartialEq but NOT
//   Eq. `Eq` is a MARKER that promises full reflexivity. HashMap/HashSet
//   keys require `Eq + Hash` (not just PartialEq) because they assume that
//   `x == x` always holds.
//
// WHEN derive ISN'T ENOUGH
//
// If you need a custom equality (e.g. case-insensitive strings) or a
// custom Debug, write the impl by hand instead of `#[derive(...)]`. We
// stick with derive in this exercise.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Below is a `Tag` struct meant to be used as a key in a `HashSet`. To put
// a value into a `HashSet<T>`, the type T must satisfy: `Eq + Hash`.
//
// To clone a tag for the test, T must be `Clone`. To compare two tags with
// `assert_eq!`, T must be `PartialEq + Debug`. To call `Tag::default()`,
// T must be `Default`.
//
// Add ONE `#[derive(...)]` line above `Tag` that derives EXACTLY:
//
//     Debug, Clone, PartialEq, Eq, Hash, Default
//
// Order doesn't matter; spelling does.

// I AM NOT DONE

use std::collections::HashSet;

// Add the derive attribute on the line right above `struct Tag`.
???
struct Tag {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn debug_works() {
        // {:?} requires Debug.
        let t = Tag { name: String::from("rust") };
        let s = format!("{:?}", t);
        assert!(s.contains("rust"));
    }

    #[test] fn clone_and_eq_work() {
        let t = Tag { name: String::from("rust") };
        let u = t.clone();          // requires Clone
        assert_eq!(t, u);           // requires PartialEq + Debug
    }

    #[test] fn default_is_empty() {
        let t = Tag::default();     // requires Default
        assert_eq!(t.name, "");
    }

    #[test] fn hashset_membership() {
        // HashSet<Tag> requires Tag: Eq + Hash.
        let mut s: HashSet<Tag> = HashSet::new();
        s.insert(Tag { name: String::from("rust") });
        s.insert(Tag { name: String::from("rust") });   // duplicate
        s.insert(Tag { name: String::from("ferris") });
        assert_eq!(s.len(), 2);
        assert!(s.contains(&Tag { name: String::from("rust") }));
    }
}

fn main() {}
