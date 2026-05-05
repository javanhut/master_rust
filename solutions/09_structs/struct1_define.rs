// SOLUTION — struct1_define

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn make_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn manhattan(p: Point) -> i32 {
    p.x.abs() + p.y.abs()
}

// WHY THIS IS OPTIMAL:
//
//   Two `i32` fields cost 8 bytes total and are stored inline — no heap, no
//   indirection, no fuss. Naming the fields (`x`, `y`) is much friendlier
//   than a tuple `(i32, i32)` once any non-trivial code reads from them.
//
//   The shorthand `Point { x, y }` is the universally preferred form when
//   parameter and field share a name. Writing `Point { x: x, y: y }` is
//   accepted but redundant.
//
//   Taking `p` by value here is fine because `Point` is small and `Copy`-
//   friendly. We did NOT derive `Copy` because we want to teach moves
//   honestly later — the test that calls `make_point(...)` and then never
//   reuses the value works either way.
//
// EQUIVALENT BUT NOISIER:
//
//   fn make_point(x: i32, y: i32) -> Point {
//       Point { x: x, y: y }       // explicit, fine, but redundant
//   }
//
//   fn manhattan(p: Point) -> i32 {
//       let Point { x, y } = p;    // destructuring also works
//       x.abs() + y.abs()
//   }
//   Use destructuring when you'd otherwise repeat `p.foo` four or five times,
//   or when the pattern carries meaning.
