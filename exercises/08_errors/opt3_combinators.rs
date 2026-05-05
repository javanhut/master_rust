// =============================================================================
//  opt3 — Option combinators
// =============================================================================
//
// `match` always works, but for simple "transform / default / chain"
// patterns the standard library gives you methods that read better and
// chain cleanly. They're called COMBINATORS — methods on `Option<T>`
// that return another `Option` or unwrap to a `T`.
//
// TRANSFORMING — keep the Option shape
// ────────────────────────────────────
//   .map(f)        — Some(x) -> Some(f(x));  None -> None
//   .and_then(f)   — Some(x) -> f(x);        None -> None
//                    `f` must itself return Option<U>. Use this when the
//                    next step CAN ALSO FAIL — flattening avoids
//                    Option<Option<U>>.
//
//     Some(5).map(|n| n + 1)            // Some(6)
//     Some(5).and_then(|n| half(n))     // Some(_) or None
//     None  .map(|n: i32| n + 1)        // None
//
// CHOOSING A FALLBACK — also keep the Option shape
// ────────────────────────────────────────────────
//   .or(other)         — self if Some, else `other` (eagerly evaluated).
//   .or_else(f)        — self if Some, else `f()` (closure runs lazily).
//
//     a.or(b)               // a unless None, then b
//     a.or_else(|| compute())  // compute() only runs if a is None
//
// EXTRACTING — leave Option, get T
// ────────────────────────────────
//   .unwrap_or(d)         — value or `d` (eager).
//   .unwrap_or_else(f)    — value or `f()` (lazy).
//   .unwrap_or_default()  — value or `T::default()` (0, "", empty Vec, ...).
//
// EAGER vs LAZY — when does it matter?
// ────────────────────────────────────
// `unwrap_or(expensive_compute())` always runs `expensive_compute()`,
// even when the Option is `Some`. That's a waste — and a bug if the
// fallback has side effects (logging, file IO). Reach for the `_else`
// form whenever the fallback is non-trivial:
//
//     opt.unwrap_or_else(|| read_default_from_disk())
//
// CHAINING — the real reason combinators exist
// ────────────────────────────────────────────
// You can string them together to express "parse, validate, default"
// as a single expression:
//
//     "42".parse::<i32>().ok()      // Result -> Option (next exercise topic)
//        .and_then(|n| half(n))     // halve if even
//        .map(|n| n * 10)
//        .unwrap_or(-1)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `add_one(opt)`     -> Option<i32>: map; Some(n) becomes Some(n+1).
//   - `default_zero(opt)`-> i32:         use unwrap_or.
//   - `or_compute(opt)`  -> i32:         use unwrap_or_else with the closure
//                                         `|| 100 - 1` (proves laziness).
//   - `default_t(opt)`   -> String:      use unwrap_or_default
//                                         (String::default() is "").
//   - `pipeline(s)`      -> i32:         parse `s` as i32 via `.parse().ok()`,
//                                         double it with `.map`, fall back
//                                         to `-1` with `.unwrap_or`.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn add_one(opt: Option<i32>) -> Option<i32> {
    opt.???(|n| n + 1)
}

fn default_zero(opt: Option<i32>) -> i32 {
    opt.???(0)
}

fn or_compute(opt: Option<i32>) -> i32 {
    opt.???(|| 100 - 1)
}

fn default_t(opt: Option<String>) -> String {
    opt.???()
}

fn pipeline(s: &str) -> i32 {
    s.parse::<i32>().???()
        .???(|n| n * 2)
        .???(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn add_some() { assert_eq!(add_one(Some(4)), Some(5)); }
    #[test] fn add_none() { assert_eq!(add_one(None),    None);    }

    #[test] fn dz_some() { assert_eq!(default_zero(Some(7)), 7); }
    #[test] fn dz_none() { assert_eq!(default_zero(None),    0); }

    #[test] fn oc_some() { assert_eq!(or_compute(Some(1)),  1);  }
    #[test] fn oc_none() { assert_eq!(or_compute(None),     99); }

    #[test] fn dt_some() { assert_eq!(default_t(Some("hi".into())), "hi"); }
    #[test] fn dt_none() { assert_eq!(default_t(None),              "");   }

    #[test] fn pipe_ok()  { assert_eq!(pipeline("21"),   42); }
    #[test] fn pipe_err() { assert_eq!(pipeline("nope"), -1); }
}

fn main() {}
