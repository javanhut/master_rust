// =============================================================================
//  str3 — concatenation: `+`, `format!`, `push_str`, `push`
// =============================================================================
//
// Rust gives you four common ways to glue strings together. They are NOT
// interchangeable — each makes a different tradeoff between ergonomics,
// allocations, and ownership.
//
// 1. THE `+` OPERATOR  — has a SURPRISING signature:
//
//        impl Add<&str> for String { type Output = String; ... }
//
//    In English: `+` consumes the LEFT String (takes it by value) and borrows
//    the RIGHT side as `&str`. Returns a new String — but in practice it
//    reuses the left String's buffer, so it's efficient.
//
//        let a = String::from("Hello, ");
//        let b = String::from("world");
//        let c = a + &b;            // a is MOVED, b is borrowed (&b -> &str)
//        // a no longer usable here
//
//    For more than two pieces, `+` chains awkwardly:
//
//        let s = a + &b + &c + &d;  // legal, but...
//
// 2. `format!` MACRO — like `println!` but returns a `String` instead of
//    printing. Doesn't consume any of its arguments. Best for readability
//    when you have multiple pieces or interpolation:
//
//        let s = format!("{name} is {age}");
//        let s = format!("{}-{}-{}", a, b, c);
//
//    Slight overhead vs. `+` (it goes through the formatter), but for non-
//    hot-loop code the readability wins almost every time.
//
// 3. `push_str(&str)` — APPENDS a string slice in place. Most efficient when
//    you already have a mutable String you're building up:
//
//        let mut buf = String::new();
//        buf.push_str("hello");
//        buf.push_str(", world");
//
// 4. `push(char)` — APPENDS a single Unicode scalar. Note: a `char` is 4
//    bytes in Rust. push will encode it as 1–4 UTF-8 bytes into the String.
//
//        buf.push('!');
//        buf.push('🦀');
//
// RULE OF THUMB
//
//     Two pieces, throwaway result:        a + &b
//     Interpolating into a template:       format!(...)
//     Building inside a loop:               push_str / push  (no realloc tax)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `greet_plus`:   build "Hello, <name>!" using the `+` operator.
//   - `greet_format`: build "Hello, <name>!" using `format!`.
//   - `build_csv`:    given a slice of &str, build a comma-separated String
//                     using a loop with `push_str` and `push(char)`.
//                     ("a","b","c") -> "a,b,c"   no trailing comma.

// I AM NOT DONE

fn greet_plus(name: &str) -> String {
    let hello = String::from("Hello, ");
    hello ??? name + "!"
}

fn greet_format(name: &str) -> String {
    ???!("Hello, {name}!")
}

fn build_csv(items: &[&str]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.???(',');         // single char
        }
        out.???(item);            // string slice
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn plus_basic()    { assert_eq!(greet_plus("Ada"),    "Hello, Ada!"); }
    #[test] fn format_basic()  { assert_eq!(greet_format("Ada"),  "Hello, Ada!"); }
    #[test] fn plus_and_format_match() {
        assert_eq!(greet_plus("Grace"), greet_format("Grace"));
    }

    #[test] fn csv_three()     { assert_eq!(build_csv(&["a","b","c"]), "a,b,c"); }
    #[test] fn csv_one()       { assert_eq!(build_csv(&["solo"]),      "solo"); }
    #[test] fn csv_empty()     { assert_eq!(build_csv(&[]),            ""); }
}

fn main() {}
