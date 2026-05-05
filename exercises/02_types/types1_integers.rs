// =============================================================================
//  types1 — integers
// =============================================================================
//
// Integer types in Rust:
//
//     signed:    i8   i16   i32   i64   i128   isize
//     unsigned:  u8   u16   u32   u64   u128   usize
//
// The number is the WIDTH IN BITS. The unsigned versions can hold larger
// positive numbers but cannot hold negatives. `isize`/`usize` are
// pointer-sized — 64 bits on a typical 64-bit machine. They are what you
// use for indexing into collections.
//
//     i8       — range  -128 .. 127
//     u8       — range     0 .. 255
//     i32      — range  ≈ ±2.1 billion           ← the DEFAULT for an
//                                                  unsuffixed integer literal
//     u64      — range  0 .. ≈ 1.8 × 10^19
//
// LITERAL SYNTAX
//
//     42        // i32
//     42i64     // i64
//     42u8      // u8
//     0xff      // hex
//     0o77      // octal
//     0b1010    // binary
//     1_000_000 // underscore is a digit separator, helps readability
//
// ARITHMETIC
//
//     +  -  *  /  %        // % is remainder, NOT modulo for negatives
//     a / b                // INTEGER division — fractional part discarded
//                          //   7 / 2 == 3
//
// OVERFLOW
//
// In debug builds, integer overflow PANICS. In release builds it wraps
// silently — but you can opt in to specific behavior:
//
//     a.checked_add(b)        -> Option<T>   None on overflow
//     a.wrapping_add(b)       -> T           always wraps
//     a.saturating_add(b)     -> T           clamps to MIN / MAX
//     a.overflowing_add(b)    -> (T, bool)   the wrapped value + a flag
//
// CASTING
//
// Use `as` for explicit numeric conversion. It TRUNCATES on narrowing.
//
//     let big: i32 = 257;
//     let small = big as u8;     // 1   (257 mod 256)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Make `add_byte` add two `u8` without panicking on overflow — wrap.
//   - Make `truncate_to_u8` cast from u32 down to u8.
//   - Make `mid` return the integer midpoint of two i32 values without
//     overflow on huge inputs. (Hint: don't write `(a + b) / 2`.)

// I AM NOT DONE

fn add_byte(a: u8, b: u8) -> u8 {
    a.???(b)              // a method on u8 that wraps on overflow
}

fn truncate_to_u8(x: u32) -> u8 {
    x ??? u8              // cast operator
}

fn mid(a: i32, b: i32) -> i32 {
    // Classic "average without overflow" trick:
    //   a + (b - a) / 2
    // Fill in the body so it compiles AND passes the giant-numbers test.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn add_byte_normal() { assert_eq!(add_byte(10, 20), 30); }
    #[test] fn add_byte_wraps()  { assert_eq!(add_byte(250, 10), 4); } // 260 mod 256

    #[test] fn truncate()        { assert_eq!(truncate_to_u8(257), 1); }

    #[test] fn mid_simple()      { assert_eq!(mid(2, 8), 5); }
    #[test] fn mid_negative()    { assert_eq!(mid(-10, 10), 0); }
    #[test] fn mid_no_overflow() { assert_eq!(mid(i32::MAX, i32::MAX - 2), i32::MAX - 1); }
}

fn main() {}
