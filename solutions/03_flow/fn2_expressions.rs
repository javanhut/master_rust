// SOLUTION — fn2_expressions

fn tier_name(points: i32) -> &'static str {
    if points >= 100 {
        "gold"
    } else if points >= 50 {
        "silver"
    } else {
        "bronze"
    }
}

// WHY THIS IS OPTIMAL:
//   - The `if/else if/else` chain IS the function body, no intermediate
//     `let tier = ...; tier`. The chain evaluates to one of three string
//     literals, each `&'static str`, all the same type — perfect.
//   - Order matters: check `>= 100` before `>= 50`, otherwise `120` would
//     silver-tier.
//
// ALTERNATIVES:
//
//   match points {
//       p if p >= 100 => "gold",
//       p if p >= 50  => "silver",
//       _             => "bronze",
//   }
//
//   `match` with guards. Slightly more lines, but scales beautifully when
//   you add ranges or multiple conditions. For 3 thresholds, the `if/else`
//   chain reads better.
//
//   match points {
//       100..   => "gold",
//       50..=99 => "silver",
//       _       => "bronze",
//   }
//
//   With range patterns. Lovely once you're comfortable with `match`.
