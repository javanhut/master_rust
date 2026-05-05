// =============================================================================
//  enum1 — defining enums (the three variant flavours)
// =============================================================================
//
// An `enum` is a type whose value is exactly ONE of a fixed list of named
// variants. Where a `struct` says "all of these fields together", an `enum`
// says "exactly one of these alternatives". This is sometimes called a
// "sum type" or "tagged union".
//
// You've already met two famous enums:
//
//     enum Option<T> { None, Some(T) }
//     enum Result<T, E> { Ok(T), Err(E) }
//
// (We covered those in chapter 8 — here we're learning to build our own.)
//
// THREE VARIANT FLAVOURS
// ──────────────────────
//
// 1. UNIT (data-less) — just a name, no payload:
//
//        enum TrafficLight { Red, Yellow, Green }
//
// 2. TUPLE — variants that wrap one or more positional values:
//
//        enum IpAddr { V4(u8, u8, u8, u8), V6(String) }
//
// 3. NAMED-FIELD (struct-like) — variants with named fields, just like a
//    struct:
//
//        enum Event {
//            Click { x: i32, y: i32 },
//            Scroll { dy: i32 },
//        }
//
// You can mix all three flavours in the SAME enum. That's the point.
//
// CONSTRUCTING VARIANTS
//
//     let stop  = TrafficLight::Red;
//     let home  = IpAddr::V4(127, 0, 0, 1);
//     let click = Event::Click { x: 10, y: 20 };
//
// Each variant's name is namespaced under the enum: `Shape::Circle`, not
// just `Circle`. (You can `use Shape::*;` to import them all if you want.)
//
// SIZE NOTE
//   An enum's size is roughly `size_of_largest_variant + tag`. So `Shape`
//   below is the size of `Rectangle { w, h }` (two f64s = 16 bytes) plus
//   a small discriminant — even when the value is a `Circle`. Don't lose
//   sleep over this; just know it's not free.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Define an enum `Shape` with three variants:
//   - `Unit`            — data-less (a sentinel "empty shape")
//   - `Circle(f64)`     — tuple variant holding a radius
//   - `Rectangle { w: f64, h: f64 }` — named-field variant
//
// Then implement the free function `area(s: &Shape) -> f64` that returns:
//   - 0.0 for `Unit`
//   - π * r * r for `Circle` (use `std::f64::consts::PI`)
//   - w * h for `Rectangle`
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

enum Shape {
    ???,
    ???(f64),
    ??? { w: f64, h: f64 },
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Unit => ???,
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { w, h } => ???,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn unit_is_zero() {
        assert_eq!(area(&Shape::Unit), 0.0);
    }
    #[test] fn circle_area() {
        let a = area(&Shape::Circle(2.0));
        assert!((a - std::f64::consts::PI * 4.0).abs() < 1e-9);
    }
    #[test] fn rectangle_area() {
        assert_eq!(area(&Shape::Rectangle { w: 3.0, h: 4.0 }), 12.0);
    }
}

fn main() {}
