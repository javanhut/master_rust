// =============================================================================
//  own_quiz — chapter 4 capstone
// =============================================================================
//
// Time to combine everything. You'll write three small functions that
// together exercise: borrowing, mutation through `&mut`, slicing, and
// (a tiny bit of) lifetimes.
//
// LIFETIMES — JUST ENOUGH TO PASS THIS QUIZ
//
// You've already been writing functions that return references. The
// compiler usually figures out the relationship between the input and
// output references on its own — that's called LIFETIME ELISION.
// Sometimes it can't, and you have to spell it out. Example:
//
//     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { ... }
//
// Read this as: "for some lifetime `'a`, both inputs and the output share
// it". In plain English: the returned reference is valid for as long as
// BOTH inputs are. The compiler can't elide this because it can't tell
// from the signature alone whether the result borrows from `a`, from `b`,
// or potentially from either — so we tell it: from either, and they have
// to agree.
//
// You'll meet lifetimes properly in chapter 17. For now, just use the
// signature exactly as given. Don't change it.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   1. `longest<'a>(a: &'a str, b: &'a str) -> &'a str`
//        Return whichever of the two strings has more BYTES. Ties → return `a`.
//
//   2. `push_all(dst: &mut String, parts: &[&str])`
//        Append every part in `parts` onto `dst`, in order, with no separator.
//
//   3. `count_word(haystack: &str, needle: &str) -> usize`
//        Return how many times `needle` appears in `haystack`. Use the
//        standard library — `str::matches` returns an iterator over every
//        match.

// I AM NOT DONE

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() ??? b.len() {
        a
    } else {
        b
    }
}

fn push_all(dst: &mut String, parts: &[&str]) {
    for p in parts {
        dst.???(p);   // method that appends a &str onto a String
    }
}

fn count_word(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).???()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn longest_a_wins() {
        assert_eq!(longest("hello", "hi"), "hello");
    }
    #[test] fn longest_b_wins() {
        assert_eq!(longest("a", "abcd"), "abcd");
    }
    #[test] fn longest_tie_returns_a() {
        assert_eq!(longest("rust", "lang"), "rust");
    }

    #[test] fn push_all_basic() {
        let mut s = String::from("> ");
        push_all(&mut s, &["a", "b", "c"]);
        assert_eq!(s, "> abc");
    }
    #[test] fn push_all_empty() {
        let mut s = String::from("keep");
        push_all(&mut s, &[]);
        assert_eq!(s, "keep");
    }

    #[test] fn count_word_zero() {
        assert_eq!(count_word("the cat sat", "dog"), 0);
    }
    #[test] fn count_word_some() {
        assert_eq!(count_word("la la la la", "la"), 4);
    }
    #[test] fn count_word_overlap() {
        // `matches` is non-overlapping. "aaaa" contains "aa" twice (not three).
        assert_eq!(count_word("aaaa", "aa"), 2);
    }
}

fn main() {}
