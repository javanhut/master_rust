// SOLUTION — proj3_types

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CountUnit {
    Lines,
    Words,
    Chars,
}

impl Stats {
    pub fn new() -> Self {
        Stats { lines: 0, words: 0, chars: 0 }
    }

    pub fn add(&mut self, kind: CountUnit, n: usize) {
        match kind {
            CountUnit::Lines => self.lines += n,
            CountUnit::Words => self.words += n,
            CountUnit::Chars => self.chars += n,
        }
    }
}

// WHY THIS IS OPTIMAL:
//
//   `Stats` is plain old data — three `usize` fields. We derive:
//
//     Debug         — for assert_eq! failure messages.
//     PartialEq/Eq  — for assert_eq! itself.
//     Clone, Copy   — Stats is small (24 bytes on 64-bit). Make it Copy
//                     so it passes around by value, no heap, no move
//                     hassles. The merge step in proj6 will appreciate it.
//
//   `CountUnit` is a "C-style enum" — variants carry no data. It compiles
//   down to a single `u8`-sized tag. Pattern-matching on it in `add` is
//   exhaustive: drop a variant later and the compiler will point at every
//   match site that needs to grow. That's the point of enums.
//
// ALTERNATIVES:
//
//   1. `#[derive(Default)]` on Stats — then `Stats::default()` does the
//      same job as `new()`. We wrote `new()` explicitly so beginners can
//      see the constructor pattern. Both are idiomatic.
//
//   2. Replace the enum with a method per category:
//
//          pub fn add_lines(&mut self, n: usize) { self.lines += n; }
//          pub fn add_words(&mut self, n: usize) { self.words += n; }
//          pub fn add_chars(&mut self, n: usize) { self.chars += n; }
//
//      Cleaner here — but then proj7's CLI flag would still need an
//      enum-shaped value to dispatch on. Keeping the enum pays back later.
//
//   3. Use `u64` instead of `usize`. `usize` is already pointer-sized, so
//      on 64-bit systems they coincide. `usize` reads as "a count or
//      length", which is what these fields are.
//
// KEY TAKEAWAYS:
//
//   - Domain types make the rest of the program legible. `count_basic`
//     returning `Stats` is much clearer than `(usize, usize, usize)`.
//   - Derive what you can derive. The compiler-generated impls are
//     correct, fast, and free.
//   - Tag-only enums are zero-cost dispatch. Use them whenever the set of
//     options is closed and known at compile time.
