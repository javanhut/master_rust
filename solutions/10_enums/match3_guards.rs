// SOLUTION — match3_guards

fn categorise(opt: Option<i32>) -> &'static str {
    match opt {
        Some(x) if x > 0 => "positive",
        Some(x) if x < 0 => "negative",
        Some(_) => "zero",
        None    => "missing",
    }
}

fn clamp_pos(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) if x > 100 => 100,
        Some(x) if x < 0   => 0,
        Some(x)            => x,
        None               => 0,
    }
}

// WHY THIS IS OPTIMAL:
//
//   Both functions use guards for the cases that depend on the VALUE of
//   the inner integer, then a guardless `Some(x)` arm to mop up the
//   remaining "any other Some" case. This is the canonical guard
//   pattern: 2-3 specialised arms, then a general fallback.
//
//   The order is important. `Some(_) if x < 0` would never fire if
//   `Some(_)` had appeared first — the unguarded `Some(_)` would steal
//   every `Some` value. With guards, the compiler still picks the FIRST
//   matching arm; an unguarded pattern below specialised guards is the
//   right shape.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `match opt { Some(x) => if x > 100 { 100 } else if x < 0 { 0 } else { x }, None => 0 }`
//     - Equivalent and slightly shorter, but mixes pattern matching with
//       imperative if/else inside the arm. Guards keep the decision
//       table flat and uniform.
//
//   `opt.map(|x| x.clamp(0, 100)).unwrap_or(0)`
//     - For `clamp_pos` THIS IS BETTER — `Option::map` and the built-in
//       `i32::clamp` express the intent in one breath. We wrote it with
//       guards here to drill the syntax. Real code: use the combinator.
//
// SUBTLETY:
//   Guards do NOT participate in exhaustiveness analysis. If you only
//   write `Some(x) if x > 0` and `None`, the compiler will reject the
//   match because `Some(x)` where `x <= 0` is not covered. The
//   guardless `Some(_)` (or `Some(x)`) arm is mandatory.
