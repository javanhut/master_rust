// =============================================================================
//  intro1 — your first Rust program
// =============================================================================
//
// WELCOME. This is your first lesson. Read every comment slowly.
//
// A Rust program starts at a function called `main`. The runner will compile
// this file and run the resulting binary; passing means "exits with code 0".
//
// SYNTAX TOUR — line by line, before you touch anything:
//
//   fn main() { ... }
//   ─┬  ─┬┬─  ─┬─  ─┬─
//    │   ││    │    │
//    │   ││    │    └── the BODY: a block, opens with `{` closes with `}`.
//    │   ││    └─────── empty parameter list. main takes nothing.
//    │   │└──────────── the function's NAME. `main` is special — it's the
//    │   │              entry point and there must be exactly one.
//    │   └───────────── a SPACE — Rust is whitespace-tolerant.
//    └───────────────── the keyword `fn` introduces a function definition.
//
//   println!("Hello, world!");
//   ──┬──── ┬ ──────┬───────  ┬
//     │     │       │         └── every STATEMENT ends with `;`.
//     │     │       └──────────── a STRING LITERAL: text between double quotes.
//     │     └──────────────────── parentheses hold the macro's arguments.
//     │                            (looks just like a function call.)
//     └──────────────────────────  the `!` at the end means this is a MACRO,
//                                  not a normal function. Macros expand into
//                                  code at compile time. `println!` expands
//                                  into a checked call to `print to stdout
//                                  followed by newline`. We use a macro here
//                                  because format strings are checked at
//                                  compile time — you'll learn macros later.
//
// YOUR TASK
// ─────────
// 1. Delete the line that says  `// I AM NOT DONE`  below this block.
//    The runner refuses to compile any exercise that still contains it —
//    this forces you to read the file first.
// 2. Replace the `???` so the program prints exactly:
//
//        Hello, master_rust!
//
//    (no extra spaces, lowercase r in `rust`, exclamation mark at the end)
//
// Save the file. The runner is watching — it will re-compile and tell you
// whether it passed.

// I AM NOT DONE

fn main() {
    println!(???);
}
