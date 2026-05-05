// =============================================================================
//  modules_quiz — capstone: a tiny `math` library, organised properly
// =============================================================================
//
// Time to put it all together. You'll build a small math library spread
// across three nested modules:
//
//     mod math
//         pub mod geometry      // shapes & area
//         pub mod stats         // statistics on slices
//         pub mod prelude       // a curated re-export bundle
//
// Layout:
//
//   geometry::Circle { radius: f64 }
//   geometry::Rectangle { width: f64, height: f64 }
//   geometry::area(&Shape) — but we'll keep it simpler: each shape gets an
//                            `area(&self) -> f64` method.
//   geometry::PI: f64 — internal constant, NOT exposed (private).
//
//   stats::mean(values: &[f64]) -> Option<f64>     — None on empty input
//   stats::sum(values: &[f64]) -> f64
//
//   prelude — re-exports `Circle`, `Rectangle`, `mean`, `sum` so a user can
//   write `use crate::math::prelude::*;` and have everything they care
//   about in scope at once.
//
// THINGS TO PRACTISE:
//
//   - `pub` only on items that are part of the contract.
//   - Private items (`PI`) used by public methods, hidden from outside.
//   - Nested modules, reached via `math::geometry::Circle`.
//   - `pub use` to build a `prelude` facade.
//
// A NOTE ON TESTS
//
// The test module sits at the file's top level. To reach our items it
// uses `use super::*;` (which brings `math` into scope) and then drills
// down with paths like `math::geometry::Circle` or imports the prelude.
// In a real multi-file crate, integration tests under `tests/` would
// reach in via `use my_crate::math::prelude::*;` — same pattern, just a
// different starting path.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build the structure described above. Specifics:
//
//   - geometry::PI: f64 = 3.141592653589793  (PRIVATE constant)
//   - Circle::area(&self)    = PI * radius * radius
//   - Rectangle::area(&self) = width * height
//   - stats::sum(&[f64])     = sum of values  (use `iter().sum()`)
//   - stats::mean(&[f64])    = Some(sum/len) when non-empty, else None
//   - prelude re-exports: Circle, Rectangle, mean, sum.

// I AM NOT DONE

mod math {
    pub mod geometry {
        // Private to `geometry` — not exposed even via prelude.
        const PI: f64 = 3.141592653589793;

        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn area(&self) -> f64 {
                ??? * self.radius * self.radius
            }
        }

        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            pub fn area(&self) -> f64 {
                self.??? * self.???
            }
        }
    }

    pub mod stats {
        pub fn sum(values: &[f64]) -> f64 {
            values.iter().???()
        }

        pub fn mean(values: &[f64]) -> Option<f64> {
            if values.is_empty() {
                ???
            } else {
                Some(sum(values) / values.len() as f64)
            }
        }
    }

    pub mod prelude {
        // Re-export the items a typical user will reach for.
        pub use super::geometry::{Circle, Rectangle};
        pub use super::stats::{???, ???};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Pull the curated public surface in via the prelude — exactly how a
    // real user of this library would.
    use math::prelude::*;

    #[test] fn circle_area() {
        let c = Circle { radius: 2.0 };
        // 4π — allow a tiny float wobble.
        assert!((c.area() - 12.566370614359172).abs() < 1e-9);
    }

    #[test] fn rect_area() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test] fn sum_basic() {
        assert_eq!(sum(&[1.0, 2.0, 3.0]), 6.0);
    }

    #[test] fn mean_basic() {
        assert_eq!(mean(&[2.0, 4.0, 6.0]), Some(4.0));
    }

    #[test] fn mean_empty_is_none() {
        assert_eq!(mean(&[]), None);
    }

    // Original deep paths still work — `pub use` adds names, doesn't remove.
    #[test] fn deep_paths_work_too() {
        let c = math::geometry::Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(math::stats::sum(&[10.0, 20.0]), 30.0);
    }
}

fn main() {}
