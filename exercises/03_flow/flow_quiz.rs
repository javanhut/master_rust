// =============================================================================
//  flow_quiz — chapter 3 capstone (FizzBuzz, Rust style)
// =============================================================================
//
// The famous FizzBuzz problem, but returning a Vec<String> so we can test it.
//
// For each integer i in 1..=n, push:
//   "FizzBuzz"  if i is divisible by 15
//   "Fizz"      if i is divisible by 3
//   "Buzz"      if i is divisible by 5
//   otherwise   the number as a string  (use `i.to_string()`)
//
// We haven't covered Vec or String formally yet — but you can absolutely
// use them by following these patterns. (And this is a great hook into
// chapter 4.)
//
//   let mut out: Vec<String> = Vec::new();
//   out.push("hello".to_string());
//   out.push(format!("number {}", 42));
//
// =============================================================================

// I AM NOT DONE

fn fizzbuzz(n: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 1..=n {
        // ORDER MATTERS — check 15 first, then 3, then 5, else fallback.
        let s = if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % ??? == 0 {
            "Fizz".to_string()
        } else if i % ??? == 0 {
            "Buzz".to_string()
        } else {
            ???
        };
        out.push(s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn empty() { assert_eq!(fizzbuzz(0), Vec::<String>::new()); }

    #[test] fn small() {
        let v = fizzbuzz(5);
        assert_eq!(v, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test] fn fifteen() {
        let v = fizzbuzz(15);
        assert_eq!(v.last().unwrap(), "FizzBuzz");
        assert_eq!(v[2], "Fizz");
        assert_eq!(v[4], "Buzz");
        assert_eq!(v.len(), 15);
    }
}

fn main() {}
