// =============================================================================
//  struct3 — unit structs (zero-sized marker types)
// =============================================================================
//
// A UNIT STRUCT has no fields at all:
//
//     struct Marker;
//
// It is a type with exactly ONE value (also written `Marker`). It occupies
// ZERO bytes at runtime — the compiler knows there is nothing to store.
//
// Why would anyone want such a thing?
//
//   - As a TYPE-LEVEL TAG that lets you implement traits on something
//     specific without dragging data along. (Traits get a full chapter
//     later — for now think of a trait as "a thing my type can do".)
//
//   - As a PHANTOM / MARKER inside generic code, to distinguish otherwise
//     identical types.
//
// Quick taste — using a unit struct as a marker that implements a trait:
//
//     trait Greet {
//         fn hello(&self) -> &'static str;
//     }
//
//     struct English;
//     struct Pirate;
//
//     impl Greet for English { fn hello(&self) -> &'static str { "Hello"  } }
//     impl Greet for Pirate  { fn hello(&self) -> &'static str { "Ahoy"   } }
//
// `English` and `Pirate` carry no data. They exist only so the type system
// has somewhere to attach `impl Greet for ...`. The values `English` and
// `Pirate` cost nothing — `std::mem::size_of::<English>()` is `0`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Two unit structs `English` and `Pirate` already exist in the skeleton.
// Make them implement the provided `Greet` trait so:
//   - `English.hello()` returns "Hello"
//   - `Pirate.hello()`  returns "Ahoy"
// Also implement `greet_twice` to call `.hello()` on whichever value it gets,
// and complete the `size_check` constant to assert the structs are zero-sized.

// I AM NOT DONE

trait Greet {
    fn hello(&self) -> &'static str;
}

struct English;
struct Pirate;

impl Greet for English {
    fn hello(&self) -> &'static str {
        ???
    }
}

impl Greet for Pirate {
    fn hello(&self) -> &'static str {
        ???
    }
}

// Takes any `T` that knows how to `Greet`. Returns "X, X!" — e.g. "Hello, Hello!".
fn greet_twice<T: Greet>(g: &T) -> String {
    let h = g.???();
    format!("{}, {}!", h, h)
}

// Both unit structs should be zero-sized. Fill in the right number.
const ENGLISH_SIZE: usize = std::mem::size_of::<English>();
const PIRATE_SIZE:  usize = std::mem::size_of::<Pirate>();
const EXPECTED_SIZE: usize = ???;

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn english_says_hello() { assert_eq!(English.hello(), "Hello"); }
    #[test] fn pirate_says_ahoy()   { assert_eq!(Pirate.hello(),  "Ahoy");  }
    #[test] fn greet_twice_english() {
        assert_eq!(greet_twice(&English), "Hello, Hello!");
    }
    #[test] fn greet_twice_pirate() {
        assert_eq!(greet_twice(&Pirate), "Ahoy, Ahoy!");
    }
    #[test] fn unit_structs_are_zero_sized() {
        assert_eq!(ENGLISH_SIZE, EXPECTED_SIZE);
        assert_eq!(PIRATE_SIZE,  EXPECTED_SIZE);
    }
}

fn main() {}
