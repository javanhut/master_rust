// =============================================================================
//  lifetimes_quiz — capstone: a borrow-only Parser
// =============================================================================
//
// Time to put everything together. You'll build a tiny zero-allocation
// parser that walks over a string and hands back SLICES of it. No `String`,
// no `to_string()` — every output reference borrows from the original
// input. That's the whole point.
//
// THE TYPE
//
//     pub struct Parser<'a> {
//         input: &'a str,
//         pos:   usize,
//     }
//
// `Parser<'a>` carries a slice of input plus the current byte cursor.
// Crucially, the lifetime `'a` is tied to whatever string the parser was
// constructed from. A `Parser` cannot outlive its input.
//
// THE API
//
//   pub fn new(input: &'a str) -> Parser<'a>
//       Construct a parser positioned at the start of `input`.
//
//   pub fn peek(&self) -> Option<char>
//       Return the next character WITHOUT consuming it. None at end of
//       input.
//
//   pub fn advance(&mut self) -> Option<char>
//       Return the next character AND move past it. None at end of
//       input. Note: `pos` is a BYTE offset, but `char` is up to 4 bytes
//       in UTF-8; you have to advance by the byte length of the char
//       you just consumed, not by 1. Use `char::len_utf8`.
//
//   pub fn take_while<P: Fn(char) -> bool>(&mut self, pred: P) -> &'a str
//       Consume characters while `pred(c)` returns true; return a SLICE
//       of the original input covering exactly those characters. Note
//       the return type: `&'a str`, NOT `&str`. The slice lives as long
//       as the original input, NOT just as long as `&self`. That's why
//       the lifetime `'a` exists on the struct — to make this signature
//       expressible.
//
// WHY `&'a str` AND NOT `&str` ON `take_while`?
//
// If we returned `&str` (Rule 3 elision: "borrows from self"), the slice
// would borrow from the parser — meaning the caller couldn't use the
// parser again while the slice is alive, and the slice would die when
// the parser dies. Both are wrong: the slice actually points into
// `input`, which lives for `'a`, far longer than a `&self` borrow.
//
// We override the elision by writing `'a` explicitly. Now the result
// borrows from `input` directly, and the parser is free to be advanced
// further (or even dropped) without invalidating it.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Fill in the `???`s. Tests will parse some simple inputs.

// I AM NOT DONE

pub struct Parser??? {
    input: ???,
    pos: usize,
}

impl??? Parser??? {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        // Return the char starting at `self.pos`, or None if at end.
        // Hint: `self.input[self.pos..].chars().next()`.
        ???
    }

    pub fn advance(&mut self) -> Option<char> {
        // Read the next char, then bump `pos` by its UTF-8 byte length.
        let c = self.peek()?;
        self.pos += ???;
        Some(c)
    }

    pub fn take_while<P: Fn(char) -> bool>(&mut self, pred: P) -> &'a str {
        // Remember where we started, advance while the predicate holds,
        // then return the slice [start..pos] of the ORIGINAL input.
        // The returned slice has lifetime 'a — same as the parser's input.
        let start = self.pos;
        while let Some(c) = self.peek() {
            if !pred(c) { break; }
            self.advance();
        }
        ???   // the slice of self.input from `start` to `self.pos`
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn peek_does_not_consume() {
        let p = Parser::new("hi");
        let mut p = p;
        assert_eq!(p.peek(), Some('h'));
        assert_eq!(p.peek(), Some('h'));
        assert_eq!(p.peek(), Some('h'));
    }

    #[test] fn advance_consumes() {
        let mut p = Parser::new("ab");
        assert_eq!(p.advance(), Some('a'));
        assert_eq!(p.advance(), Some('b'));
        assert_eq!(p.advance(), None);
        assert_eq!(p.peek(), None);
    }

    #[test] fn advance_handles_multibyte_chars() {
        // 🦀 is 4 bytes in UTF-8; 'a' is 1.
        let mut p = Parser::new("a🦀b");
        assert_eq!(p.advance(), Some('a'));
        assert_eq!(p.advance(), Some('🦀'));
        assert_eq!(p.advance(), Some('b'));
        assert_eq!(p.advance(), None);
    }

    #[test] fn take_while_basic_word() {
        let mut p = Parser::new("hello, world");
        let word = p.take_while(|c| c.is_ascii_alphabetic());
        assert_eq!(word, "hello");
        assert_eq!(p.peek(), Some(','));
    }

    #[test] fn take_while_returns_slice_outliving_self() {
        // This test is the *whole point* of `'a`. The slice returned by
        // take_while borrows from the original input, NOT from the
        // parser. So we can drop the parser and keep using the slice.
        let input = String::from("abc123 rest");
        let word: &str = {
            let mut p = Parser::new(&input);
            p.take_while(|c| c.is_ascii_alphanumeric())
            // p drops here, but `word` is a slice of `input`, not of p.
        };
        assert_eq!(word, "abc123");
    }

    #[test] fn take_while_can_be_called_repeatedly() {
        let mut p = Parser::new("foo=bar");
        let key = p.take_while(|c| c.is_ascii_alphabetic());
        assert_eq!(key, "foo");
        assert_eq!(p.advance(), Some('='));
        let val = p.take_while(|c| c.is_ascii_alphabetic());
        assert_eq!(val, "bar");
        assert_eq!(p.peek(), None);
    }

    #[test] fn take_while_empty_match_returns_empty_slice() {
        let mut p = Parser::new("123abc");
        let letters = p.take_while(|c| c.is_ascii_alphabetic());
        assert_eq!(letters, "");
        assert_eq!(p.peek(), Some('1'));
    }
}

fn main() {}
