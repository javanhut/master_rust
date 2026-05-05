// =============================================================================
//  match4 — `if let`, `let else`, `while let`
// =============================================================================
//
// `match` is great when you handle every variant. When you only care about
// ONE pattern, three lighter forms exist.
//
// `if let`
// ────────
// "If the value matches this pattern, run this block":
//
//     if let Some(x) = opt {
//         println!("got {x}");
//     } else {
//         println!("nothing");
//     }
//
// Equivalent to a 2-arm `match` where the second arm is `_ => ...`.
// Cleaner when the "other" arm is a simple else (or absent).
//
// `let else`  (Rust 1.65+)
// ────────────────────────
// Same idea, but it BINDS into the surrounding scope and the else block
// MUST diverge (return / break / panic / continue):
//
//     fn first_word(s: &str) -> Option<&str> {
//         let Some(idx) = s.find(' ') else {
//             return Some(s);            // no space at all
//         };
//         Some(&s[..idx])                // `idx` is in scope here
//     }
//
// Beautiful for guard clauses: "extract this, otherwise abort".
//
// `while let`
// ───────────
// Loop AS LONG AS the pattern matches:
//
//     let mut stack = vec![1, 2, 3];
//     while let Some(top) = stack.pop() {
//         println!("{top}");
//     }
//     // exits when pop() returns None
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Three small functions:
//
//   1. `double_or_zero(opt: Option<i32>) -> i32`
//        Use `if let Some(x) = opt { ... } else { ... }`.
//        Some(x) → x*2, None → 0.
//
//   2. `parse_or_zero(s: &str) -> i32`
//        Use `let else`. Try `s.parse::<i32>()`; on Err return 0 from the
//        function. On Ok bind the value and return it.
//
//   3. `pop_all(v: &mut Vec<i32>) -> i32`
//        Use `while let Some(x) = v.pop()` to drain `v` into a sum, then
//        return the sum.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

fn double_or_zero(opt: Option<i32>) -> i32 {
    if let Some(x) = ??? {
        x * 2
    } else {
        ???
    }
}

fn parse_or_zero(s: &str) -> i32 {
    let ??? Ok(n) = s.parse::<i32>() ??? {
        return 0;
    };
    n
}

fn pop_all(v: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    while let Some(x) = v.???() {
        sum += x;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn doubler() {
        assert_eq!(double_or_zero(Some(7)), 14);
        assert_eq!(double_or_zero(None),     0);
    }
    #[test] fn parser() {
        assert_eq!(parse_or_zero("42"),  42);
        assert_eq!(parse_or_zero("xyz"), 0);
        assert_eq!(parse_or_zero(""),    0);
    }
    #[test] fn drainer() {
        let mut v = vec![1, 2, 3, 4];
        assert_eq!(pop_all(&mut v), 10);
        assert!(v.is_empty());
    }
}

fn main() {}
