// =============================================================================
//  intro2 — printing and formatting
// =============================================================================
//
// `println!` is the everyday way to look at values while learning Rust.
// You'll also see `print!` (no newline), `eprintln!` (stderr), `format!`
// (returns a String instead of printing), and `dbg!` (prints with file:line).
//
// FORMAT STRING BASICS
// ────────────────────
// A format string is a literal that contains placeholder pairs `{}`. Each
// `{}` consumes one argument, in order:
//
//     println!("{} + {} = {}", 2, 3, 2 + 3);   // 2 + 3 = 5
//
// PLACEHOLDER FLAVOURS YOU NEED TO KNOW NOW
//
//     {}      Display      — human-friendly. Implemented for primitives,
//                            String, &str. NOT implemented for collections
//                            like Vec, because there is no single "right"
//                            way to display them.
//
//     {:?}    Debug        — programmer-friendly. Almost every standard type
//                            implements this. When in doubt, use `{:?}`.
//
//     {:#?}   pretty Debug — multi-line pretty form. Great for nested data.
//
// POSITIONAL & NAMED ARGUMENTS
//
//     println!("{0} {1} {0}", "ha", "ho");        // ha ho ha
//     println!("{name} is {age}", name = "Ada", age = 36);
//
// (Since Rust 1.58 you may also write `{name}` and have it captured directly
// from a same-named local variable: `let name = "Ada"; println!("{name}");`)
//
// NUMERIC FORMATTING (you'll meet more later)
//
//     {:5}    width 5, right-aligned         "   42"
//     {:<5}   width 5, left-aligned          "42   "
//     {:05}   width 5, zero-padded number    "00042"
//     {:.3}   3 decimal places               "3.142"
//     {:b}    binary                         "101010"
//     {:x}    lower-case hex                 "2a"
//
// YOUR TASK
// ─────────
// Make the program below compile AND produce EXACTLY this output (the test
// runner will check the exit code, not the text — but you should still get
// the formatting right because solution comparison is part of learning):
//
//     pi ≈ 3.14
//     Ada is 36
//     bytes: [1, 2, 3]
//
// Replace each ??? appropriately.

// I AM NOT DONE

fn main() {
    let pi = 3.14159f64;
    let name = "Ada";
    let age = 36;
    let bytes = [1, 2, 3];

    // Print pi rounded to 2 decimal places.
    println!("pi ≈ {???}", pi);

    // Print "Ada is 36" using NAMED placeholders that capture the locals.
    println!("??? is ???");

    // Print the array using Debug formatting.
    println!("bytes: {???}", bytes);
}
