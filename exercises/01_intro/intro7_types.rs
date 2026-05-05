// =============================================================================
//  intro7 — type annotations and the turbofish
// =============================================================================
//
// Rust's inference is strong but not magical. There are two situations where
// you MUST tell the compiler the type explicitly:
//
//   A) The expression's type genuinely depends on what you're going to do
//      with it later, and you haven't done it yet.
//
//          let n = "42".parse().unwrap();
//          //                ^^^^^^^^
//          // parse can return many types. The compiler doesn't know which
//          // unless you tell it.
//
//   B) You want a non-default numeric type.
//
//          let bytes_received: u64 = 0;
//
// Two ways to annotate:
//
//   On the LEFT (annotation on the binding):
//       let n: i64 = "42".parse().unwrap();
//
//   On the RIGHT (the "turbofish" — `::<T>` after a generic function):
//       let n = "42".parse::<i64>().unwrap();
//
// They are equivalent. Pick whichever reads better at the use site. Most
// real-world Rust code uses the binding annotation; turbofish is common
// when a method call is in the middle of a longer chain.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Annotate `count` correctly so it parses as a u32.
//   - Use a turbofish on `parsed` so it parses as an i64.

// I AM NOT DONE

fn parse_things() -> (u32, i64) {
    let count = "150".parse().unwrap(); // <-- annotate the binding
    let parsed = "-42".parse().unwrap(); // <-- use a turbofish on parse
    (count, parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_are_correct() {
        let (a, b) = parse_things();
        // The very fact that this compiles proves the types are right.
        let _checks_u32: u32 = a;
        let _checks_i64: i64 = b;
        assert_eq!(a, 150);
        assert_eq!(b, -42);
    }
}

fn main() {}
