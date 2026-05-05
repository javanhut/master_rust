// SOLUTION — fn4_loops

fn count_evens(n: u32) -> u32 {
    let mut count = 0;
    for i in 0..=n {
        if i % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn first_above(threshold: i32, xs: &[i32]) -> Option<i32> {
    let mut idx = 0;
    loop {
        if idx >= xs.len() {
            break None;
        }
        if xs[idx] > threshold {
            break Some(xs[idx]);
        }
        idx += 1;
    }
}

// WHY THIS IS OPTIMAL FOR THE LESSON:
//
//   count_evens uses a `for` over a RangeInclusive — the natural shape for
//   "every integer from a to b inclusive".
//
//   first_above demonstrates the unique super-power of `loop`: `break
//   value;` makes the whole loop expression equal to that value. We
//   return `loop { ... }` directly as the function's trailing expression
//   — no `result` binding needed.
//
// IDIOMATIC NEXT-LEVEL VERSIONS:
//
//   fn count_evens(n: u32) -> u32 {
//       (0..=n).filter(|i| i % 2 == 0).count() as u32
//   }
//
//   fn first_above(threshold: i32, xs: &[i32]) -> Option<i32> {
//       xs.iter().copied().find(|&x| x > threshold)
//   }
//
//   Each is a single line. Iterators are how production Rust expresses
//   this kind of logic; the loop versions are equivalent and useful when
//   you need control flow that doesn't fit a chain (early exits with side
//   effects, complex multi-iterator state).
//
// MATHEMATICAL SHORTCUT for count_evens:
//   The number of evens in 0..=n is `n / 2 + 1`. Constant time.
