# master_rust

A beginner-friendly, deeply-explained, Rustlings-style course that takes you from zero to professional-level Rust — without assuming you've read The Book or already know what `iter` or `unwrap` mean.

Every exercise teaches **one concept at a time** with a long teaching block at the top of the file, then asks you to fill in `???` blanks. When you save, the watcher recompiles and tells you whether you passed. Once you pass, you can compare your code against an **annotated optimal solution** that explains *why* the optimal answer is optimal and what alternatives a senior Rust developer would consider.

---

## Quick start

```sh
# Build the runner once.
cargo build --release

# Then, from the repo root, just run it. With no args it enters watch mode
# and starts you on your first pending exercise.
cargo run --release
```

Or install the binary into your PATH:

```sh
cargo install --path .
master_rust          # watch mode — your daily driver
```

Open the exercise file the runner names (it'll be something like `exercises/01_intro/intro1.rs`). Read every comment. Edit. Save. Repeat.

---

## Commands

| Command                              | What it does                                                  |
|--------------------------------------|---------------------------------------------------------------|
| `master_rust`                        | Watch mode — your default. Auto re-runs the current exercise. |
| `master_rust list`                   | Every exercise + ✓ / · for done / pending.                    |
| `master_rust run [name]`             | Run a specific exercise once (or the next pending).           |
| `master_rust hint [name]`            | Print the hint block for the exercise.                        |
| `master_rust solution <name>`        | Show the annotated optimal solution (only after you pass).    |
| `master_rust solution <name> --force`| Peek at the solution before passing (it's your call).         |
| `master_rust reset <name>`           | Forget that you completed an exercise.                        |
| `master_rust progress`               | Big progress bar.                                             |

Watch mode is the intended day-to-day flow. Every time you save any `.rs` file under `exercises/`, the runner picks up the next pending exercise, recompiles, and prints either `✓ passed` or the compiler error.

---

## How an exercise is structured

```
exercises/01_intro/intro4_mutability.rs       ← what you edit
solutions/01_intro/intro4_mutability.rs       ← shown after you pass
```

Each exercise file contains, in order:

1. **The lesson** — a long top-of-file comment that teaches the concept from first principles, with code samples and common pitfalls.
2. **The marker** — a single `// I AM NOT DONE` line. The runner refuses to compile any file that still contains this line. Delete it once you've actually read the lesson and made your first attempt.
3. **The skeleton** — code with `???` blanks for you to fill in.
4. **The tests** — a `#[cfg(test)] mod tests` block that determines whether you passed. You don't need to edit it, but reading it is part of understanding the exercise.

Solutions are *full* drop-in replacements with embedded commentary on **why** they're optimal and what alternatives exist.

---

## Course outline

> **Currently shipping: all 20 chapters** (146 exercises, every solution verified end-to-end).

### ✅ Chapter 1 — Intro & Variables (8 exercises)

`println!` and formatting · `let` & immutability · `let mut` · shadowing · `const` · type annotations & turbofish · capstone quiz.

### ✅ Chapter 2 — Primitive types (6 exercises)

Integers (signed/unsigned, overflow handling) · floats (and why you can't add `i32 + f64`) · `bool` and `char` · tuples · arrays vs slices · capstone quiz.

### ✅ Chapter 3 — Functions & control flow (6 exercises)

Function signatures · expressions vs statements · `if`/`else if`/`else` as an expression · `loop`/`while`/`for` · early returns · capstone (FizzBuzz).

### Coming next

- Chapter 4 — Ownership, borrowing, lifetimes
- Chapter 5 — Strings (`String` vs `&str`, `to_string`, `to_owned`, `&str` slicing)
- Chapter 6 — `Vec<T>`, `HashMap<K,V>`, slicing
- Chapter 7 — Iterators (`map`, `filter`, `fold`, `collect`, the iterator method zoo)
- Chapter 8 — `Option<T>`, `Result<T, E>`, `?`, `unwrap`/`expect` and when each is right
- Chapter 9 — Structs, methods, `impl` blocks
- Chapter 10 — Enums and pattern matching
- Chapter 11 — Traits (the heart of Rust), generics, trait objects
- Chapter 12 — Modules, crates, Cargo workspaces
- Chapter 13 — Smart pointers (`Box`, `Rc`, `RefCell`, `Arc`)
- Chapter 14 — Concurrency (`thread`, channels, `Mutex`, `Send`/`Sync`)
- Chapter 15 — Error handling at scale (`thiserror`, `anyhow`, custom error types)
- Chapter 16 — Closures, `Fn`/`FnMut`/`FnOnce`, the function-trait hierarchy
- Chapter 17 — Lifetimes deep-dive
- Chapter 18 — Async/await
- Chapter 19 — Macros (declarative + a taste of procedural)
- Chapter 20 — A real project: build a CLI tool from scratch

---

## Philosophy

Where Rustlings says *"this is a let binding"*, master_rust says *"this is a let binding, here is what's actually happening with the type-inference engine, here is what `mut` does and doesn't change, here is the compiler error you'll get if you forget it, and here is what a senior Rust developer would write instead."*

- **No prerequisite reading.** You shouldn't need to context-switch to The Book mid-exercise. Every concept is introduced in the file that uses it.
- **Read before you write.** Every exercise opens with a substantial teaching block. The `// I AM NOT DONE` marker is there to make sure you actually read it.
- **Short feedback loops.** Watch mode → save → green checkmark or red error in under a second.
- **Optimal answers, with reasoning.** A passing solution is not always a good solution. Every exercise has an annotated reference that walks through *why* the optimal answer is optimal and what alternatives a Rust pro would consider.
- **Tests, not vibes.** Every exercise that asks you to implement something is graded by `#[test]` blocks the runner executes. No "looks right to me."

---

## Repo layout

```
master_rust/
├── Cargo.toml
├── info.toml                  ← exercise manifest (order, mode, hints)
├── src/main.rs                ← the runner
├── exercises/
│   ├── 01_intro/*.rs          ← the lessons + skeletons
│   ├── 02_types/*.rs
│   └── 03_flow/*.rs
└── solutions/
    └── …same layout…          ← annotated optimal solutions
```

Progress lives in `.master_rust_progress` (gitignored). Per-exercise compile artifacts go to `.master_rust_build/` (gitignored). Wipe either at any time.

---

## Requirements

- A Rust toolchain (`rustc` and `cargo`) — `rustup` install is fine. The course targets the 2021 edition; any rustc from the last few years works.

That's it. No other dependencies.
