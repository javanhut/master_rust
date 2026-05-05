// SOLUTION — intro3_variables
//
// The trick: you don't need a local at all. The trailing expression of the
// function body IS the return value.

fn area_of_square(side: i32) -> i32 {
    side * side
}

// WHY THIS IS OPTIMAL:
//   - Zero bindings, zero mutation, zero ceremony.
//   - The function body is a single expression — the most testable shape.
//
// PEDAGOGICAL ALTERNATIVES (legal but less idiomatic):
//
//   fn area_of_square(side: i32) -> i32 {
//       let area = side * side;
//       area
//   }
//   Fine. Names a sub-step. Use this when the right-hand side is complex
//   enough that the name aids readability — `side * side` is not.
//
//   fn area_of_square(side: i32) -> i32 {
//       return side * side;
//   }
//   Works, but `return` at the end of a function is non-idiomatic Rust.
//   Save `return` for early exits.
//
// PITFALL TO REMEMBER:
//
//   fn area_of_square(side: i32) -> i32 {
//       side * side;          // ❌ trailing `;` — function returns ()
//   }
