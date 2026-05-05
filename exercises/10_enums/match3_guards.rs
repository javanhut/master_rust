// =============================================================================
//  match3 — `if` guards on match arms
// =============================================================================
//
// A pattern can ONLY check shape: "is this Some?", "is this 0?", "is this
// a `Coin::Quarter`?" When you also need a runtime test — "is this
// Some(x) where x > 10?" — you bolt an `if` GUARD onto the arm:
//
//     match opt {
//         Some(x) if x > 0 => "positive",
//         Some(x) if x < 0 => "negative",
//         Some(_)          => "zero",
//         None             => "absent",
//     }
//
// Read the arm as PATTERN + GUARD. Both must be true for the arm to fire.
//
// WHY GUARDS, NOT JUST `if` IN THE BODY?
//
//   Compare:
//
//       match v {
//           Some(x) if x > 0 => big(x),
//           Some(x)          => small(x),
//           None             => zero(),
//       }
//
//       match v {
//           Some(x) => if x > 0 { big(x) } else { small(x) },
//           None    => zero(),
//       }
//
//   The first form FALLS THROUGH to later arms when the guard fails —
//   that's the magic. The second handles the if/else inline. Use guards
//   when you want unmatched cases to try the next arm.
//
// EXHAUSTIVENESS WARNING
//   The compiler does NOT consider guards when checking exhaustiveness.
//   `Some(x) if x > 0` does NOT count as covering `Some(x)`. You'll
//   typically need a fallback arm without a guard to mop up.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `categorise(opt: Option<i32>) -> &'static str`:
//   - Some(x) where x > 0   → "positive"
//   - Some(x) where x < 0   → "negative"
//   - Some(_)               → "zero"
//   - None                  → "missing"
//
// Then implement `clamp_pos(opt: Option<i32>) -> i32`:
//   - Some(x) where x > 100 → 100
//   - Some(x) where x < 0   → 0
//   - Some(x)               → x         (unchanged)
//   - None                  → 0
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn categorise(opt: Option<i32>) -> &'static str {
    match opt {
        Some(x) if x > 0 => ???,
        Some(x) if x ??? => "negative",
        Some(_) => "zero",
        None    => ???,
    }
}

fn clamp_pos(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) if x > 100 => ???,
        Some(x) if ???     => 0,
        Some(x)            => x,
        None               => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn cat() {
        assert_eq!(categorise(Some( 5)), "positive");
        assert_eq!(categorise(Some(-5)), "negative");
        assert_eq!(categorise(Some( 0)), "zero");
        assert_eq!(categorise(None),     "missing");
    }
    #[test] fn clamp() {
        assert_eq!(clamp_pos(Some(150)), 100);
        assert_eq!(clamp_pos(Some( 50)),  50);
        assert_eq!(clamp_pos(Some( -3)),   0);
        assert_eq!(clamp_pos(None),        0);
    }
}

fn main() {}
