// SOLUTION — enum1_define

enum Shape {
    Unit,
    Circle(f64),
    Rectangle { w: f64, h: f64 },
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Unit => 0.0,
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { w, h } => w * h,
    }
}

// WHY THIS IS OPTIMAL:
//
//   The three variant flavours each pull their weight:
//     - `Unit` is just a tag — no allocation, no payload, useful as a
//       sentinel or "no-shape" placeholder.
//     - `Circle(f64)` uses a tuple variant because there's only ONE piece
//       of data and "radius" is obvious from context. Naming the field
//       (`Circle { r: f64 }`) would be noise.
//     - `Rectangle { w, h }` uses named fields because two f64s without
//       names would be ambiguous — is it (width, height) or (height,
//       width)? Naming them makes call sites self-documenting.
//
//   The function takes `&Shape` rather than `Shape` so callers don't have
//   to give up ownership. `match` on a reference automatically borrows the
//   inner fields (`r`, `w`, `h` are `&f64` here, but `*` is implicit in
//   arithmetic via auto-deref).
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   Three separate structs + a trait (`trait HasArea { fn area(&self); }`)
//     - Heavier; worth it when behaviour is open-ended and third parties
//       need to plug in new shapes. For a closed set of variants, an enum
//       is leaner and gives you exhaustiveness checking for free.
//
//   `Shape::Rectangle(f64, f64)`
//     - Compiles, but you've thrown away the field names. Past you knew
//       which f64 was the width; future you (or your reviewer) does not.
//
// SUBTLETY:
//   Variant names live in the enum's namespace. To call them bare you'd
//   write `use Shape::*;` at the top of the function or module — handy
//   in long match arms, noisy at the top level.
