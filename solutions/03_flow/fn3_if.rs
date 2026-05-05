// SOLUTION — fn3_if

fn classify(x: i32) -> &'static str {
    if x < 0 {
        "negative"
    } else if x == 0 {
        "zero"
    } else {
        "positive"
    }
}

// WHY THIS IS OPTIMAL:
//   - Each arm yields a `&'static str` literal. The whole `if` expression
//     is the function body.
//   - `==` for the zero check (NOT `=`, which would be assignment).
//
// USING match:
//
//   match x.cmp(&0) {
//       std::cmp::Ordering::Less    => "negative",
//       std::cmp::Ordering::Equal   => "zero",
//       std::cmp::Ordering::Greater => "positive",
//   }
//
//   `Ord::cmp` returns the three-valued `Ordering`. Slightly more typing
//   for this case, but absolutely the right tool when comparing two
//   non-zero values where each ordering deserves a unique branch.
