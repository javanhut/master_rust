// =============================================================================
//  own7 — dangling references (and how Rust refuses to make them)
// =============================================================================
//
// In C and C++, this kind of bug is famous and easy to write:
//
//     // C-ish pseudo-code
//     char* dangle() {
//         char buf[16] = "hi";
//         return buf;          // ← returns a pointer to a local
//     }                        //   the local is gone the moment we return.
//
// The pointer points into the dead stack frame. Reading through it is
// undefined behaviour. Whole categories of CVEs live here.
//
// THE RUST EQUIVALENT — REJECTED AT COMPILE TIME
//
//     fn dangle() -> &String {
//         let s = String::from("x");
//         &s                    // ← reference to a local
//     }                         //   `s` is dropped here; the reference
//                               //   would point to nothing.
//
// The compiler says, roughly: "your function returns a `&String`, but the
// `String` it points to was created here and is dropped at the end of this
// function. The reference would outlive its referent. NO."
//
// You'll see error[E0106] (missing lifetime specifier) or E0515 (cannot
// return reference to local variable), depending on exactly how you wrote
// it. Both are the borrow checker doing its job.
//
// THE FIX
//
// You have two choices. The good one is almost always:
//
//   ▶ RETURN THE OWNED VALUE.
//
//     fn no_dangle() -> String {
//         let s = String::from("x");
//         s                     // moves ownership to the caller
//     }
//
// Now the caller owns `s`, the value lives as long as the caller wants,
// and there's no reference to dangle. This is the idiomatic Rust answer
// to "I want to construct a thing inside a function and hand it back".
//
// (The other choice is to take a reference IN as a parameter and return
// a sub-slice of it — that ties the output's lifetime to the input. We
// touch that pattern in own8.)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Below is a function that the compiler currently REFUSES to accept.
// Fix `make_greeting` so it compiles. The fix is a one-character change
// to the return type plus a one-line change to the body.
//
// Goal: `make_greeting()` should hand the caller an OWNED `String`
// containing "hello".
//
// This exercise is "compile mode" — passing means the file compiles
// cleanly. There are no tests.

// I AM NOT DONE

// BROKEN — fix it.
//
//   fn make_greeting() -> &String {
//       let s = String::from("hello");
//       &s
//   }
//
// Replace the ??? on the next two lines with a working version.

fn make_greeting() -> ??? {
    let s = String::from("hello");
    ???
}

fn main() {
    let g = make_greeting();
    println!("{g}");
    // We OWN g here. When this function ends, g is dropped, and that's
    // perfectly fine — there's no outstanding reference into it.
}
