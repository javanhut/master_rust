// =============================================================================
//  closure5 — closures + iterators: the bread-and-butter combo
// =============================================================================
//
// The single biggest reason to like closures in Rust is iterator chains.
// Almost every interesting `Iterator` method takes a closure:
//
//     .map(|x| ...)        — transform each element
//     .filter(|x| ...)     — keep elements where the closure returns true
//     .fold(init, |acc, x| ...) — collapse the iterator into a single value
//     .for_each(|x| ...)   — run a side-effect on each element
//     .any(|x| ...) / .all(|x| ...)
//     .find(|x| ...)
//     .take_while(|x| ...) / .skip_while(|x| ...)
//
// Iterators were chapter 7 — here we focus on the closures inside them.
//
// THREE WORKHORSES:
//
//   map — produces a new iterator where each element has been transformed:
//
//       let doubled: Vec<i32> = (1..=4).map(|x| x * 2).collect();
//       // [2, 4, 6, 8]
//
//   filter — produces a new iterator that yields only elements for which
//   the closure returns true. Note: filter's closure receives a REFERENCE
//   (`&T`), not the element by value. That's why you'll see `|&x|` to
//   pattern-match through the reference, or `|x| **x > 0` to deref it:
//
//       let evens: Vec<i32> = (1..=10).filter(|&x| x % 2 == 0).collect();
//       // [2, 4, 6, 8, 10]
//
//   fold — the swiss-army knife. Takes an initial value and a closure
//   `(acc, item) -> acc`. Reduces the whole iterator down to one value:
//
//       let sum = (1..=100).fold(0, |acc, x| acc + x);    // 5050
//
// CLOSURES SHINE HERE because they capture local context cheaply:
//
//     let threshold = 10;
//     let big_count = numbers.iter().filter(|&&x| x > threshold).count();
//
// With named functions you'd have to thread `threshold` through manually.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - `squares_of_evens(nums)` — given a slice of i32, return a Vec<i32>
//     containing the SQUARE of every EVEN number, in order.
//     Use `.filter(...)` then `.map(...)` then `.collect()`.
//
//   - `sum_of_positive(nums)` — sum every positive element using `.fold`.
//
//   - `longest_word(text)` — given a sentence, return the longest word as
//     a &str borrowed from `text`. Use `.split_whitespace()` and
//     `.max_by_key(|w| w.len())`. If the text is empty, return "".

// I AM NOT DONE

fn squares_of_evens(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| ???)
        .map(|&n| ???)
        .collect()
}

fn sum_of_positive(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &n| ???)
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|w| ???)
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn squares_basic() {
        assert_eq!(squares_of_evens(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
    }

    #[test] fn squares_all_odd() {
        assert_eq!(squares_of_evens(&[1, 3, 5]), Vec::<i32>::new());
    }

    #[test] fn squares_empty() {
        assert_eq!(squares_of_evens(&[]), Vec::<i32>::new());
    }

    #[test] fn sum_positive_basic() {
        assert_eq!(sum_of_positive(&[1, -2, 3, -4, 5]), 9);
    }

    #[test] fn sum_positive_all_negative() {
        assert_eq!(sum_of_positive(&[-1, -2, -3]), 0);
    }

    #[test] fn sum_positive_empty() {
        assert_eq!(sum_of_positive(&[]), 0);
    }

    #[test] fn longest_simple() {
        assert_eq!(longest_word("the quick brownish fox"), "brownish");
    }

    #[test] fn longest_one_word() {
        assert_eq!(longest_word("hello"), "hello");
    }

    #[test] fn longest_empty() {
        assert_eq!(longest_word(""), "");
        assert_eq!(longest_word("   "), "");
    }
}

fn main() {}
