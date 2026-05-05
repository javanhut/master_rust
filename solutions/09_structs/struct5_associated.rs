// SOLUTION — struct5_associated

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn starting_at(value: u32) -> Self {
        Self { value }
    }

    fn pair() -> (Self, Self) {
        (Self::new(), Self::new())
    }

    fn get(&self) -> u32 {
        self.value
    }
}

// WHY THIS IS OPTIMAL:
//
//   `Self` instead of `Counter` is the convention inside `impl` blocks for
//   one big reason: refactoring. If you ever rename `Counter` to
//   `Tally`, every method body is already correct — only the `struct`
//   declaration and the `impl Counter` header need to change.
//
//   `Self::new()` reuses the canonical constructor instead of duplicating
//   `Self { value: 0 }`. If "starts at 0" later means "starts at 0 AND
//   registers with a global metric", `pair` automatically picks up the
//   change. Calling other associated functions through `Self::` is the
//   normal way to share construction logic.
//
//   `starting_at` uses field-init shorthand because the parameter is named
//   `value`, the same as the field.
//
// EQUIVALENT BUT NOISIER:
//
//   impl Counter {
//       fn new() -> Counter {
//           Counter { value: 0 }
//       }
//   }
//   Works identically. Just brittle to renames and noisier.
//
//   fn pair() -> (Counter, Counter) {
//       (Counter { value: 0 }, Counter { value: 0 })
//   }
//   Inlines the construction. Avoid for the same reason functions exist:
//   one source of truth for "what does it mean to make a fresh Counter?".
