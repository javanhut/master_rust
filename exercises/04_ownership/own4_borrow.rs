// =============================================================================
//  own4 — shared references (`&T`)
// =============================================================================
//
// Moving and cloning are heavy. Most of the time you want to LOOK at a
// value without taking ownership of it. That's a SHARED REFERENCE:
//
//     fn len(s: &String) -> usize { s.len() }
//
//     let s = String::from("hello");
//     let n = len(&s);          // pass a reference — `s` is NOT moved.
//     println!("{s} has {n}");  // ✅ s still owned by us.
//
// Notation:
//
//     &T          — a shared reference to a `T`. Read-only. Cheap to copy.
//     &x          — produces a shared reference to whatever `x` names.
//     *r          — dereferences (rarely needed; `.method()` auto-derefs).
//
// THE BORROWING RULES (the famous ones)
//
//   At any given time, for a given value, EITHER:
//     - any number of shared references `&T`     (many readers), OR
//     - exactly one mutable reference `&mut T`   (one writer).
//
//   Plus: a reference must never outlive the value it points to.
//
// This exercise focuses on the FIRST half — shared references. You can
// have as many `&T` as you like, simultaneously. None of them can mutate.
//
// AUTO-DEREF
//
// You almost never write `*r` to call a method. `r.len()` works whether
// `r: &String` or `r: String` — the dot operator auto-dereferences.
//
// ALSO USEFUL — `&str`
//
// Everywhere you see `&String`, prefer `&str`. (We dig into slices in own6;
// for now just know that `&str` is the "borrow-a-piece-of-text" type.)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `string_length(s: &String) -> usize` — return the byte length.
//
//   - `sum_of_two(a: &i32, b: &i32) -> i32` — sum two values via shared
//      references. (Yes, `i32` is `Copy` so this is a contrived example —
//      the goal is to practice the syntax `&i32` and dereferencing.)
//
//   - `count_chars(s: &String) -> usize` — count the Unicode characters
//      (NOT bytes) — useful for emoji-safe counting.

// I AM NOT DONE

fn string_length(s: ???) -> usize {
    s.len()
}

fn sum_of_two(a: &i32, b: &i32) -> i32 {
    // Hint: dereference with `*` to get the underlying i32 (or rely on
    //       operator auto-deref — `a + b` works because of `impl Add for &i32`).
    ??? + ???
}

fn count_chars(s: &String) -> usize {
    s.chars().???()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn length_of_hello() {
        let s = String::from("hello");
        assert_eq!(string_length(&s), 5);
        // Crucially, `s` is still ours after the call:
        assert_eq!(s, "hello");
    }

    #[test] fn many_readers_ok() {
        let s = String::from("rust");
        // Multiple shared references at the same time — totally fine.
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;
        assert_eq!(string_length(r1) + string_length(r2) + string_length(r3), 12);
    }

    #[test] fn sums() {
        let a = 10;
        let b = 32;
        assert_eq!(sum_of_two(&a, &b), 42);
    }

    #[test] fn chars_vs_bytes() {
        // "héllo" has 5 chars but 6 bytes (é is two UTF-8 bytes).
        let s = String::from("héllo");
        assert_eq!(count_chars(&s), 5);
        assert_eq!(string_length(&s), 6);
    }
}

fn main() {}
