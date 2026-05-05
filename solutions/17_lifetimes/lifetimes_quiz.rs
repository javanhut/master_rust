// SOLUTION — lifetimes_quiz

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    pub fn take_while<P: Fn(char) -> bool>(&mut self, pred: P) -> &'a str {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if !pred(c) { break; }
            self.advance();
        }
        &self.input[start..self.pos]
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The whole design centers on ONE choice: `take_while` returns
//   `&'a str`, not `&str`. Let's read it carefully.
//
//   The default elision (Rule 3) on
//
//       pub fn take_while<P: Fn(char) -> bool>(&mut self, pred: P) -> &str
//
//   would tie the output to `&self`'s lifetime. That's BAD here:
//
//     - The caller would be unable to call any other `&mut self` method
//       (or even `&self` method) while the slice is alive. The slice
//       would hold an exclusive borrow of the parser.
//     - The slice would die when the parser does — but conceptually
//       the slice points into `input`, which lives much longer (`'a`).
//
//   Writing `-> &'a str` overrides Rule 3 and says "the result borrows
//   from `input`, lifetime `'a`, NOT from `self`." The compiler can
//   prove this because:
//
//     - `start` and `self.pos` are valid indices into `self.input`.
//     - `self.input` has type `&'a str`, so any sub-slice
//       `&self.input[start..self.pos]` also has type `&'a str`.
//
//   Once the slice is constructed, it carries lifetime `'a`, untied
//   from the `&mut self` borrow that was used to call `take_while`.
//   The borrow of `self` lasts only for the duration of the call; the
//   returned slice outlives the call. That's why the
//   `take_while_returns_slice_outliving_self` test passes — we drop
//   the parser entirely and the slice survives, because the slice was
//   never borrowing from the parser in the first place.
//
//   `peek` is straightforward: slice from `pos` to the end, then take
//   the first `char` of that. No allocation, O(1) (well, O(1) since
//   `chars().next()` only decodes the first scalar value).
//
//   `advance` is the UTF-8 trap: `self.pos += 1` would BREAK on any
//   multi-byte character (it would land in the middle of a code point
//   and the next slice operation would panic). `c.len_utf8()` returns
//   the number of bytes the just-decoded char occupied — exactly what
//   we need to advance by.
//
// LIFETIMES PUT TOGETHER — A TOUR OF WHAT YOU LEARNED:
//
//   - lt1: lifetimes are SCOPES, not durations. The compiler enforces
//     "every reference is used inside the region its referent exists."
//
//   - lt2: explicit `<'a>` on functions when elision can't pick.
//     Annotations don't affect runtime; they only let the compiler
//     check constraints.
//
//   - lt3: three elision rules — one input ref, single-`&self`,
//     etc. Most code never names a lifetime because of these.
//
//   - lt4: structs with reference fields need `<'a>`. Such structs
//     can't outlive the data they borrow.
//
//   - lt5: `'static` is "lives forever." Bound `T: 'static` means
//     "contains no short-lived borrows." Owned types pass trivially.
//
//   - lt6: `&'long T` works where `&'short T` is needed (covariance).
//     The reverse fails.
//
//   - this quiz: `&'a str` on a method overrides Rule 3 to point the
//     caller at the LONGER underlying borrow rather than `&self`.
//
// ALTERNATIVES YOU'LL SEE IN REAL PARSERS:
//
//   1. Owned outputs — `take_while -> String`. One allocation per call,
//      simpler types, no lifetime in the API. Use this when callers
//      want to mutate the result, or when the parser deallocates the
//      input before the caller is done.
//
//   2. Iterator-based — `take_while -> impl Iterator<Item=char> + 'a`.
//      Lazier; pairs nicely with `collect()`. Slightly more complex
//      because borrowing the parser through an iterator means the
//      parser can't be touched until the iterator drops.
//
//   3. `nom`-style combinator parsers — return a `(remaining_input,
//      matched)` tuple, both `&'a str`. Pure functional style; no
//      mutable parser state. Equivalent expressive power.
//
// CHAPTER 17 CHEAT SHEET:
//
//     'a            named lifetime parameter (region of code)
//     'static       lives for the entire program
//     T: 'static    T contains no borrows shorter than 'static
//     &'a T         reference valid for the region 'a
//     'long: 'short 'long outlives 'short — a longer ref where shorter
//                   is asked is always fine
//     <'_>          "infer this lifetime"
//     elision:      1) each input ref → own 'a
//                   2) one input ref → output gets it
//                   3) &self present → output gets self's 'a
