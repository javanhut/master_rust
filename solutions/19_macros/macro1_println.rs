// SOLUTION — macro1_println

fn main() {
    println!("hello");
    println!("the answer is {}", 42);
    println!("{} + {} = {}", 2, 3, 2 + 3);

    let name = "ferris";
    println!("hello, {name}");

    let s: String = format!("two plus two is {}", 2 + 2);
    let _ = s;
}

// WHY THIS IS OPTIMAL:
//
//   A reading exercise — the only "work" was understanding what `println!`
//   actually is. No code change needed beyond removing the
//   `I AM NOT DONE` marker.
//
// THE FOUR THINGS TO REMEMBER:
//
//   1. `name!(...)` is a MACRO, not a function. The `!` is the giveaway.
//   2. Macros are expanded by the compiler BEFORE type-checking. They can
//      do things normal functions cannot — variable arity, heterogeneous
//      arg types, compile-time validation of format literals.
//   3. `print!` / `println!` / `eprintln!` / `format!` / `write!` all
//      delegate to `format_args!`, which builds a `core::fmt::Arguments`
//      value. That's the single source of truth for formatting.
//   4. The format string MUST be a string literal — a runtime `&str` won't
//      work because the macro tokenizes the literal during expansion.
//
// EXPANSION DEMO (paraphrased):
//
//     println!("hello, {name}");
//
//   becomes something like:
//
//     {
//         use ::std::io::Write;
//         let mut out = ::std::io::stdout().lock();
//         out.write_fmt(format_args!("hello, {0}\n", name)).unwrap();
//     }
//
//   You can confirm with `cargo expand` (a cargo subcommand) on a real
//   project — outside the scope of this single-file exercise.
//
// COMMON MISTAKES:
//
//   - Trying to pass a runtime String as the format literal. The compiler
//     wants a literal so it can parse placeholders at compile time.
//   - Confusing `println!` with `print!` and forgetting the trailing
//     newline (or doubling up).
//   - Reaching for `dbg!` in production code. `dbg!` is wonderful in
//     development — it prints `file:line] expr = value` and returns the
//     value — but leaves clutter in release builds. Reach for `tracing` or
//     `log` for real telemetry (a later topic).
