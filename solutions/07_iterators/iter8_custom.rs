// SOLUTION — iter8_custom

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        let next_b = self.a + self.b;
        self.a = self.b;
        self.b = next_b;
        Some(current)
    }
}


// WHY THIS IS OPTIMAL:
//
//   The struct stores exactly the state needed to step the recurrence:
//   `a` is the value about to be emitted, `b` is the one after. Each
//   `next` call:
//     1. saves `a` as the answer to return,
//     2. advances state with `(a, b) -> (b, a + b)`,
//     3. returns `Some(saved)`.
//
//   Because we never return `None`, this is an INFINITE iterator. That's
//   not a bug — it's the whole point of lazy iterators. Callers control
//   length with `.take(n)`, `.take_while(...)`, or `.find(...)`.
//
//   Computing `next_b = self.a + self.b` BEFORE the assignments matters:
//   once we write `self.a = self.b`, the old `self.a` is gone. Stash the
//   sum first, then update both fields. (You could also use
//   `std::mem::replace` or destructure into a tuple — pick what reads.)
//
// WHAT YOU GET FOR FREE:
//
//   Implementing `next` unlocks the ENTIRE Iterator trait — every method
//   we've seen in this chapter and dozens more. The tests show four
//   without writing any extra code:
//
//       .take(n)          .sum()          .take_while(|...|)
//       .filter(|...|)    .nth(i)         .collect()
//
//   This is the trait-system payoff: ONE method to implement, hundreds of
//   methods provided. It's also why "implement IntoIterator on my type"
//   is sometimes worth it on its own — anything generic over iterators
//   suddenly works with your collection.
//
// ALTERNATIVES:
//
//   You can write the same thing functionally with `std::iter::successors`:
//       std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))
//           .map(|(a, _)| a)
//   …which yields the same sequence with no struct. For a single throw-
//   away pipeline, that's nicer. For a reusable named iterator, the
//   `impl Iterator` form documents intent better and is what the standard
//   library does for things like `Chars`, `Lines`, `Range`, etc.
//
//   To make the iterator FUSED (return `None` for ever after the first),
//   either implement `FusedIterator` as a marker, or just trust the
//   convention. For a never-Nones-anyway sequence like Fibonacci it's
//   moot.
