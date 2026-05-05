// SOLUTION — struct_quiz

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rectangle {
    width:  u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(side: u32) -> Self {
        Self::new(side, side)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn contains(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// WHY THIS IS OPTIMAL:
//
//   `Rectangle` has two `u32`s — eight bytes inline, no heap, no Drop, no
//   surprises. Every method takes `&self` because none of them need to
//   mutate or consume the receiver; `contains` likewise borrows `other`
//   so the caller keeps both rectangles around for further use.
//
//   `Self::square` delegating to `Self::new` is the textbook "constructor
//   reuse" pattern: one source of truth for "what does it mean to build
//   a Rectangle?". If construction ever grows (logging, validation), every
//   constructor benefits automatically.
//
//   `>=` (not `>`) in `contains` is the point of the equal-fit test — a
//   5x5 rectangle does fit inside another 5x5 rectangle. Be explicit about
//   the boundary case in your head whenever you write a comparison.
//
// EQUIVALENT BUT NOISIER:
//
//   fn square(side: u32) -> Self {
//       Self { width: side, height: side }
//   }
//   Works, but duplicates the constructor logic. Prefer `Self::new(...)`.
//
//   fn perimeter(&self) -> u32 {
//       self.width + self.width + self.height + self.height
//   }
//   Identical at the assembly level after constant folding, but `2 * (w + h)`
//   reads like the math formula and is the universal idiom.
//
//   fn contains(self, other: Rectangle) -> bool { ... }
//   Also works — `Rectangle` is small and could derive `Copy`. Borrowing
//   (`&self`, `&other`) is the conservative default: it composes with
//   non-Copy fields you might add later without changing the API.
