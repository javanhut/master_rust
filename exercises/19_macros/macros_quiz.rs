// =============================================================================
//  macros_quiz — capstone: a `hashmap!` literal macro
// =============================================================================
//
// Time to combine everything from this chapter. The standard library
// gives you `vec![1, 2, 3]` for Vec literals but does NOT ship a
// `hashmap!` macro. Many projects roll their own (or pull in the
// `maplit` crate). It's a perfect exercise for `macro_rules!`.
//
// USAGE WE WANT
// ─────────────
//
//     let m = hashmap!{ "a" => 1, "b" => 2, "c" => 3 };
//     assert_eq!(m.get("a"), Some(&1));
//
// `hashmap!{ ... }` should evaluate to a `std::collections::HashMap<K, V>`
// where K and V are inferred from the key/value expressions. The `=>`
// separator is a stylistic nod to `match` arms and to `maplit::hashmap!`.
//
// EXPANSION SHAPE
// ───────────────
//
// We want each `K => V` pair to become an `m.insert(K, V);` call.
// Standard recipe:
//
//     {
//         let mut m = ::std::collections::HashMap::new();
//         m.insert(k1, v1);
//         m.insert(k2, v2);
//         ...
//         m
//     }
//
// The inner block is what gives the macro a value.
//
// PATTERN
// ───────
//
//     ( $( $k:expr => $v:expr ),* $(,)? )
//
//   - `$( ... ),*`  — zero-or-more comma-separated repetitions.
//   - `$k:expr => $v:expr` — capture key and value, with `=>` between.
//   - `$(,)?`       — optional trailing comma (so `{ a => 1, }` works).
//
// EXPANSION
// ─────────
//
//     {{
//         let mut m = ::std::collections::HashMap::new();
//         $( m.insert($k, $v); )*
//         m
//     }}
//
// The double braces are again "macro_rules! delimiters" + "Rust block
// expression."
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Implement `hashmap!` per the recipe above.
//   - The empty form `hashmap!{}` should produce an empty `HashMap`.
//   - Trailing commas should be allowed.
//
// Use the `::std::collections::HashMap` path so the macro works in any
// module without requiring callers to `use HashMap;` first. (Library-quality
// macros always reach for absolute paths.)
//
// Replace each `???`. Don't change the tests.

// I AM NOT DONE

macro_rules! hashmap {
    ( $( $k:??? => $v:??? ),* $(,)? ) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.???($k, $v); )*
        m
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let m: HashMap<&str, i32> = hashmap! {};
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn three_entries() {
        let m = hashmap! {
            "a" => 1,
            "b" => 2,
            "c" => 3,
        };
        assert_eq!(m.len(), 3);
        assert_eq!(m.get("a"), Some(&1));
        assert_eq!(m.get("b"), Some(&2));
        assert_eq!(m.get("c"), Some(&3));
    }

    #[test]
    fn no_trailing_comma_also_works() {
        let m = hashmap! {
            "x" => 10,
            "y" => 20
        };
        assert_eq!(m.get("x"), Some(&10));
        assert_eq!(m.get("y"), Some(&20));
    }

    #[test]
    fn duplicate_keys_last_wins() {
        // HashMap::insert overwrites, so the last `=>` for a key wins.
        let m = hashmap! {
            "k" => 1,
            "k" => 2,
            "k" => 3,
        };
        assert_eq!(m.len(), 1);
        assert_eq!(m.get("k"), Some(&3));
    }

    #[test]
    fn key_and_value_are_expressions() {
        let key = String::from("hello");
        let n = 5;
        let m = hashmap! {
            key.clone() => n * 2,
            String::from("world") => n + 1,
        };
        assert_eq!(m.get(&key), Some(&10));
        assert_eq!(m.get("world"), Some(&6));
    }
}

fn main() {}
