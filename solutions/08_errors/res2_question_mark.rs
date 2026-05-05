// SOLUTION — res2_question_mark

fn parse_i32(s: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(s.parse::<i32>()?)
}

fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

fn triple(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse::<i32>()?;
    Ok(n * 3)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `parse_i32` is the smallest possible `?` example. Reading it:
//     - `s.parse::<i32>()` produces `Result<i32, ParseIntError>`.
//     - `?` either extracts `i32` or early-returns the error.
//     - We then wrap the `i32` back in `Ok(...)` because our function's
//       return type is itself a `Result`.
//   This pattern — "do something fallible with `?`, wrap the success
//   in `Ok` at the end" — is the fundamental building block.
//
//   `add_two` shows short-circuiting: if the first parse fails, the
//   second never runs. The error type matches the function's `E`, so
//   no conversion is needed (we'll meet conversions in res4).
//
//   `triple` separates the parse from the arithmetic with a `let`. Use
//   this shape when the post-parse computation is non-trivial or when
//   you want to bind the value with a clear type annotation. The
//   compiler's desugaring is exactly:
//
//       let n: i32 = match s.parse::<i32>() {
//           Ok(v)  => v,
//           Err(e) => return Err(From::from(e)),
//       };
//       Ok(n * 3)
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   parse_i32 written as:
//       s.parse::<i32>()
//     This is FUNCTIONALLY identical — the inner Result already has the
//     right type. We wrote the `?`-then-`Ok` form to drill the operator,
//     not because it's necessary here. In real code you'd just return
//     the inner `Result` directly.
//
//   add_two via match:
//       match a.parse() {
//           Ok(av) => match b.parse() {
//               Ok(bv) => Ok(av + bv),
//               Err(e) => Err(e),
//           },
//           Err(e) => Err(e),
//       }
//     The "pyramid of doom". `?` collapses this down to one line.
//
//   `?` inside a function returning plain `i32`:
//       fn bad(s: &str) -> i32 { s.parse::<i32>()? }
//     ❌ COMPILE ERROR. `?` REQUIRES the surrounding function to return
//     `Result` (or `Option`, or any type that implements `Try`). The
//     compiler tells you exactly that.
//
// SUBTLETY:
//   The `From::from` conversion in the desugaring means `?` will
//   silently UPGRADE one error type into another whenever an
//   `impl From<ChildErr> for ParentErr` is in scope. That's the topic
//   of res4. For now both sides match exactly (`ParseIntError`), so
//   `From::from` is the no-op identity — but it's there.
//
//   `?` also works on `Option`: `opt?` returns the function early with
//   `None` if the option was empty. We focus on `Result` in this
//   chapter because that's where it matters most.
