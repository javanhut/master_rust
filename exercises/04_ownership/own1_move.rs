// =============================================================================
//  own1 — moves
// =============================================================================
//
// Welcome to ownership — Rust's superpower and the thing that makes it
// memory-safe without a garbage collector. The whole chapter rests on
// THREE rules:
//
//   1. Every value has a single OWNER (a variable binding).
//   2. When the owner goes out of scope, the value is dropped (freed).
//   3. There is exactly ONE owner at a time.
//
// Rule 3 is what we explore first.  Watch:
//
//     let s1 = String::from("hello");
//     let s2 = s1;                  // ← MOVE. Ownership transfers from s1 to s2.
//     println!("{}", s1);           // ❌ compile error: borrow of moved value `s1`
//
// `String` owns a heap buffer. If both `s1` and `s2` were valid, BOTH would
// try to free that buffer when their scopes ended — a "double free" bug.
// Rust prevents this at compile time by INVALIDATING `s1` after the move.
//
// COPY vs MOVE
//
// Simple stack-only types like `i32`, `bool`, `char`, fixed-size tuples of
// `Copy` types — these implement the `Copy` trait. For them, `let b = a`
// COPIES the bits, and `a` is still valid:
//
//     let a = 5;
//     let b = a;        // copy
//     println!("{a}");  // ✅ fine — a is still valid
//
// `String`, `Vec<T>`, `Box<T>` — anything that owns heap memory — is NOT
// `Copy`. Assignment moves them.
//
// HOW TO TELL WHICH IS WHICH
//
//   Rule of thumb: if a type holds a pointer to something it owns (a heap
//   allocation, a file handle, a socket), it is NOT `Copy`. It moves.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// `build_greeting` should:
//   - take a `String` called `name` BY VALUE (it owns the input),
//   - build a new `String` "Hello, NAME!" and return it.
//
// `length_after_move` is here to make the move concrete:
//   - we create a `String`,
//   - we move it into `build_greeting`,
//   - the function returns a NEW `String`,
//   - we measure the new one's length and return it.
//
// You may NOT use `.clone()` in this exercise. The point is to feel moves.

// I AM NOT DONE

fn build_greeting(name: String) -> String {
    // Build "Hello, <name>!". format! returns an owned String.
    ???
}

fn length_after_move() -> usize {
    let s = String::from("Ferris");
    let greeting = build_greeting(???);   // move `s` in
    // After this line, `s` is no longer usable — it has been moved.
    greeting.???()                        // return the byte length of `greeting`
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn greets() {
        assert_eq!(build_greeting(String::from("Ferris")), "Hello, Ferris!");
    }
    #[test] fn empty_name() {
        assert_eq!(build_greeting(String::new()), "Hello, !");
    }
    #[test] fn length_is_correct() {
        // "Hello, Ferris!" is 14 bytes.
        assert_eq!(length_after_move(), 14);
    }
}

fn main() {}
