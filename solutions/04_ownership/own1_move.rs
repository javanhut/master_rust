// SOLUTION — own1_move

fn build_greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

fn length_after_move() -> usize {
    let s = String::from("Ferris");
    let greeting = build_greeting(s);
    greeting.len()
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `format!` is the bread-and-butter way to build an owned `String` from
//   pieces. It returns a brand-new heap String, leaving its arguments
//   alone (well — `name` is moved IN, but format! reads it via Display
//   and the resulting String is fresh).
//
//   `greeting.len()` returns the byte length, which is what `.len()`
//   always means on a `String`/`&str` — bytes, not characters. For ASCII
//   text those are the same. For UTF-8 they aren't; `.chars().count()`
//   gives the Unicode-scalar count.
//
// WHAT MOVE FEELS LIKE:
//
//     let s = String::from("Ferris");
//     let greeting = build_greeting(s);
//     // println!("{s}");   // ❌ borrow of moved value
//
//   At compile time the compiler tracks that `s` was passed by value into
//   a function that takes ownership. After that line, `s` is poisoned —
//   not because the bytes are gone (they aren't), but because the COMPILER
//   refuses to let you touch a value whose ownership it has already given
//   away. There is no runtime check; this is purely static analysis.
//
// ALTERNATIVES:
//
//   - `name + "!"` (using `String + &str`) — works but less obvious, and
//     the `+` operator on String *moves* the left operand. format! is
//     clearer for anything beyond a single concat.
//
//   - `String::with_capacity(n).push_str(...)` — the manual route, useful
//     when you can size up-front and you're in a hot loop. Overkill here.
