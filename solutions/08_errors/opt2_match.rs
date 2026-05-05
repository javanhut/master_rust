// SOLUTION — opt2_match

fn describe(opt: Option<i32>) -> String {
    match opt {
        Some(n) => format!("some {}", n),
        None    => "none".to_string(),
    }
}

fn double_or(opt: Option<i32>, default: i32) -> i32 {
    match opt {
        Some(n) => n * 2,
        None    => default,
    }
}

fn bucket(opt: Option<i32>) -> &'static str {
    match opt {
        None             => "missing",
        Some(0)          => "zero",
        Some(n) if n > 0 => "positive",
        Some(_)          => "negative",
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `describe` shows the canonical shape: `Some(n)` binds the payload,
//   `None` is the empty arm. The compiler verifies BOTH variants are
//   handled — that's the safety property `Option` exists to give you.
//
//   `double_or` is "extract or default". Reading top-to-bottom you
//   confront the missing case first; the happy path is the trailing
//   arm. Equally valid to swap the order — exhaustiveness doesn't
//   care about order on non-overlapping patterns.
//
//   `bucket` exercises:
//     - matching a SPECIFIC value inside a variant: `Some(0)` only
//       fires when the inner integer is zero. The compiler uses
//       structural equality.
//     - GUARDS: `Some(n) if n > 0` matches any `Some` whose payload
//       passes the runtime test. Guards do not count toward
//       exhaustiveness — that's why we need the final `Some(_)` arm.
//     - WILDCARD inside a variant: `Some(_)` catches "any remaining
//       Some", which by elimination is the negative case.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `double_or` via combinators (next exercise!):
//       opt.map(|n| n * 2).unwrap_or(default)
//     One line, no match. We're using `match` here to drill the shape.
//
//   `bucket` via if/else cascade:
//       if opt.is_none() { "missing" }
//       else if opt == Some(0) { "zero" }
//       else if opt.unwrap() > 0 { "positive" }
//       else { "negative" }
//     Loses exhaustiveness, repeats the unwrap, harder to read.
//
//   `describe` returning `&'static str` for the None case directly:
//     The `Some` arm produces a `String` (from `format!`), so both arms
//     must produce `String`. That's why we wrote `"none".to_string()`
//     — to align the types. `match` arms must agree on type.
//
// SUBTLETY:
//   The arm order in `bucket` matters because patterns OVERLAP:
//   `Some(0)` would also match `Some(n) if n > 0` if you flipped the
//   order to put `n > 0` first (well, not zero — but `Some(_)` would
//   eat everything). Rust matches top-to-bottom; the FIRST matching
//   pattern wins. Put specific patterns above general ones.
