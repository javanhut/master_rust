// =============================================================================
//  iter8 — implementing Iterator yourself
// =============================================================================
//
// Everything in this chapter — `map`, `filter`, `sum`, `collect`, `take`,
// `zip`, the lot — comes from one trait method:
//
//     trait Iterator {
//         type Item;
//         fn next(&mut self) -> Option<Self::Item>;
//     }
//
// Once you implement `next` for your own type, the entire combinator
// ecosystem is YOURS. Free `take`, free `map`, free `collect`, free `for`.
//
// THE TWO PIECES
//
//     struct Counter { n: u32 }
//
//     impl Iterator for Counter {
//         type Item = u32;
//         fn next(&mut self) -> Option<u32> {
//             self.n += 1;
//             if self.n <= 5 { Some(self.n) } else { None }
//         }
//     }
//
//     // free combinators!
//     let v: Vec<u32> = Counter { n: 0 }.map(|x| x * 10).collect();
//     // [10, 20, 30, 40, 50]
//
// REMEMBER
//
//   - `type Item = ...;` is an ASSOCIATED TYPE — it tells the trait what
//     `next` returns inside the `Option`.
//   - `next` takes `&mut self` because it has to advance internal state.
//   - Returning `None` ONCE means "done forever". Most well-behaved
//     iterators keep returning `None` after the first `None` (a property
//     called "fused"). For exercise iterators, we just won't call them
//     again after `None`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build a `Fibonacci` struct that yields the Fibonacci sequence
// 0, 1, 1, 2, 3, 5, 8, 13, ... as `u64`. We let it run forever (no upper
// bound); callers limit it with `.take(n)`.
//
// The struct holds two `u64`s — the next value to return and the one after
// it. `Fibonacci::new()` starts the sequence at 0, 1. Each `next()` returns
// the current `a`, then advances: `(a, b) -> (b, a + b)`.
//
// You'll see how naturally `.take(n)`, `.map`, `.sum`, etc. compose with
// your custom iterator — the tests show off three or four for free.

// I AM NOT DONE

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: ???, b: ??? }                 // start: 0, 1
    }
}

impl Iterator for Fibonacci {
    type Item = ???;                                 // u64

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        // Advance the state. After this, self.a holds the NEXT value to
        // emit, and self.b holds the value after THAT.
        let next_b = self.a + self.b;
        self.a = ???;                                // becomes the old b
        self.b = ???;                                // becomes a + b
        Some(current)                                // infinite stream — never None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_ten() {
        let v: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test] fn sum_first_ten() {
        // free `.sum()` on a custom iterator!
        let s: u64 = Fibonacci::new().take(10).sum();
        assert_eq!(s, 88);
    }

    #[test] fn evens_under_100() {
        let v: Vec<u64> = Fibonacci::new()
            .take_while(|&x| x < 100)
            .filter(|&x| x % 2 == 0)
            .collect();
        assert_eq!(v, vec![0, 2, 8, 34]);
    }

    #[test] fn nth_works() {
        // Iterator::nth is also free — index 7 of fib is 13.
        assert_eq!(Fibonacci::new().nth(7), Some(13));
    }
}

fn main() {}
