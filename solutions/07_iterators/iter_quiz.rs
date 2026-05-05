// SOLUTION — iter_quiz

fn count_evens(n: u32) -> u32 {
    (0..=n).filter(|x| x % 2 == 0).count() as u32
}

fn first_above(threshold: i32, xs: &[i32]) -> Option<i32> {
    xs.iter().copied().find(|&x| x > threshold)
}

fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        })
        .collect()
}

fn sum_of_squares_of_evens(xs: &[i32]) -> i32 {
    xs.iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
}


// WHY THIS IS OPTIMAL:
//
//   count_evens — `(0..=n)` IS an iterator. `.filter(|x| x % 2 == 0)` keeps
//   only the evens. `.count()` returns the number of survivors as `usize`,
//   so we cast to `u32` to match the signature. Compared to the chapter-3
//   version (a mutable counter, a for-loop, a body):
//
//       fn count_evens(n: u32) -> u32 {
//           let mut c = 0;
//           for i in 0..=n { if i % 2 == 0 { c += 1; } }
//           c
//       }
//
//   The new version reads almost exactly like the English description.
//   Same machine code at the end of the day; less for the human to track.
//
//   first_above — `.find(|&x| x > threshold)` short-circuits at the first
//   match and returns `Option<i32>` (after `.copied()` strips the
//   reference). This replaces a manual `loop { match xs.get(i) { ... }
//   break Some(...); ... }` with one line.
//
//   fizzbuzz — `match (i % 3, i % 5)` is the cleanest fizzbuzz Rust has.
//   Each arm produces an owned `String`. `.collect::<Vec<String>>()`
//   builds the result. Note the symmetry: every arm produces the same
//   type, so `match` is happy as a single expression inside `.map`.
//
//   sum_of_squares_of_evens — the canonical "filter, map, terminator"
//   trio. Filter BEFORE map: we only square survivors. `.sum()` requires
//   the result type to be inferable; the function's return type fixes
//   it as `i32`.
//
// THE BIG IDEA:
//
//   These functions are MUCH shorter because the iterator chain
//   describes WHAT we want, not HOW the loop walks. Index variables,
//   accumulator initialisations, and break conditions vanish — they
//   were boilerplate that the standard library can express in two or
//   three method calls. After this chapter, "wait, can I write this as
//   a chain?" should be your default question whenever you reach for
//   `for { let mut acc; ... }`.
//
// ALTERNATIVES:
//
//   `count_evens` without the cast: change the return type to `usize`,
//   or count manually with `.fold(0u32, |c, _| c + 1)`. The cast is
//   cleanest given the existing signature.
//
//   `first_above` with `.position` instead of `.find` would give the
//   INDEX, not the value. Different question, different tool.
//
//   `sum_of_squares_of_evens` written as a fold:
//       xs.iter().fold(0, |acc, &x| if x % 2 == 0 { acc + x*x } else { acc })
//   Same answer. The filter+map+sum trio is more readable for most
//   eyes; reach for fold when the logic doesn't decompose cleanly.
