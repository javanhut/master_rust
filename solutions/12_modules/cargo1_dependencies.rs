// SOLUTION — cargo1_dependencies

fn edition_label() -> &'static str {
    "2024"
}

fn main() {
    let label = edition_label();
    let _ = label;
}

// WHY THIS IS OPTIMAL:
//
//   This exercise is a READING exercise. The "code" exists only so the
//   runner has something to compile. The actual material — Cargo.toml
//   layout, semver requirements, features, and `cargo add` — is in the
//   exercise comments and applies as soon as you leave the single-file
//   sandbox and start a real `cargo new` project.
//
// QUICK REFERENCE — THE FOUR THINGS TO REMEMBER:
//
//   1. `cargo new myapp` (binary) or `cargo new mylib --lib` (library).
//   2. `cargo add <crate>` writes the dependency to Cargo.toml — don't
//      edit the file by hand for routine adds.
//   3. Version strings like `"1.0"` mean ">=1.0.0, <2.0.0". For `0.x`
//      crates, each minor bump is treated as breaking.
//   4. Features are named toggles enabled with
//      `serde = { version = "1.0", features = ["derive"] }`. Useful when a
//      crate offers optional functionality you only sometimes want.
//
// COMMON MISTAKES TO AVOID:
//
//   - Pinning to "*" or "0" — leaves you at the resolver's mercy and makes
//     reproducible builds harder. Pick a real floor.
//   - Hand-editing Cargo.lock — the resolver owns it. To bump versions
//     within your Cargo.toml constraints, run `cargo update`.
//   - Re-implementing what a stable crate already does. Search crates.io
//     and lib.rs first; chances are the boring stuff (CLI parsing,
//     serialisation, HTTP) has a well-loved solution.
//
// FILE-BASED NOTE:
//
//   Once you graduate from this single-file world to a real Cargo project,
//   the modules from mod1–mod4 each live in their own files (`src/foo.rs`
//   or `src/foo/mod.rs`), declared at the crate root with `mod foo;` /
//   `pub mod foo;`. The semantics are identical to the inline form — only
//   where the source lives changes.
