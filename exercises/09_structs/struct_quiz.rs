// =============================================================================
//  struct_quiz — Rectangle (capstone for chapter 9)
// =============================================================================
//
// Time to combine everything from this chapter:
//
//   - named-field struct
//   - `impl` block with `&self` methods
//   - associated functions / `Self::new`
//   - methods that take another `&Rectangle` argument
//
// You will build a small `Rectangle` type with the following surface:
//
//     impl Rectangle {
//         fn new(width: u32, height: u32) -> Self;
//         fn square(side: u32) -> Self;          // associated, builds w == h
//         fn area(&self) -> u32;
//         fn perimeter(&self) -> u32;
//         fn contains(&self, other: &Rectangle) -> bool;
//     }
//
// `contains` is true iff `self` is large enough (in BOTH dimensions) to
// hold `other` — i.e. `self.width >= other.width && self.height >= other.height`.
//
// All methods are read-only — `&self` everywhere. There is no allocation
// anywhere in this exercise; integer arithmetic is enough.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Fill in the struct definition and every method body. Use `Self` inside
// the impl block. Use field-init shorthand where it applies.

// I AM NOT DONE

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rectangle {
    ???
    ???
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { ??? }
    }

    fn square(side: u32) -> Self {
        // A square is a rectangle whose width and height are both `side`.
        // Reuse `Self::new` rather than duplicating the constructor.
        Self::???(side, side)
    }

    fn area(&self) -> u32 {
        self.??? * self.???
    }

    fn perimeter(&self) -> u32 {
        2 * (self.??? + self.???)
    }

    fn contains(&self, other: &Rectangle) -> bool {
        self.??? >= other.??? && self.??? >= other.???
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn area_basic() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.area(), 12);
    }
    #[test] fn area_square() {
        let s = Rectangle::square(5);
        assert_eq!(s.area(), 25);
    }
    #[test] fn perimeter_basic() {
        assert_eq!(Rectangle::new(3, 4).perimeter(), 14);
    }
    #[test] fn perimeter_square() {
        assert_eq!(Rectangle::square(7).perimeter(), 28);
    }
    #[test] fn contains_strict() {
        let big   = Rectangle::new(10, 10);
        let small = Rectangle::new(3, 4);
        assert!(big.contains(&small));
        assert!(!small.contains(&big));
    }
    #[test] fn contains_equal_fits() {
        let a = Rectangle::new(5, 5);
        let b = Rectangle::new(5, 5);
        // Equal-sized rectangle fits exactly — `>=` not `>`.
        assert!(a.contains(&b));
    }
    #[test] fn contains_one_dim_too_big() {
        let a = Rectangle::new(10, 1);
        let b = Rectangle::new(2,  2);
        // Wide enough but not tall enough.
        assert!(!a.contains(&b));
    }
    #[test] fn square_is_a_rectangle() {
        // `Self::square(4)` should be exactly `Rectangle::new(4, 4)`.
        assert_eq!(Rectangle::square(4), Rectangle::new(4, 4));
    }
}

fn main() {}
