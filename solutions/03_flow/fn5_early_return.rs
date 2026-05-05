// SOLUTION — fn5_early_return

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i: u32 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// WHY THIS IS OPTIMAL:
//
//   safe_divide — the `b == 0` guard is a one-line early return. Reading
//   top-to-bottom you encounter the precondition first and only meet the
//   "happy path" once you know it's safe. This is the classic shape of
//   defensive Rust functions.
//
//   is_prime — early returns for `n < 2` and "found a divisor" keep the
//   nesting flat. The trailing `true` is the success expression.
//
//   The `i * i <= n` test is the textbook trick: if n had a divisor
//   greater than √n, it would also have one less than √n, which we'd
//   already have found. So we can stop at √n. We avoid floating-point
//   `(i as f64).sqrt()` by squaring `i` instead.
//
// ALTERNATIVE shapes:
//
//   safe_divide:
//       (b != 0).then(|| a / b)
//
//     Bool::then is "produce Some(x) if true, else None" — the closure
//     runs only on true, so the division is safe. Beautiful one-liner.
//
//   is_prime:
//       n >= 2 && (2..=((n as f64).sqrt() as u32)).all(|i| n % i != 0)
//
//     One expression, but mixes float math; the `i*i` form is preferred.
//
// PERFORMANCE NOTE:
//   For very large `n`, real-world primality testing uses Miller-Rabin or
//   sieves, not trial division. This trial-division version is fine up to
//   about n ≈ 10^9 in interactive testing.
