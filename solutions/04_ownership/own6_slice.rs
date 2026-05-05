// SOLUTION — own6_slice

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn sum_slice(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {
        total += x;
    }
    total
}

fn head_tail(xs: &[i32]) -> Option<(&i32, &[i32])> {
    if xs.is_empty() {
        None
    } else {
        Some((&xs[0], &xs[1..]))
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `first_word` is the textbook example from The Book. We walk the bytes
//   (cheap — no UTF-8 decoding, since ASCII space is a single byte and we
//   only compare against ASCII) and return `&s[0..i]` the moment we see
//   one. Because the return type is `&str`, the result borrows from the
//   input — the compiler verifies that the input outlives the returned
//   slice. No allocation; no copy.
//
//   `sum_slice(xs: &[i32])` accepts:
//       - `&v` where v: Vec<i32>           (deref coerces to slice)
//       - `&arr` where arr: [i32; N]       (array → slice)
//       - any sub-slice `&v[1..3]`
//   That flexibility is why we never write `&Vec<i32>` as a parameter.
//   The `for &x in xs` pattern destructures `&i32` into a fresh `i32`
//   binding — handy when the element type is `Copy`.
//
//   `head_tail` returns BOTH a reference to the first element AND a slice
//   of the rest. Same lifetime — both views borrow from `xs` and are valid
//   only as long as the original is.
//
// IDIOMATIC ALTERNATIVES YOU'LL SEE IN REAL CODE:
//
//   first_word with iterators:
//
//       fn first_word(s: &str) -> &str {
//           s.split_whitespace().next().unwrap_or(s)
//       }
//
//     Slightly different semantics for multiple spaces / tabs, but cleaner.
//
//   sum_slice with iterators:
//
//       fn sum_slice(xs: &[i32]) -> i32 { xs.iter().sum() }
//
//   head_tail with the standard helper:
//
//       fn head_tail(xs: &[i32]) -> Option<(&i32, &[i32])> {
//           xs.split_first()
//       }
//
//     `split_first` literally exists for this. In real code you'd use it.
//     We did the manual version here so you'd see the slicing syntax.
//
// IMPORTANT: WHY THE COMPILER LIKES THIS
//
//   `first_word` returns a slice whose lifetime is tied (by elision) to the
//   input. If a caller does:
//
//       let word;
//       {
//           let s = String::from("hello world");
//           word = first_word(&s);   // borrows s
//       }                            // s dropped here
//       println!("{word}");          // ❌ word would point into freed memory
//
//   …the borrow checker rejects it. You'd see "borrowed value does not live
//   long enough". That static check is what makes slices memory-safe.
