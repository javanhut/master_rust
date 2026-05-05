// SOLUTION — intro6_constants

const SECONDS_PER_HOUR: u32 = 60 * 60;
const SECONDS_PER_DAY:  u32 = SECONDS_PER_HOUR * 24;

fn days_to_seconds(days: u32) -> u32 {
    days * SECONDS_PER_DAY
}

// WHY THIS IS OPTIMAL:
//   - The two constants compose: SECONDS_PER_DAY is defined IN TERMS OF
//     SECONDS_PER_HOUR, so the relationship is documented in code, not
//     just in the head of whoever wrote it.
//   - Both are `u32`; `days * SECONDS_PER_DAY` stays in `u32`.
//
// COMMON BEGINNER MISTAKES:
//
//   1. Forgetting the type annotation:
//        const SECONDS_PER_DAY = SECONDS_PER_HOUR * 24;   // ❌
//      Constants always need an explicit type.
//
//   2. Using `let`:
//        let SECONDS_PER_DAY = ...;                       // ❌ at module scope
//      `let` is only valid inside functions.
//
//   3. Lowercase name — compiles, but you'll get a warning, and
//      capitalised names are how Rust signals "this is a constant".
//
// WHY NOT `static`?
//   `static SECONDS_PER_DAY: u32 = ...;` would also work but represents an
//   actual addressable global. Use `static` for truly globally-shared
//   resources (rare). For pure compile-time numbers use `const`.
