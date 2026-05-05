// =============================================================================
//  own6 — slices: `&str` and `&[T]`
// =============================================================================
//
// A SLICE is a borrowed VIEW into a contiguous run of elements that some
// other value owns. It carries two pieces of data: a pointer + a length.
//
//     &str        — slice of UTF-8 string bytes.        Source: `String` or literal.
//     &[T]        — slice of `T` elements.              Source: `Vec<T>` or `[T; N]`.
//     &mut [T]    — mutable slice (rarer in beginnerland).
//
// MAKING A SLICE
//
//     let s = String::from("hello world");
//     let hello: &str = &s[0..5];     // bytes 0..5
//     let world: &str = &s[6..];      // bytes 6..end
//     let whole: &str = &s[..];       // all of it
//
//     let v = vec![10, 20, 30, 40];
//     let mid: &[i32] = &v[1..3];     // [20, 30]
//
// String literals like `"hello"` are themselves of type `&'static str` —
// they point into the program's read-only data segment.
//
// WHY SLICES MATTER
//
//   - You can write functions that work on a piece of data without caring
//     who owns the whole thing.
//   - The slice's lifetime is tied to the owner — if you tried to keep
//     `hello` after `s` was dropped, the borrow checker would refuse.
//   - `first_word(&s) -> &str` is the canonical example: a function that
//     returns a sub-slice of the input without allocating anything.
//
// THE PARAMETER YOU USUALLY WANT IS &str / &[T], NOT &String / &Vec<T>
//
//   `&String` only accepts a String. `&str` accepts a String AND a literal
//   AND a sub-slice of either. Same logic for `&[T]` vs `&Vec<T>`. ALWAYS
//   prefer the slice form for function parameters.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `first_word(s: &str) -> &str` : return the substring up to the first
//     space, or the entire string if there's no space.
//
//   - `sum_slice(xs: &[i32]) -> i32` : sum a slice of i32 (no Vec, no array
//     literal — accept the borrowed view).
//
//   - `head_tail(xs: &[i32]) -> Option<(&i32, &[i32])>` : if `xs` is empty
//     return None, otherwise return the first element + the rest of the
//     slice.

// I AM NOT DONE

fn first_word(s: &str) -> &str {
    // Walk the bytes; on the first space, return the slice [0..i].
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[???];     // bytes 0 up to (not including) i
        }
    }
    s   // no space found — the whole string is one word
}

fn sum_slice(xs: &[i32]) -> i32 {
    let mut total = 0;
    for &x in xs {
        total ??? x;
    }
    total
}

fn head_tail(xs: &[i32]) -> Option<(&i32, &[i32])> {
    if xs.is_empty() {
        ???
    } else {
        Some((&xs[0], &xs[???]))    // take element 0 and the slice from 1..end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn first_word_basic() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test] fn first_word_literal() {
        // &str accepts a literal directly — that's why it's the right type.
        assert_eq!(first_word("rust is fun"), "rust");
    }

    #[test] fn first_word_no_space() {
        assert_eq!(first_word("oneword"), "oneword");
    }

    #[test] fn sum_array() {
        let arr = [1, 2, 3, 4];
        assert_eq!(sum_slice(&arr), 10);
    }

    #[test] fn sum_vec() {
        let v = vec![10, 20, 30];
        assert_eq!(sum_slice(&v), 60);
    }

    #[test] fn head_tail_some() {
        let v = vec![1, 2, 3];
        let (h, t) = head_tail(&v).unwrap();
        assert_eq!(*h, 1);
        assert_eq!(t, &[2, 3]);
    }

    #[test] fn head_tail_none() {
        let v: Vec<i32> = vec![];
        assert!(head_tail(&v).is_none());
    }
}

fn main() {}
