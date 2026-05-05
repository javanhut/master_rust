// SOLUTION — str6_parse

fn parse_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap()
}

fn parse_or_zero(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or(0)
}

fn sum_csv_ints(s: &str) -> i32 {
    let mut total = 0;
    for piece in s.split(',') {
        total += piece.trim().parse::<i32>().unwrap();
    }
    total
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   .parse::<T>() is the universal text-to-T entry point. Annotating the
//   target type via turbofish is preferred when you DON'T have a binding
//   to attach a `: T` annotation to (e.g. inside an expression).
//
//   .unwrap() — quick and loud. Panics on a parse error. Fine in tests and
//   in tiny scripts; not fine for production input. We are deliberately
//   keeping error handling out of this chapter because it gets a chapter
//   of its own (chapter 8: Result, ?, custom errors).
//
//   .unwrap_or(0) — the gentle escape hatch. Replaces an Err with a default.
//   Useful for "best-effort" parsing where 0 is a valid fallback. Cousins:
//       .unwrap_or_else(|_| compute_default())
//       .unwrap_or_default()           // 0 for ints, "" for String, etc.
//
//   sum_csv_ints — `.trim()` BEFORE `.parse()` is the single most common
//   bug-fix you'll perform on parse code. " 10".parse::<i32>() FAILS, while
//   " 10".trim().parse::<i32>() succeeds. Always trim untrusted text.
//
// IDIOMATIC FUNCTIONAL FORM (preview of iterators chapter):
//
//     fn sum_csv_ints(s: &str) -> i32 {
//         s.split(',')
//          .map(|p| p.trim().parse::<i32>().unwrap())
//          .sum()
//     }
//
// PROPER ERROR-HANDLING PREVIEW (chapter 8):
//
//     fn sum_csv_ints(s: &str) -> Result<i32, std::num::ParseIntError> {
//         s.split(',').map(|p| p.trim().parse::<i32>()).sum()
//     }
//
//   `Result` implements `Sum`, so a single bad piece short-circuits the
//   whole thing into Err. Beautiful — but we'll get there.
