// =============================================================================
//  cargo1 — Cargo, crates, dependencies (a guided tour)
// =============================================================================
//
// This exercise is mostly prose. The Rust file you're looking at compiles
// trivially; the goal here is to LEARN Cargo so the rest of your Rust
// career stops looking like magic.
//
// CARGO IS RUST'S BUILD SYSTEM AND PACKAGE MANAGER
//
// You've been compiling single files with `rustc` so far so the runner can
// teach modules in isolation. In real life you almost never call `rustc`
// directly. You let Cargo do it. Cargo:
//
//   - resolves dependencies from a registry (crates.io by default),
//   - downloads and compiles them,
//   - runs your tests,
//   - builds binaries and libraries,
//   - manages release vs. debug profiles,
//   - hands off to `rustc` under the hood with the right flags.
//
// CRATE vs. PACKAGE vs. MODULE
//
//   - A MODULE  is the `mod foo { ... }` thing you've been learning.
//   - A CRATE   is a single compilation unit — either a binary (with
//                 a `main` function) or a library (with a `lib.rs`).
//   - A PACKAGE is one or more crates plus a `Cargo.toml`. Most packages
//                 contain exactly one library or one binary; some contain
//                 several binaries plus a shared library.
//
// CARGO.TOML — THE MANIFEST
//
// Every package has a `Cargo.toml` at its root. Minimal example:
//
//     [package]
//     name = "mything"
//     version = "0.1.0"
//     edition = "2024"
//
//     [dependencies]
//     serde = "1.0"
//     anyhow = "1"
//
//     [dev-dependencies]
//     # Used only for `cargo test` and examples; not shipped to users.
//     proptest = "1"
//
//     [build-dependencies]
//     # Used by build.rs scripts at compile time.
//
// The `edition` field is the Rust edition — a backwards-compatible language
// snapshot. This course uses `edition = "2024"`.
//
// SEMVER — VERSION REQUIREMENTS
//
// Crates publish versions that follow SemVer: MAJOR.MINOR.PATCH.
//
//     "1.0"        — actually means ">=1.0.0, <2.0.0" — any 1.x.y.
//     "1.2.3"      — means ">=1.2.3, <2.0.0".
//     "=1.2.3"     — exact pin.
//     "^1.2.3"     — same as "1.2.3" — explicit caret form.
//     "~1.2.3"     — ">=1.2.3, <1.3.0" — only patch updates.
//     "*"          — anything (don't do this).
//
// The first non-zero digit is the "compatibility floor." For `0.x.y`
// versions, every minor bump is treated as breaking — `"0.4"` means
// `>=0.4.0, <0.5.0`. That's why pre-1.0 crates churn so much.
//
// `Cargo.lock` records the EXACT versions the resolver picked for the
// current build. Commit it for binaries; for libraries it's optional
// (your downstreams resolve their own). Don't edit it by hand.
//
// `cargo add` — THE EASY WAY
//
// Modern Cargo ships `cargo add` (since 1.62). Just run:
//
//     cargo add serde                   # latest version, written to Cargo.toml
//     cargo add serde --features derive # opt into a feature
//     cargo add tokio --features full
//     cargo add proptest --dev          # adds to [dev-dependencies]
//     cargo add my_local --path ../my_local
//     cargo add my_git   --git https://github.com/foo/bar
//
// FEATURES
//
// A FEATURE is a named compile-time toggle that turns optional code on or
// off. Crates expose features so callers can pay only for what they use:
//
//     [dependencies]
//     serde = { version = "1.0", features = ["derive"] }
//     tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
//
// Features are addatitive — if two of your dependencies pull in `serde`
// with different features, Cargo unions them. Don't try to "subtract."
//
// You can also disable a crate's default features and opt back in:
//
//     reqwest = { version = "0.12", default-features = false, features = ["json"] }
//
// COMMON CARGO COMMANDS
//
//     cargo new myapp            # binary package (src/main.rs)
//     cargo new mylib --lib      # library package (src/lib.rs)
//     cargo build                # debug build
//     cargo build --release      # optimised build
//     cargo run                  # build + run
//     cargo test                 # run tests
//     cargo test some_pattern    # run tests whose name matches
//     cargo check                # type-check without producing a binary (FAST)
//     cargo doc --open           # build & open the docs for your deps
//     cargo fmt                  # format the code
//     cargo clippy               # extra lints
//     cargo update               # bump Cargo.lock to latest matching versions
//     cargo tree                 # show the dependency graph
//
// WORKSPACES (preview)
//
// A workspace is a directory containing several packages that share a
// `target/` directory and a single `Cargo.lock`:
//
//     [workspace]
//     members = ["crates/parser", "crates/runtime", "apps/cli"]
//
// Useful for splitting a project into multiple crates without juggling
// separate dependency trees. Out of scope for this chapter; just know it
// exists.
//
// =============================================================================
//  YOUR (TINY) TASK
// =============================================================================
// The Rust code below is self-contained — no external crates, no Cargo.toml.
// Its only purpose is to compile cleanly so the runner can mark you
// complete. Read the comments above thoroughly, then DELETE the
// `// I AM NOT DONE` line and submit.
//
// You'll know this material clicked when, in a fresh project on your own
// machine, you can:
//
//   1. `cargo new playground && cd playground`
//   2. `cargo add anyhow`
//   3. open `Cargo.toml` and recognise every line
//   4. write `use anyhow::Result;` in `src/main.rs` and have it compile.

// I AM NOT DONE

/// Returns the Rust edition this exercise is compiled with — purely so the
/// file does something runnable. The teaching is in the comments above.
fn edition_label() -> &'static str {
    "2024"
}

fn main() {
    let label = edition_label();
    // No assertion, no test — the file just needs to compile and run.
    let _ = label;
}
