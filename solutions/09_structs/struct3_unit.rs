// SOLUTION — struct3_unit

trait Greet {
    fn hello(&self) -> &'static str;
}

struct English;
struct Pirate;

impl Greet for English {
    fn hello(&self) -> &'static str {
        "Hello"
    }
}

impl Greet for Pirate {
    fn hello(&self) -> &'static str {
        "Ahoy"
    }
}

fn greet_twice<T: Greet>(g: &T) -> String {
    let h = g.hello();
    format!("{}, {}!", h, h)
}

const ENGLISH_SIZE: usize = std::mem::size_of::<English>();
const PIRATE_SIZE:  usize = std::mem::size_of::<Pirate>();
const EXPECTED_SIZE: usize = 0;

// WHY THIS IS OPTIMAL:
//
//   Unit structs are the right tool whenever a trait impl is the entire
//   reason a type needs to exist — a "tag in the type system". `English` and
//   `Pirate` carry no data, occupy no memory, and the compiler can resolve
//   `g.hello()` statically when the concrete type is known. There is no
//   runtime cost over a plain function call.
//
//   Returning `&'static str` (a string slice baked into the binary) is the
//   correct shape for a fixed greeting — there is no allocation. We only
//   reach for `String` in `greet_twice` because we genuinely build a NEW
//   string at runtime.
//
//   `std::mem::size_of::<T>()` is a `const fn` that lets us encode the
//   "zero-sized" guarantee directly in the test.
//
// EQUIVALENT BUT NOISIER:
//
//   struct English {}
//   struct Pirate  {}
//   The empty-braces form is also zero-sized and also legal. Most Rustaceans
//   prefer `struct English;` for true marker types — fewer characters, and
//   it signals "no fields, ever".
//
//   fn greet_twice(g: &dyn Greet) -> String { ... }
//   Using a trait object instead of a generic works, but it pays for a vtable
//   lookup. For tiny marker types and statically known callers, the generic
//   form monomorphises to the same code as a direct call.
