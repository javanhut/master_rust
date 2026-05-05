// =============================================================================
//  lt4 — structs that hold references
// =============================================================================
//
// Up to now you've stored owned values in structs: `String`, `Vec<T>`, plain
// `i32`. A struct can ALSO hold a reference, but the moment it does, the
// struct is tied to the lifetime of whatever it borrows from.
//
//     struct Excerpt<'a> {
//         part: &'a str,
//     }
//
// Read this as: "an `Excerpt` is parameterized by a lifetime `'a`, and the
// `part` field is a reference valid for at least `'a`."
//
// THE GOLDEN RULE
//
//   ▶ A STRUCT WITH BORROWED FIELDS CANNOT OUTLIVE THE DATA IT BORROWS.
//
// If `Excerpt` borrows from a `String`, the `String` must live at least as
// long as the `Excerpt`. The compiler enforces this by giving every
// `Excerpt` instance a lifetime no longer than the lifetime of its
// underlying `&'a str`.
//
// HOW TO READ `<'a>` ON A STRUCT
//
//   `struct Excerpt<'a>` — a generic parameter that's a lifetime, not a
//   type. When you USE the struct elsewhere, you write `Excerpt<'_>`
//   (anonymous lifetime, "infer it") or the explicit `Excerpt<'a>` if
//   the surrounding code names a region.
//
//   In `impl` blocks you must repeat the parameter:
//
//       impl<'a> Excerpt<'a> {
//           fn part(&self) -> &str { self.part }   // Rule 3 elides the rest
//       }
//
// WHEN YOU NEED THIS PATTERN
//
//   - You want a "view" type that refers into existing data without
//     copying — e.g., a parser holding a slice of input.
//   - You want zero-allocation parsing.
//   - You're modeling a sub-region of a larger data structure.
//
// WHEN YOU SHOULDN'T
//
//   - If the struct will be stored long-term, returned from many places,
//     or sent across threads, owned data (`String`, `Vec<T>`) is far less
//     painful. The convenience cost of owned data is usually worth it.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Implement a tiny `Excerpt` type that holds a slice of an owning
// `String`. You'll fill in:
//
//   1. The struct definition `struct Excerpt<'a> { ... }`.
//   2. A constructor `Excerpt::first_sentence(s: &str) -> Excerpt<'_>`
//      that returns the substring up to (but not including) the first
//      '.' in `s`. If there's no '.', return the whole string.
//   3. A getter `fn part(&self) -> &str` — Rule 3 elides everything.
//   4. A `len(&self) -> usize` for the byte length of the part.
//
// We've left blanks where the lifetime annotations belong.

// I AM NOT DONE

struct Excerpt??? {
    part: ???,
}

impl??? Excerpt??? {
    fn first_sentence(s: &str) -> Excerpt<'_> {
        // Find the byte index of the first '.', and slice up to it.
        let end = s.find('.').unwrap_or(s.len());
        Excerpt { part: ??? }
    }

    fn part(&self) -> &str {
        ???
    }

    fn len(&self) -> usize {
        ???
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn excerpt_basic_sentence() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let ex = Excerpt::first_sentence(&novel);
        assert_eq!(ex.part(), "Call me Ishmael");
        assert_eq!(ex.len(), 15);
    }

    #[test] fn excerpt_no_period_keeps_whole_string() {
        let s = String::from("just one line");
        let ex = Excerpt::first_sentence(&s);
        assert_eq!(ex.part(), "just one line");
    }

    #[test] fn excerpt_empty_input() {
        let s = String::from("");
        let ex = Excerpt::first_sentence(&s);
        assert_eq!(ex.part(), "");
        assert_eq!(ex.len(), 0);
    }

    #[test] fn excerpt_borrows_not_copies() {
        // If `Excerpt` were doing a String allocation, this would still
        // work — but the *point* of the type is that `part` is a slice
        // pointing into `novel`. We can't observe that directly without
        // unsafe, but we can confirm the API contract: the returned
        // string is exactly the prefix of the input, byte-for-byte.
        let novel = String::from("a.b.c");
        let ex = Excerpt::first_sentence(&novel);
        assert_eq!(ex.part().as_bytes(), &novel.as_bytes()[..1]);
    }
}

fn main() {}
