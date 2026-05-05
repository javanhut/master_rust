// SOLUTION — own7_dangling

fn make_greeting() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let g = make_greeting();
    println!("{g}");
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The bug was the return type `&String`. The function constructs a
//   `String` in a local binding `s`, and the local is dropped at the
//   closing `}`. Returning `&s` would hand the caller a reference to
//   freed memory — a classic dangle. Rust will not allow it.
//
//   The fix is to return `String` instead of `&String`. Now the function
//   MOVES the owned String to the caller. The String's lifetime is bound
//   to the caller's binding, not to this function's stack frame. No
//   dangle, no leak, no allocation beyond the one we already made.
//
// HOW THE BORROW CHECKER PHRASES THE ORIGINAL ERROR:
//
//   error[E0106]: missing lifetime specifier
//     --> src/main.rs:1:23
//      |
//   1  | fn make_greeting() -> &String {
//      |                       ^ expected named lifetime parameter
//
//   And if you try to add a lifetime, you get:
//
//   error[E0515]: cannot return reference to local variable `s`
//
//   The compiler is telling you, in two different voices, the same thing:
//   you're trying to ship a reference outlasting the value it points to.
//
// WHEN IT'S OK TO RETURN A REFERENCE:
//
//   When the reference points into something you DIDN'T create — typically
//   a function parameter:
//
//       fn first_byte(s: &str) -> &u8 {
//           &s.as_bytes()[0]
//       }
//
//   Here the returned `&u8` borrows from the caller's `s`, not from a
//   local. The borrow checker is happy. We do this pattern in own6's
//   `first_word` and own8's `longest`.
//
// RULE OF THUMB:
//
//   - "I built it in here" → return it by value.
//   - "I'm pointing into something the caller gave me" → return a reference.
