// =============================================================================
//  lt1 — what `'a` actually IS
// =============================================================================
//
// You've already met lifetimes briefly (own_quiz, closures_quiz). Now we slow
// down and ask the question those earlier exercises sidestepped: what IS a
// lifetime? What does the compiler actually do when it sees `'a`?
//
// THE ONE-SENTENCE ANSWER
//
//   A lifetime is a REGION OF CODE — a contiguous span of source where a
//   reference is allowed to be used.
//
// It is NOT a duration in seconds. It is NOT a runtime concept. It does not
// exist in the compiled binary. It's purely a label the compiler attaches
// to references at compile time so it can prove the following invariant:
//
//   ▶ EVERY REFERENCE IS USED ONLY INSIDE THE SCOPE ITS REFERENT IS ALIVE.
//
// If the compiler can't prove that, it refuses to build the program. That's
// the borrow checker doing its job.
//
// A TINY MENTAL MODEL
//
//   {
//       let s = String::from("hi");   // ┐ s exists from here…
//       let r = &s;                   // │ r borrows from s
//       println!("{r}");              // │ r used — fine, s still alive
//   }                                 // ┘ …to here. r and s die together.
//
// The compiler labels `s`'s scope with some lifetime, call it `'s`. The
// reference `r` is given a lifetime `'r` such that `'r ⊆ 's` — `r`'s region
// of valid use is CONTAINED INSIDE `s`'s region of existence. As long as
// every USE of `r` falls inside `'r`, you're safe.
//
// The compiler picks these regions automatically; you usually never see
// them. You only have to write `'a` when the compiler can't figure out the
// relationship on its own (e.g., a function returning a reference derived
// from one of two inputs — `longest`).
//
// COMMON CONFUSIONS, RESOLVED
//
//   - "A lifetime is how long a value lives." — Almost. It's how long a
//     REFERENCE is allowed to be used; the value's actual lifetime is at
//     least that long. The compiler proves the inequality.
//
//   - "Annotating `'a` makes the value live longer." — NO. Annotations
//     describe relationships; they do not change what the program does at
//     runtime. They never extend or shorten anything. They only let you
//     (or refuse to let you) compile.
//
//   - "Every reference has a lifetime I can name." — Every reference HAS a
//     lifetime, but most of them are anonymous (`'_`) and inferred. You
//     name one only when needed.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Below is a function `first_char` that the compiler currently REJECTS.
// The bug is the classic one from the model above: a reference is being
// kept alive past the end of the scope its referent lives in.
//
// Fix `first_char` so the file compiles. The fix is one line — move the
// declaration of `s` so its scope contains every USE of the reference
// `c` that points into it. (Hint: pull `s` out of the inner block.)
//
// This exercise is "compile mode" — passing means the file compiles
// cleanly. There are no tests.

// I AM NOT DONE

// BROKEN — fix it.
//
//   fn first_char() -> char {
//       let c: &char;
//       {
//           let s = vec!['h', 'i'];
//           c = &s[0];
//       }   // ← `s` is dropped here, but `c` still points into it.
//       *c  // using `c` after its referent is gone — UB if allowed.
//   }
//
// Rewrite the body so `s` outlives every use of `c`. Replace the ???.

fn first_char() -> char {
    ???
}

fn main() {
    let c = first_char();
    println!("first char is {c}");
    // The point of this exercise is not the value — it's that the borrow
    // checker accepts the program. If it compiles, you understood the
    // scope-vs-reference relationship.
}
