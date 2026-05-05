// =============================================================================
//  own3 — ownership and functions
// =============================================================================
//
// Functions and ownership interact in three ways:
//
//   1. PASS BY VALUE  — the function takes ownership.
//        fn takes(s: String) { /* `s` lives here, then is dropped */ }
//        After the call, the caller can't use the original binding.
//
//   2. RETURN BY VALUE — the function gives ownership back.
//        fn makes() -> String { String::from("hi") }
//        The caller now owns the returned value.
//
//   3. BORROW          — the function only LOOKS at the value.
//        fn looks(s: &String) -> usize { s.len() }
//        Original is unaffected. Covered in own4 + own5.
//
// THE "TAKE AND GIVE BACK" PATTERN
//
// Sometimes you genuinely want a function to consume a value, modify it,
// and hand the modified version back. That looks like:
//
//     fn shout(mut s: String) -> String {
//         s.push('!');
//         s
//     }
//
//     let s = String::from("hi");
//     let s = shout(s);     // shadow with the returned value
//
// This is verbose. In real code we'd take `&mut String` instead. We do it
// the long way here on purpose — it makes the move-and-return dance vivid.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Implement three little functions:
//
//   - `take_and_drop(s)`    : takes `s: String` by value and does nothing
//                             with it. The point is that `s` is dropped at
//                             the end of the function. (Returns ().)
//
//   - `take_and_return(s)`  : takes a `String`, appends "!" to it, and
//                             returns the modified `String` to the caller.
//
//   - `roundtrip()`         : creates a `String`, hands it to
//                             `take_and_return`, and returns the result.
//                             Notice you must REBIND to capture the return.
//
// You may NOT use references (`&` / `&mut`) in this exercise.

// I AM NOT DONE

fn take_and_drop(s: String) {
    // Just let `s` go out of scope here. No body needed... but you DO need
    // to accept the parameter.
    let _ = ???;
}

fn take_and_return(mut s: String) -> String {
    s.???('!');     // mutate the owned String in place
    s               // return ownership to the caller
}

fn roundtrip() -> String {
    let s = String::from("hi");
    let s = take_and_return(???);   // move s in, get a new owned String back
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn drop_compiles() {
        // We can call it; the only assertion is that ownership transferred.
        let s = String::from("bye");
        take_and_drop(s);
        // If you uncomment the next line you should get a compile error:
        //   println!("{s}");
        // because `s` was moved into `take_and_drop`.
    }

    #[test] fn returns_modified() {
        assert_eq!(take_and_return(String::from("hi")), "hi!");
    }

    #[test] fn round_trips() {
        assert_eq!(roundtrip(), "hi!");
    }
}

fn main() {}
