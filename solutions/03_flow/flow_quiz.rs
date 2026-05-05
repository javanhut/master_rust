// SOLUTION — flow_quiz (FizzBuzz)

fn fizzbuzz(n: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 1..=n {
        let s = if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };
        out.push(s);
    }
    out
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//   - Demonstrates `if/else if/else` as an expression, with each arm
//     yielding `String`. The whole chain is bound to `s` and pushed once.
//   - `15` first because `15` ⇒ both `3` and `5` would also match.
//
// COMMON BUG: writing
//
//     if i % 3 == 0 { "Fizz" }
//     else if i % 5 == 0 { "Buzz" }
//     else if i % 15 == 0 { "FizzBuzz" }     // unreachable!
//
// The 15 branch can never fire because every multiple of 15 is already a
// multiple of 3.
//
// IDIOMATIC ITERATOR VERSION (preview):
//
//   fn fizzbuzz(n: u32) -> Vec<String> {
//       (1..=n).map(|i| match (i % 3, i % 5) {
//           (0, 0) => "FizzBuzz".to_string(),
//           (0, _) => "Fizz".to_string(),
//           (_, 0) => "Buzz".to_string(),
//           _      => i.to_string(),
//       }).collect()
//   }
//
//   Tuple-pattern `match` — beautifully captures the "two boolean tests"
//   structure. This is why `match` is so loved in Rust.
//
// PERFORMANCE NOTE:
//   `String::with_capacity` and writing to the same buffer is faster, but
//   FizzBuzz is the canonical "clarity over speed" exercise. Don't
//   optimise here.
