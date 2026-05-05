// SOLUTION — modules_quiz

mod math {
    pub mod geometry {
        const PI: f64 = 3.141592653589793;

        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn area(&self) -> f64 {
                PI * self.radius * self.radius
            }
        }

        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            pub fn area(&self) -> f64 {
                self.width * self.height
            }
        }
    }

    pub mod stats {
        pub fn sum(values: &[f64]) -> f64 {
            values.iter().sum()
        }

        pub fn mean(values: &[f64]) -> Option<f64> {
            if values.is_empty() {
                None
            } else {
                Some(sum(values) / values.len() as f64)
            }
        }
    }

    pub mod prelude {
        pub use super::geometry::{Circle, Rectangle};
        pub use super::stats::{mean, sum};
    }
}

// WHY THIS IS OPTIMAL:
//
//   ORGANISATION mirrors the problem domain. `geometry` owns shapes;
//   `stats` owns statistics; `prelude` owns the public surface. Each of
//   the three has a clear job and a single reason to change. That's the
//   payoff of modules: change the implementation of `mean` without ever
//   thinking about `Circle`.
//
//   PI is `const` and PRIVATE to `geometry`. It's an implementation
//   detail of `Circle::area` — there's no reason to expose it. If a user
//   wants π, the standard library already provides
//   `std::f64::consts::PI`. Hiding it removes one thing from the public
//   surface, which is one less thing to support forever.
//
//   `mean` returns `Option<f64>` because the mean of nothing is undefined.
//   Returning `0.0` or `f64::NAN` would let bugs hide; `None` makes the
//   missing answer impossible to ignore.
//
//   `prelude` is `pub use` over four names. Users write
//   `use math::prelude::*;` and have everything they need — but the
//   precise paths (`math::geometry::Circle`) still work for callers who
//   prefer to be explicit. Both styles coexist for free.
//
// EQUIVALENT BUT WORSE:
//
//   pub fn mean(values: &[f64]) -> f64 {
//       if values.is_empty() { 0.0 }
//       else { values.iter().sum::<f64>() / values.len() as f64 }
//   }
//     — silently lies for empty input. The Option signature is more
//       honest and forces callers to think about the edge case once.
//
//   No prelude — users always write the deep path. Workable, but every
//   user file ends up with a wall of `use crate::math::geometry::Circle;`
//   lines. `prelude` puts that boilerplate in ONE place under the
//   library's control.
//
//   `pub const PI: f64 = ...` — exposing the private constant. Now PI is
//   part of your public API; deleting it tomorrow is a breaking change.
//   Keep the contract tight; the standard library already provides this
//   value to anyone who wants it.
//
// FILE-BASED EQUIVALENT:
//
//   In a real Cargo crate this would be:
//
//       src/lib.rs           pub mod math;
//       src/math/mod.rs      pub mod geometry; pub mod stats; pub mod prelude;
//       src/math/geometry.rs (the Circle/Rectangle code)
//       src/math/stats.rs    (sum/mean)
//       src/math/prelude.rs  (the `pub use` lines)
//
//   The runner here can only handle a single `.rs` file, so we use
//   inline `mod` blocks. Same paths, same privacy, same prelude trick —
//   the only thing that changes when you move to real Cargo is the file
//   layout.
