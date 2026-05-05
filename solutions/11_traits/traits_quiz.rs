// SOLUTION — traits_quiz

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width:  f64,
    height: f64,
}

struct Triangle {
    base:   f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn total_area_static<S: Shape>(shapes: &[S]) -> f64 {
    let mut sum = 0.0;
    for s in shapes {
        sum += s.area();
    }
    sum
}

fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    let mut sum = 0.0;
    for s in shapes {
        sum += s.area();
    }
    sum
}

// WHY THIS IS OPTIMAL:
//
//   Three concrete shapes, one trait with one required method, two
//   dispatch styles wrapping the same loop. That's exactly the geometry
//   of polymorphism in Rust: traits define the contract, generics give
//   you static dispatch, `dyn` gives you dynamic dispatch, and you pick
//   per call site based on whether the collection is homogeneous.
//
// STATIC vs DYNAMIC IN THIS EXACT CODE:
//
//   total_area_static<S: Shape>(shapes: &[S])
//
//     The compiler will produce a separate compiled copy of this function
//     for every concrete S used at call sites. Tests call it with `[Rectangle; 3]`
//     and `[Triangle; 2]` — that's two specialised copies. The body is tiny,
//     LLVM almost certainly inlines `s.area()`, and the loop becomes a
//     direct sum with zero indirection.
//
//   total_area_dyn(shapes: &[Box<dyn Shape>])
//
//     ONE compiled copy. Each `s.area()` does a vtable lookup (one
//     indirect call). The compiler doesn't inline across the boundary,
//     but you can mix Rectangles, Triangles, and Circles in a single
//     vector — which is impossible in the static version.
//
// WHY `for s in shapes` AND NOT `for s in shapes.iter()`:
//
//   `for s in shapes` already calls `IntoIterator` on the slice reference,
//   which is `iter()`. So `s: &S` (or `s: &Box<dyn Shape>` in the dyn
//   version). Both are fine for `s.area()` — auto-deref + auto-borrow
//   handle the rest.
//
//   In the dyn version, `s: &Box<dyn Shape>`, and `Box<T>` derefs to T.
//   So `s.area()` is `(**s).area()` — Box → dyn Shape → vtable call. The
//   compiler does this for free.
//
// EQUIVALENT WITH ITERATORS:
//
//     fn total_area_static<S: Shape>(shapes: &[S]) -> f64 {
//         shapes.iter().map(|s| s.area()).sum()
//     }
//
//   Same code, half the lines, requires `f64: Sum<f64>` (it is). Iterator
//   pipeline lands in chapter 7 — using the explicit `for` is just as
//   fine and maybe clearer at this point.
//
// COULD WE USE `&[&dyn Shape]` INSTEAD OF `&[Box<dyn Shape>]`?
//
//   Yes — for borrowed-only collections. `Box<dyn Shape>` is the right
//   shape when you want OWNED, heap-stored heterogeneous values that
//   live as long as the collection. `&dyn Shape` is the right shape when
//   the values live elsewhere and you only want a temporary view.
//
// FINAL ANSWER TO "WHEN DO I USE WHICH?":
//
//   - Single concrete type at the call site, hot loop → static (`<S: Shape>`).
//   - Plugin-style mixing of types in a collection → dynamic (`Box<dyn Shape>`).
//   - In doubt, start static; reach for dyn the moment you need a
//     heterogeneous collection or a stable plugin ABI.
