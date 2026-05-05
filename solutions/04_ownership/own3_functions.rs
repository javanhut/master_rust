// SOLUTION — own3_functions

fn take_and_drop(s: String) {
    let _ = s;
}

fn take_and_return(mut s: String) -> String {
    s.push('!');
    s
}

fn roundtrip() -> String {
    let s = String::from("hi");
    let s = take_and_return(s);
    s
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `take_and_drop` does the simplest possible thing: it accepts ownership
//   and lets the value be dropped at the closing brace. The `let _ = s;`
//   line is purely cosmetic — without it the body would be empty, which
//   also works. We use it so the parameter name shows up in the body and
//   the moment of "drop happens here" is visible.
//
//   `take_and_return` mutates `s` (note the `mut` on the parameter — that's
//   how you say "I plan to mutate this binding inside the function") and
//   then returns the binding by writing `s` as the final expression. Because
//   `String` is not `Copy`, this MOVES the String to the caller. No copy,
//   no clone, no allocation — just a pointer hand-off.
//
//   `roundtrip` shadows the original `s` with the returned `String`. This
//   shadowing pattern is common with the take-and-give-back style.
//
// THE BIG PICTURE:
//
//   The take-and-return dance is what you do BEFORE you know about
//   borrowing. Once you do (own4–own5), you'll write:
//
//       fn add_bang(s: &mut String) { s.push('!'); }
//
//       let mut s = String::from("hi");
//       add_bang(&mut s);
//       // s is still ours, and it's now "hi!"
//
//   That avoids moving the value in and back out. The compiler will
//   produce identical machine code for the two styles in release builds —
//   the difference is purely about who-owns-what at the source level.
//
// COMPILE-FAIL EXPERIMENT (try this in a scratch file):
//
//       let s = String::from("x");
//       take_and_drop(s);
//       println!("{s}");      // error[E0382]: borrow of moved value: `s`
