// =============================================================================
//  macro3 — repetition: `$( ... ),*` and `$( ... ),+`
// =============================================================================
//
// `square!(x)` only handles ONE argument. Most "useful" macros — `vec!`,
// `println!`, `assert_eq!`, `hashmap!` — take a variable number of items.
// That's where REPETITION comes in.
//
// SHAPE
// ─────
//
//     $( PATTERN )SEP REPEATER
//
//   - `PATTERN`  — any normal pattern, can capture `$x:expr` etc.
//   - `SEP`      — an optional separator token (`,`, `;`, `=>`). Omit if
//                  there is none between repeated chunks.
//   - `REPEATER`:
//         `*`  — zero or more
//         `+`  — one or more
//         `?`  — zero or one (no separator allowed)
//
// USE THE SAME SHAPE IN THE EXPANSION
//
// To EMIT one expansion piece per matched repetition, wrap the piece in
// `$( ... )SEP REPEATER` again. The `$(` `)*` markers MUST surround any
// captured variable from the matching side.
//
//     macro_rules! print_all {
//         ( $( $x:expr ),* ) => {
//             $(
//                 println!("{}", $x);
//             )*
//         };
//     }
//
//     print_all!(1, 2, 3);
//     // expands to:
//     //   println!("{}", 1);
//     //   println!("{}", 2);
//     //   println!("{}", 3);
//
// `vec!` — A FAMILIAR EXAMPLE
//
// The standard `vec![1, 2, 3]` is approximately:
//
//     macro_rules! vec_like {
//         ( $( $x:expr ),* ) => {{
//             let mut v = Vec::new();
//             $( v.push($x); )*
//             v
//         }};
//     }
//
// Note the double braces `{{ ... }}`: the OUTER pair is `macro_rules!`
// expansion delimiters; the INNER pair is the Rust block expression that
// the expansion produces (so the macro yields a value).
//
// TRAILING COMMAS
//
// `$( $x:expr ),*` accepts `1, 2, 3` but NOT `1, 2, 3,`. To accept the
// trailing comma, use `$( $x:expr ),* $(,)?`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `min_of!` — takes one or more expressions and reduces them
// pairwise with `.min(...)`:
//
//     min_of!(3, 1, 4, 1, 5, 9, 2, 6)   // == 1
//     min_of!(42)                        // == 42
//
// Strategy: capture a HEAD expression, then capture zero-or-more TAIL
// expressions, and emit `head $( .min(tail) )*`:
//
//     macro_rules! min_of {
//         ( $head:expr $(, $tail:expr )* ) => {
//             $head $( .min($tail) )*
//         };
//     }
//
// That makes ONE element a no-op (just `$head`) and N elements collapse
// to `$head.min(t1).min(t2)...min(tn)`. The `+` repeater would also
// work over one-or-more, but the head/tail trick lets us require AT
// LEAST one element while still calling `.min` zero times for that case.
//
// Replace each `???`. Don't change the tests.

// I AM NOT DONE

macro_rules! min_of {
    ( $head:??? $(, $tail:??? )* ) => {
        $head $( .???($tail) )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_element() {
        assert_eq!(min_of!(42), 42);
    }

    #[test]
    fn two_elements() {
        assert_eq!(min_of!(7, 3), 3);
    }

    #[test]
    fn many_elements() {
        assert_eq!(min_of!(3, 1, 4, 1, 5, 9, 2, 6), 1);
    }

    #[test]
    fn floats_too() {
        // .min on f64 lives on the type. The macro is type-agnostic.
        let result: f64 = min_of!(3.5_f64, 1.25, 4.0, 2.75);
        assert!((result - 1.25).abs() < 1e-9);
    }

    #[test]
    fn expression_arguments() {
        // Each input is an arbitrary `expr`, not just a literal.
        let a = 10;
        let b = 4;
        assert_eq!(min_of!(a + 1, b * 2, a - b), 6);
    }
}

fn main() {}
