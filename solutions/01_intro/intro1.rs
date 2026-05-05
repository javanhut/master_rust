// SOLUTION — intro1
//
// The whole point is to build muscle memory for the Rust ceremony around
// `main`. Once you've done a hundred of these, the structure should feel
// invisible.
//
// Everyone's first instinct is to write println!("Hello, world!");
// — but the exercise asked for "Hello, master_rust!" exactly.

fn main() {
    println!("Hello, master_rust!");
}

// WHY THIS IS OPTIMAL:
//   - Single statement, idiomatic, no extra allocation, no unnecessary
//     `format!` or temporary variables. `println!` writes the literal
//     directly with a newline appended.
//
// ALTERNATIVES YOU MIGHT SEE:
//   1.  let msg = "Hello, master_rust!";
//       println!("{msg}");
//       Fine, slightly slower to read for one-line programs. Useful when
//       the same string is referenced multiple times.
//
//   2.  println!("{}", "Hello, master_rust!");
//       Works but redundant — the literal could just be the format string.
//
//   3.  print!("Hello, master_rust!\n");
//       Equivalent. `println!` is the convention because forgetting `\n` is
//       a common bug, and `println!` always flushes the line.
