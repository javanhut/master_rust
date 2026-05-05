// =============================================================================
//  map1 — `HashMap`: create, insert, get, contains_key, remove
// =============================================================================
//
// `HashMap<K, V>` is Rust's general-purpose key→value table, hash-based,
// O(1) average for insert / lookup / remove. It lives in `std::collections`,
// which is NOT in the prelude — you must bring it into scope:
//
//     use std::collections::HashMap;
//
// THE FOUR EVERYDAY METHODS
//
//     let mut m: HashMap<String, i32> = HashMap::new();
//
//     m.insert(key, value);          // returns Option<V>:
//                                    //   None     if the key was new
//                                    //   Some(old) if the key was overwritten
//
//     m.get(&key)                    // -> Option<&V>
//
//     m.contains_key(&key)           // -> bool
//
//     m.remove(&key)                 // -> Option<V>
//                                    //   Some(value) if it existed, else None
//
// `get`, `contains_key`, and `remove` all take a REFERENCE to the key. That's
// because they only need to compare hashes — no need to take ownership.
//
// KEY REQUIREMENTS
//
// The key type `K` must implement `Eq` and `Hash`. All the integer types,
// `bool`, `char`, `String`, and `&str` already do. Your own types need
// `#[derive(Eq, PartialEq, Hash)]` to be usable as keys (a chapter-9 topic).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_scores()`: return a HashMap<String, i32> with these entries:
//                     "alice" -> 90, "bob" -> 75.
//                     Use `.to_string()` on the &str literals so the keys
//                     are owned `String` values.
//   - `score_of(m, name)`: return Option<i32> — the score for `name`,
//                          copied out of the map. (Use `.copied()`.)
//   - `has_player(m, name)`: bool — is `name` in the map?
//   - `kick(m, name)`: remove `name` and return the removed score
//                     as Option<i32>.
//
// Replace every `???`. Do NOT touch the tests.

use std::collections::HashMap;

// I AM NOT DONE

fn make_scores() -> HashMap<String, i32> {
    let mut m: HashMap<String, i32> = HashMap::???();
    m.???("alice".to_string(), 90);
    m.insert("bob".to_string(), ???);
    m
}

fn score_of(m: &HashMap<String, i32>, name: &str) -> Option<i32> {
    m.???(name).copied()
}

fn has_player(m: &HashMap<String, i32>, name: &str) -> bool {
    m.???(name)
}

fn kick(m: &mut HashMap<String, i32>, name: &str) -> Option<i32> {
    m.???(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn scores_built() {
        let m = make_scores();
        assert_eq!(m.len(), 2);
        assert_eq!(m.get("alice"), Some(&90));
        assert_eq!(m.get("bob"),   Some(&75));
    }
    #[test] fn score_lookup() {
        let m = make_scores();
        assert_eq!(score_of(&m, "alice"), Some(90));
        assert_eq!(score_of(&m, "ghost"), None);
    }
    #[test] fn contains_works() {
        let m = make_scores();
        assert!(has_player(&m, "bob"));
        assert!(!has_player(&m, "carol"));
    }
    #[test] fn kick_existing() {
        let mut m = make_scores();
        assert_eq!(kick(&mut m, "alice"), Some(90));
        assert_eq!(m.len(), 1);
        assert!(!has_player(&m, "alice"));
    }
    #[test] fn kick_missing() {
        let mut m = make_scores();
        assert_eq!(kick(&mut m, "ghost"), None);
        assert_eq!(m.len(), 2);
    }
}

fn main() {}
