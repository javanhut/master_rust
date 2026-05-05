// =============================================================================
//  macro1 — what `println!` actually does
// =============================================================================
//
// You've been writing `println!("{}", x)` since chapter 1. Time to look
// behind the curtain. `println!` is a MACRO, not a function — that's what
// the trailing `!` means everywhere you see it (`vec!`, `format!`,
// `assert_eq!`, `dbg!`, ...). Macros are code that writes code, expanded
// by the compiler BEFORE type-checking.
//
// WHY ISN'T IT A FUNCTION?
//
// Three reasons a normal `fn` cannot do `println!`'s job:
//
//   1. VARIABLE ARITY. `println!("{}", a)`, `println!("{} {} {}", a, b, c)`,
//      and `println!("hi")` all "call" the same name with different numbers
//      of arguments. Rust functions have a fixed arity. Macros don't.
//
//   2. HETEROGENEOUS ARGUMENT TYPES. Each `{}` placeholder may correspond
//      to a totally different type (`i32`, `&str`, `MyStruct`, ...). A
//      function would need every argument to share a single type, or be
//      a `&dyn Display`. The macro instead generates code that calls the
//      right `Display::fmt` impl for each concrete argument.
//
//   3. COMPILE-TIME FORMAT STRING CHECKING. Try `println!("{} {}", 1)` —
//      the compiler refuses, telling you "1 positional argument in format
//      string, but no arguments were given." That diagnostic happens
//      because the macro PARSES the format literal at compile time and
//      counts placeholders. A function would only see `"{} {}"` as a
//      runtime string and have no chance to check it before execution.
//
// WHAT IT EXPANDS TO (roughly)
//
// `println!("hi {}", 7)` expands to something morally equivalent to:
//
//     {
//         use ::std::io::Write;
//         let mut out = ::std::io::stdout().lock();
//         out.write_fmt(format_args!("hi {}\n", 7)).unwrap();
//     }
//
// `format_args!` is the foundational macro — it builds a `fmt::Arguments`
// value (a fancy struct that holds the format string pieces and pointers
// to the arguments' Display/Debug impls). All of `print!`, `println!`,
// `eprintln!`, `format!`, `write!` boil down to a `format_args!` call.
//
// PARSING HAPPENS AT COMPILE TIME
//
// The format string MUST be a string literal (not a `&str` variable),
// because the compiler tokenizes and walks it during expansion. That's
// also why this fails:
//
//     let s = "{}";
//     println!(s, 7);                    // error: format argument must be a string literal
//
// If you need a runtime-built format string, look at `runtime_format` /
// `Arguments::new_v1` (advanced) or just call `write!` with `format_args!`.
//
// THE FAMILY (for reference)
//
//     print!         — write to stdout, no newline.
//     println!       — write to stdout, append "\n".
//     eprint! / eprintln! — same, but to stderr.
//     format!        — return a `String` instead of writing.
//     write! / writeln!   — write to any `impl Write` (a file, a `String`, a `Vec<u8>`).
//     dbg!           — print "file:line] expr = value" and return the value.
//
// YOUR TASK
// ─────────
// This is a READING exercise. The code below already compiles. Once you've
// digested the comments above, delete the `// I AM NOT DONE` line and
// submit. Run the file in your head: each `println!` invocation expands
// into a tiny block that locks stdout, builds a `format_args!`, and pushes
// formatted bytes through.

// I AM NOT DONE

fn main() {
    // No-arg form — the macro still expands; it just produces a block
    // that writes the literal "hello\n" to stdout.
    println!("hello");

    // One-arg form. The format literal is parsed at compile time;
    // the macro generates code that calls `Display::fmt` on `42_i32`.
    println!("the answer is {}", 42);

    // Multiple args, mixed types. Each placeholder is matched to its own
    // argument's `Display` impl during expansion.
    println!("{} + {} = {}", 2, 3, 2 + 3);

    // Captured-name form (Rust 1.58+). The macro pulls `name` from the
    // surrounding scope while expanding.
    let name = "ferris";
    println!("hello, {name}");

    // `format!` returns a `String` instead of printing — same expansion
    // shape, different terminator.
    let s: String = format!("two plus two is {}", 2 + 2);
    let _ = s;
}
