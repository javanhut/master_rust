// =============================================================================
//  traits_quiz — Shape (capstone for chapter 11)
// =============================================================================
//
// Time to combine everything from this chapter:
//
//   - define a trait with a single required method,
//   - implement it for several concrete types,
//   - write a generic, statically-dispatched function over a slice of
//     ONE shape type,
//   - write a dynamically-dispatched function over a slice of mixed
//     boxed shape types.
//
// THE TRAIT
//
//     trait Shape {
//         fn area(&self) -> f64;
//     }
//
// THE TYPES
//
//     struct Circle    { radius: f64 }
//     struct Rectangle { width: f64, height: f64 }
//     struct Triangle  { base: f64, height: f64 }   // ½ × base × height
//
//     pi = std::f64::consts::PI
//
// THE TWO TOTAL-AREA FUNCTIONS
//
//     // Static: every element is the SAME concrete shape type S.
//     // Compiler stamps out one specialised copy per S you call it with.
//     fn total_area_static<S: Shape>(shapes: &[S]) -> f64
//
//     // Dynamic: heterogeneous collection, each element a Box<dyn Shape>.
//     // One compiled copy, vtable lookup per call.
//     fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64
//
// Both bodies are essentially the same shape: walk the slice, sum the
// areas. The signatures are different, and that's the whole point of the
// exercise — see both dispatch styles back to back.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define the three structs (named-field, all `f64`).
//   - Implement `Shape` for each one with the right area formula.
//   - Implement `total_area_static` and `total_area_dyn`.
//
// Use a simple `for` loop to sum — iterators arrived in chapter 7 but the
// explicit loop is fine here too. Either way works for the tests.

// I AM NOT DONE

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    ???
}

struct Rectangle {
    ???
    ???
}

struct Triangle {
    ???
    ???
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.??? * self.???
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.??? * self.???
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // ½ × base × height
        0.5 * self.??? * self.???
    }
}

// Static dispatch: all elements are the SAME type S.
// Generic parameter + bound. One specialised copy per S used at call sites.
fn total_area_static<S: ???>(shapes: &[S]) -> f64 {
    let mut sum = 0.0;
    for s in shapes {
        sum += s.???();
    }
    sum
}

// Dynamic dispatch: a slice of boxed trait objects with possibly-mixed
// concrete types. One compiled copy of the function regardless of mix.
fn total_area_dyn(shapes: &[Box<dyn ???>]) -> f64 {
    let mut sum = 0.0;
    for s in shapes {
        sum += s.???();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test] fn circle_area() {
        let c = Circle { radius: 2.0 };
        // π × 4
        assert!(approx(c.area(), std::f64::consts::PI * 4.0));
    }

    #[test] fn rectangle_area() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert!(approx(r.area(), 12.0));
    }

    #[test] fn triangle_area() {
        let t = Triangle { base: 5.0, height: 4.0 };
        // ½ × 5 × 4 = 10
        assert!(approx(t.area(), 10.0));
    }

    #[test] fn static_sum_rectangles() {
        let rects = [
            Rectangle { width: 1.0, height: 2.0 },
            Rectangle { width: 3.0, height: 4.0 },
            Rectangle { width: 5.0, height: 6.0 },
        ];
        // 2 + 12 + 30 = 44
        assert!(approx(total_area_static(&rects), 44.0));
    }

    #[test] fn static_sum_triangles() {
        let tris = [
            Triangle { base: 2.0, height: 2.0 },   // 2
            Triangle { base: 4.0, height: 3.0 },   // 6
        ];
        assert!(approx(total_area_static(&tris), 8.0));
    }

    #[test] fn dyn_sum_mixed() {
        let zoo: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle { width: 3.0, height: 4.0 }),  // 12
            Box::new(Triangle  { base:  5.0, height: 4.0 }),  // 10
            Box::new(Circle    { radius: 1.0 }),              // π
        ];
        let expected = 12.0 + 10.0 + std::f64::consts::PI;
        assert!(approx(total_area_dyn(&zoo), expected));
    }

    #[test] fn dyn_empty_is_zero() {
        let zoo: Vec<Box<dyn Shape>> = Vec::new();
        assert!(approx(total_area_dyn(&zoo), 0.0));
    }
}

fn main() {}
