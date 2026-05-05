// SOLUTION — enum3_methods

#[derive(Debug, PartialEq)]
enum Light { Red, Yellow, Green }

impl Light {
    fn next(&self) -> Light {
        match self {
            Light::Red    => Light::Green,
            Light::Green  => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }

    fn is_go(&self) -> bool {
        matches!(self, Light::Green)
    }
}

// WHY THIS IS OPTIMAL:
//
//   `next` returns a fresh `Light` rather than mutating in place. Enums
//   without payloads are tiny (`Light` is one byte) so there's zero
//   reason to reach for `&mut self` or stateful update — pure functions
//   compose better.
//
//   `is_go` uses `matches!(self, Light::Green)` — the textbook one-liner
//   for "is the value this specific variant?" It expands to the same
//   `match` we'd write by hand and gives a `bool` directly.
//
//   `&self` (not `self`) lets callers keep ownership: you can call
//   `light.next()` repeatedly without `light` being consumed.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Free function `fn next(l: &Light) -> Light { ... }`
//     - Compiles fine. But the operation is so tied to the type that
//       method syntax (`light.next()`) reads better and groups in IDEs.
//
//   `fn is_go(&self) -> bool { match self { Light::Green => true, _ => false } }`
//     - Works, but `if cond { true } else { false }`-style code smells.
//       `matches!` is what `match {... => true, _ => false }` compresses
//       to.
//
// SUBTLETY:
//   We `#[derive(PartialEq)]` so the tests can use `assert_eq!`. Without
//   it `==` on `Light` doesn't compile. `Debug` lets `assert_eq!` print
//   the value when a test fails — handy for diagnosis.
