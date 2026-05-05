// =============================================================================
//  opt2 — pattern-matching an Option
// =============================================================================
//
// `Option<T>` is just an enum, so EVERY pattern-matching tool you learned
// in chapter 10 works here:
//
//     match opt {
//         Some(x) => use_it(x),
//         None    => default(),
//     }
//
// The compiler enforces EXHAUSTIVENESS — you must handle both `Some` and
// `None`. That's the whole reason `Option` is safer than `null`: the type
// system refuses to let you forget the empty case.
//
// EXTRACTING THE INNER VALUE
// ──────────────────────────
//   The `Some(x)` pattern BINDS the inner value to `x` for the body of
//   that arm. `x` has type `T`, not `Option<T>`. No `.unwrap()` needed —
//   the match arm proved we're in the `Some` case.
//
//     let opt: Option<i32> = Some(5);
//     match opt {
//         Some(x) => println!("got {x}"),   // x: i32
//         None    => println!("nothing"),
//     }
//
// USING `match` AS AN EXPRESSION
// ──────────────────────────────
//     let label = match maybe_age {
//         Some(0)        => "newborn",
//         Some(n) if n<18 => "minor",
//         Some(_)        => "adult",
//         None           => "unknown",
//     };
//
// Every arm produces the same type (here `&str`). The match's value is
// bound to `label`. Guards (`if n < 18`) refine a pattern; they do NOT
// count toward exhaustiveness, so a guardless fallback arm is required.
//
// `if let` SHORT FORM
// ───────────────────
//     if let Some(x) = opt {
//         use_it(x);
//     } else {
//         default();
//     }
//
// `if let` is the convenience form when only ONE pattern is interesting.
// We use full `match` in this exercise to drill the exhaustive shape.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `describe(opt)`     - return "some N" for `Some(N)` and "none" for `None`.
//                           Use `format!`. The match must be exhaustive.
//   - `double_or(opt, d)` - return `n*2` for `Some(n)`, otherwise `d`.
//   - `bucket(opt)`       - classify an Option<i32>:
//                             None             -> "missing"
//                             Some(0)          -> "zero"
//                             Some(n) if n > 0 -> "positive"
//                             Some(_)          -> "negative"
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn describe(opt: Option<i32>) -> String {
    match opt {
        Some(n) => format!("some {}", ???),
        ??? => "none".to_string(),
    }
}

fn double_or(opt: Option<i32>, default: i32) -> i32 {
    match opt {
        Some(n) => ???,
        None    => ???,
    }
}

fn bucket(opt: Option<i32>) -> &'static str {
    match opt {
        None             => "missing",
        Some(0)          => ???,
        Some(n) if n > 0 => ???,
        Some(_)          => ???,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn describe_some() { assert_eq!(describe(Some(7)), "some 7"); }
    #[test] fn describe_none() { assert_eq!(describe(None),    "none");   }

    #[test] fn double_some() { assert_eq!(double_or(Some(5), 0), 10); }
    #[test] fn double_none() { assert_eq!(double_or(None,    9),  9); }

    #[test] fn bucket_missing()  { assert_eq!(bucket(None),       "missing");  }
    #[test] fn bucket_zero()     { assert_eq!(bucket(Some(0)),    "zero");     }
    #[test] fn bucket_positive() { assert_eq!(bucket(Some(3)),    "positive"); }
    #[test] fn bucket_negative() { assert_eq!(bucket(Some(-4)),   "negative"); }
}

fn main() {}
