// =============================================================================
//  proj2_io — reading input
// =============================================================================
//
// A real `wc` reads from stdin (or a file path on argv). For this course
// we want self-contained, deterministic tests — so the testable function
// takes a `&str`. The "real" stdin path is shown in commentary only.
//
// THE REAL-WORLD STDIN PATH (commentary)
//
//   use std::io::{self, Read};
//
//   fn main() -> io::Result<()> {
//       let mut buf = String::new();
//       io::stdin().read_to_string(&mut buf)?;     // ?  bubbles io::Error
//       let normalised = read_input(&buf);
//       println!("{normalised}");
//       Ok(())
//   }
//
//   - `read_to_string` slurps the entire stream into one String. For
//     large inputs you'd loop with `BufRead::read_line` instead, to
//     stream chunk-by-chunk.
//   - `?` works because main returns `io::Result<()>`. Chapter 8 lives on.
//
// WHAT WE ACTUALLY TEST
//
//   `read_input(input: &str) -> String` returns a NORMALISED copy of the
//   input:
//     - trim leading/trailing whitespace,
//     - convert any `\r\n` to `\n` (so Windows line endings don't skew
//       counts later).
//
// WHY THIS SHAPE?
//
//   By isolating the "produce a clean String from a &str" step we make
//   every later stage easy to test. The IO boundary is where errors
//   live; the pure-text functions in proj3..proj6 stay total.
//
// CONCEPTS IN PLAY
//
//   - chapter 4 ownership: we RETURN a String (owned heap allocation)
//     because the caller wants to keep it after we're gone.
//   - chapter 5 strings: `replace`, `trim`, `to_string`.
//   - chapter 8 errors: `?` — only used in the commentary stdin path.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `read_input` so the tests pass. Replace each `???`.

// I AM NOT DONE

fn read_input(input: &str) -> String {
    // Step 1 — collapse Windows-style line endings.
    let unix = input.???("\r\n", "\n");

    // Step 2 — strip leading/trailing whitespace, then own it.
    unix.???().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn trims_outer_whitespace() {
        assert_eq!(read_input("  hello  "), "hello");
    }

    #[test] fn keeps_inner_whitespace() {
        assert_eq!(read_input("a b\tc"), "a b\tc");
    }

    #[test] fn normalises_crlf() {
        assert_eq!(read_input("one\r\ntwo\r\nthree"), "one\ntwo\nthree");
    }

    #[test] fn empty_input_is_empty_string() {
        assert_eq!(read_input(""), "");
    }

    #[test] fn whitespace_only_becomes_empty() {
        assert_eq!(read_input("   \n\t  "), "");
    }

    #[test] fn returns_owned_string() {
        // The point of returning String (not &str) is independence from
        // the input's lifetime — once you have the result you can drop
        // the original buffer.
        let owned: String = read_input("hi");
        assert_eq!(owned, "hi");
    }
}

fn main() {}
