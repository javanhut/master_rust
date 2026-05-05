// SOLUTION — str3_concat

fn greet_plus(name: &str) -> String {
    let hello = String::from("Hello, ");
    hello + name + "!"
}

fn greet_format(name: &str) -> String {
    format!("Hello, {name}!")
}

fn build_csv(items: &[&str]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(item);
    }
    out
}

// WHY EACH CHOICE:
//
//   greet_plus:
//       `hello + name + "!"` — `hello` (a String) is consumed by `+`, then
//       the right-hand side `name` is &str so coerces directly. The chain
//       reuses the same buffer (after one possible grow), so this is cheap.
//       The owned-on-left, borrowed-on-right pattern is non-negotiable: you
//       cannot write `&str + &str` or `String + String`.
//
//   greet_format:
//       `format!` with a captured identifier `{name}` is the modern (1.58+)
//       idiom — clearer than positional `{}, {}`. It is the go-to for any
//       interpolation involving more than two pieces or non-string values.
//
//   build_csv:
//       The "join with separator" loop. push_str takes &str, push takes
//       char. For a single ASCII byte ',' either would work, but push(char)
//       documents intent ("one character, not a slice").
//
// THE IDIOMATIC ALTERNATIVE (a teaser for str5):
//
//     fn build_csv(items: &[&str]) -> String {
//         items.join(",")
//     }
//
//   `.join` is exactly the loop above, written for you. We do it the long
//   way here so you build muscle memory for push_str / push.
//
// PERFORMANCE NOTE:
//   For long lists, `String::with_capacity` + push_str avoids a few realloc
//   doublings. format! with many args is slower than push_str-in-a-loop,
//   but in a non-hot path the readability wins.
