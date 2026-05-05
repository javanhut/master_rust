// =============================================================================
//  fn5 — early returns and idiomatic flow
// =============================================================================
//
// `return EXPR;` exits the current function immediately with that value.
// You can use it anywhere, but Rust style usually omits it at the end of a
// function — preferring the trailing-expression form:
//
//     fn double(n: i32) -> i32 {
//         return n * 2;       // works, but unnecessary
//     }
//     fn double(n: i32) -> i32 { n * 2 }  // idiomatic
//
// EARLY RETURNS for guard conditions are GOOD STYLE — they keep the happy
// path indented at column 0:
//
//     fn discount(price: f64, member: bool) -> f64 {
//         if !member { return price; }   // guard
//         price * 0.9                    // happy path
//     }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement `safe_divide(a, b)`:
//   - If b == 0, return None EARLY (use `return`).
//   - Otherwise return Some(a / b) as the trailing expression.
//
// Then implement `is_prime(n)`:
//   - Return false early for n < 2.
//   - Otherwise check divisors 2..=sqrt(n). If any divides, return false.
//   - End with the trailing expression `true`.

// I AM NOT DONE

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return ???;
    }
    Some(a / b)
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return ???;
    }
    let mut i: u32 = 2;
    while i * i <= n {
        if n % i == 0 {
            return ???;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn divide_normal()    { assert_eq!(safe_divide(10, 2), Some(5)); }
    #[test] fn divide_by_zero()   { assert_eq!(safe_divide(10, 0), None); }

    #[test] fn primes_small() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!( is_prime(2));
        assert!( is_prime(3));
        assert!(!is_prime(4));
        assert!( is_prime(5));
        assert!(!is_prime(9));
        assert!( is_prime(13));
    }
    #[test] fn primes_larger() {
        assert!( is_prime(97));
        assert!(!is_prime(100));
        assert!( is_prime(7919));
    }
}

fn main() {}
