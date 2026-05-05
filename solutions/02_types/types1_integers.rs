// SOLUTION — types1_integers

fn add_byte(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

fn truncate_to_u8(x: u32) -> u8 {
    x as u8
}

fn mid(a: i32, b: i32) -> i32 {
    a + (b - a) / 2
}

// WHY THIS IS OPTIMAL:
//
//   wrapping_add  — explicit choice for "I want modular arithmetic". If
//                   you wrote `a + b` and a debug build saw 250 + 10, it
//                   would PANIC. wrapping_add documents intent.
//
//   `as u8`       — the standard truncating cast. For 257 → 1 the high
//                   bits are simply dropped. (For unsigned↔signed casts
//                   `as` reinterprets the bits, which can surprise — see
//                   `i8 as u8` for a fun rabbit hole.)
//
//   a + (b - a) / 2 — the "subtract first" midpoint. Compare:
//                       (a + b) / 2          // ❌ overflows when a+b > MAX
//                       a + (b - a) / 2      // ✅ never overflows
//                   Same answer for non-overflowing inputs, never panics
//                   for any pair of i32. Famous interview-grade idiom.
//
// ALTERNATIVES & WHEN TO USE THEM:
//
//   a.checked_add(b)         -> Option<u8>     // for "is this safe?" checks
//   a.saturating_add(b)      -> u8             // clamps to 0..=255
//   a.overflowing_add(b)     -> (u8, bool)     // value + did-it-wrap flag
//
//   For a midpoint of u32 specifically:  a + (b - a) / 2 still works.
//   The standard library now provides `i32::midpoint(a, b)` (Rust 1.85+),
//   which is what you'd write in real code.
