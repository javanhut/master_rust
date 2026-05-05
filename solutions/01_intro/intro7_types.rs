// SOLUTION — intro7_types

fn parse_things() -> (u32, i64) {
    let count: u32 = "150".parse().unwrap();
    let parsed = "-42".parse::<i64>().unwrap();
    (count, parsed)
}

// WHY THIS IS OPTIMAL:
//   - Two ways to annotate, deliberately demonstrated side-by-side.
//   - Annotation on the binding (`let count: u32 = ...`) tends to be more
//     readable when the parsed value is bound to a name.
//   - Turbofish (`parse::<i64>()`) is more compact in the middle of a
//     longer chain, e.g. `s.split(',').map(|n| n.parse::<i32>()).collect()`.
//
// MENTAL MODEL OF TURBOFISH:
//   `parse` is generic: `fn parse<T: FromStr>(&self) -> Result<T, ...>`.
//   Without context, the compiler can't pick T. The turbofish supplies T
//   directly: `parse::<i64>()` says "T is i64, please".
//
// WHEN NEITHER ANNOTATION HELPS:
//   If you write `let n = "42".parse().unwrap();` and never use `n`, the
//   compiler will refuse with E0282 (`type annotations needed`). The fix
//   is always one of these two annotations.
