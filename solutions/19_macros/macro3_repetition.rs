// SOLUTION — macro3_repetition

macro_rules! min_of {
    ( $head:expr $(, $tail:expr )* ) => {
        $head $( .min($tail) )*
    };
}

// WHY THIS IS OPTIMAL:
//
//   The HEAD/TAIL split is the cleanest way to require at least one
//   argument while still emitting a chain of method calls — a `+`
//   repeater (`$( $x:expr ),+`) would also accept one-or-more, but then
//   the expansion has to start the chain somewhere, and you'd end up
//   with awkward shapes like `[$($x),+].iter().copied().min().unwrap()`.
//   The head/tail trick gives you a clean reduction:
//
//       min_of!(7)                 -> 7
//       min_of!(7, 3)              -> 7.min(3)
//       min_of!(7, 3, 2)           -> 7.min(3).min(2)
//
//   ONE element produces ZERO `.min(...)` calls — the pattern `,*`
//   matches zero repetitions just fine. Each subsequent element adds
//   one more `.min(...)` to the chain.
//
// HOW THE REPETITION READS:
//
//     ( $head:expr $(, $tail:expr )* )
//
//   "Match one expression, then zero-or-more occurrences of a comma
//    followed by another expression." The outer `$( ... )*` says "this
//    sub-pattern may repeat zero or more times, no SEPARATOR is required
//    between full repetitions" — but inside that group there IS a comma,
//    so each repetition starts with one. This is the standard idiom for
//    "a comma-separated list with at least one element."
//
//     $head $( .min($tail) )*
//
//   The expansion mirrors the matching structure: emit `$head` once,
//   then emit `.min($tail)` for each captured `$tail`.
//
// ALTERNATIVES YOU MIGHT SEE:
//
//   1. Using `+` directly:
//
//          ( $( $x:expr ),+ ) => {{
//              let mut it = [$($x),+].into_iter();
//              let first = it.next().unwrap();
//              it.fold(first, |acc, x| acc.min(x))
//          }};
//
//      Works, but builds a temporary array and an iterator at runtime.
//      The head/tail version reduces to plain `.min` calls inlined by
//      the compiler — usually faster in release mode.
//
//   2. Tolerating a trailing comma:
//
//          ( $head:expr $(, $tail:expr )* $(,)? ) => { ... };
//
//      `$(,)?` is "an optional trailing comma." Library macros like
//      `vec!` and `println!` accept this for ergonomics. Add it if you
//      want `min_of!(1, 2, 3,)` to compile.
//
// COMMON MISTAKES:
//
//   - Forgetting to wrap the EXPANSION repetition in `$( ... )*`. The
//     pattern can match repetitions, but the expansion only emits them
//     when you re-use the same shape.
//   - Mismatched separators between matching and expansion. The pattern
//     and expansion separators do NOT have to match each other; only
//     the captured variables and the surrounding `$( )` do.
//   - Reaching for `tt` instead of `expr` "to be safe." It's the
//     opposite — `tt` accepts anything and surfaces compile errors at
//     the macro-call site, often with confusing diagnostics. Prefer
//     `expr` for things meant to be expressions.
