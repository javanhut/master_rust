// =============================================================================
//  proj1_skeleton — chapter 20: the capstone project begins
// =============================================================================
//
// Welcome to the FINAL chapter. Across the next eight files you will build
// a small command-line tool — a Rust-powered `wc` with extras. By the end
// it will:
//
//   - count lines, words, and chars in a body of text,
//   - report the top-K most frequent words,
//   - parse its own command-line flags (`--lines`, `--words`, `--top 5`),
//   - and split the work across threads.
//
// THE CHAPTER PLAN
//
//     proj1_skeleton    you are here — print a greeting, lock in the layout
//     proj2_io          read_input — string-handling, &str vs String, ?
//     proj3_types       Stats struct + CountUnit enum (chapters 9 + 10)
//     proj4_parse       count_basic — iterators (chapter 7) over &str
//     proj5_logic       top_words — HashMap, sort_by, take (chapters 6 + 16)
//     proj6_concurrency count_parallel — thread::scope (chapter 14)
//     proj7_cli         tie it all together with an argv parser
//     proj_quiz         summarise(text, k) — final integration test
//
// EVERY chapter so far shows up here:
//
//   - ownership & borrowing      (ch 4)         — passing &str around
//   - strings                    (ch 5)         — to_string / split_whitespace
//   - collections                (ch 6)         — HashMap of word counts
//   - iterators                  (ch 7)         — .lines().count() and friends
//   - error handling             (ch 8)         — Result / ? in the IO layer
//   - structs                    (ch 9)         — Stats, Summary
//   - enums                      (ch 10)        — CountUnit
//   - traits                     (ch 11)        — Display impl on Stats
//   - modules                    (ch 12)        — single-file but logically split
//   - smart pointers             (ch 13)        — Vec, String — heap allocations
//   - concurrency                (ch 14)        — thread::scope
//   - errors at scale            (ch 15)        — custom error enum in proj2
//   - closures                   (ch 16)        — sort_by(|a, b| ...)
//   - lifetimes                  (ch 17)        — &str return types
//
// EACH FILE IS SELF-CONTAINED. We don't share a `mod` across files — every
// later exercise re-declares the types it needs. That's the rustlings
// pattern: open one file, see the whole story.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// This first one is a freebie. Replace `???` with a string and remove the
// `// I AM NOT DONE` line. The runner will compile this file and just run
// it — exit code 0 means PASS.

// I AM NOT DONE

fn main() {
    let project_name: &str = ???;
    println!("starting capstone project: {project_name}");
}
