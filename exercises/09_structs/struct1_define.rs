// =============================================================================
//  struct1 — defining a struct with named fields
// =============================================================================
//
// A `struct` is how you build a NAMED record from several fields. Each field
// has its own name and type:
//
//     struct Point {
//         x: i32,
//         y: i32,
//     }
//
// Note the trailing comma after the last field — idiomatic Rust style.
// Field names use snake_case; the type itself uses UpperCamelCase.
//
// CREATING ONE
//
//     let p = Point { x: 3, y: 4 };
//
// The braces are required, every field must be filled, and the order does
// NOT have to match the declaration order — the field names disambiguate.
//
// FIELD ACCESS
//
//     p.x       // 3
//     p.y       // 4
//
// FIELD-INIT SHORTHAND
//
// When a local variable has the same name as a field, you can write just the
// name once instead of `name: name`:
//
//     fn make_point(x: i32, y: i32) -> Point {
//         Point { x, y }      // shorthand for `Point { x: x, y: y }`
//     }
//
// `#[derive(Debug, Clone, PartialEq)]` above a struct asks the compiler to
// generate sensible default implementations of those things for free. We use
// it freely throughout the course — we'll explain the mechanics in a later
// chapter.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define a `Point` with fields `x: i32` and `y: i32`.
//   - Implement `make_point(x, y) -> Point` using the field-init shorthand.
//   - Implement `manhattan(p) -> i32` returning |x| + |y| of the point.
//     HINT: `i32` has an `.abs()` method.

// I AM NOT DONE

#[derive(Debug, Clone, PartialEq)]
struct Point {
    ???
    ???
}

fn make_point(x: i32, y: i32) -> Point {
    // Use the field-init shorthand here.
    Point { ??? }
}

fn manhattan(p: Point) -> i32 {
    p.???.abs() + p.???.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn builds_point() {
        let p = make_point(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }
    #[test] fn manhattan_origin() {
        assert_eq!(manhattan(make_point(0, 0)), 0);
    }
    #[test] fn manhattan_positive() {
        assert_eq!(manhattan(make_point(3, 4)), 7);
    }
    #[test] fn manhattan_negative() {
        assert_eq!(manhattan(make_point(-3, -4)), 7);
    }
    #[test] fn equality_works() {
        // PartialEq was derived, so two points with the same fields are equal.
        assert_eq!(make_point(1, 2), Point { x: 1, y: 2 });
    }
}

fn main() {}
