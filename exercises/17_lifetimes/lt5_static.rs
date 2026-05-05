// =============================================================================
//  lt5 — `'static`: lives for the whole program
// =============================================================================
//
// `'static` is a special, named lifetime: it means "valid for the entire
// duration of the program." Anything with the lifetime `'static` will be
// available from the moment `main` starts to the moment the process exits.
//
// THE TWO MOST COMMON `'static` THINGS
//
//   - String literals.   `"hello"` has type `&'static str`.
//                        Literals live in the program's read-only data
//                        section, baked into the binary itself. They
//                        outlive everything you can write.
//
//   - `static` items.    `static MESSAGE: &str = "hi";` declares a
//                        compile-time constant in static memory.
//
// `'static` IS A LIFETIME, NOT A TYPE
//
//   `&'static str`       is a string slice that points into memory that
//                        will live forever — the literal "hello" qualifies.
//
//   `String`             is owned and heap-allocated. By itself it has no
//                        borrowed parts, so `String: 'static` (the BOUND
//                        — see below) holds. But `String` is NOT a
//                        `&'static str`.
//
// THE BOUND `T: 'static` (a different beast)
//
//   When you write a generic with `T: 'static`, you are NOT saying "T
//   lives forever". You are saying "T contains no references that live
//   for less than `'static`." Concretely:
//
//     - `i32: 'static`           ✔ (no references at all)
//     - `String: 'static`        ✔ (owned data, no borrowed fields)
//     - `Vec<u8>: 'static`       ✔
//     - `&'static str: 'static`  ✔ (the reference IS `'static`)
//     - `&'a str: 'static`       ✘ for a generic `'a` (it might be short)
//
//   Functions that accept `T: 'static` (e.g., `Box<dyn Trait + 'static>`,
//   `std::thread::spawn`) want a value that won't accidentally drag in a
//   short-lived borrow. Owned values pass the bound trivially.
//
// NOT EVERY REFERENCE IS `'static`!
//
// A common beginner trap is to think "if I write `&str`, it's just like a
// literal, so it's `&'static str`." NO. The literal `"hello"` is
// `&'static str` because the COMPILER stored it in static memory. A
// `&str` you got by slicing a `String` borrows from the heap and dies
// when the `String` does — far short of `'static`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Implement two small functions and one bound check.
//
//   1. `greeting() -> &'static str`
//        Return a string literal. Easy — but notice the explicit
//        `'static` in the signature. We're being deliberate.
//
//   2. `fn lives_forever<T: 'static>(_t: T) -> &'static str`
//        Take ANY `T: 'static` value (we don't care what it is) and
//        return the literal "ok". The point is the BOUND: this function
//        compiles only when called with values that contain no
//        short-lived borrows.
//
//   3. `fn first_static_str(strs: &[&'static str]) -> Option<&'static str>`
//        Take a slice of `'static` string slices and return the first
//        one (or `None` if the slice is empty). Use `strs.first()` and
//        `.copied()` (because `&&'static str` -> `&'static str` requires
//        a copy of the inner reference).
//
// The tests will exercise both successful paths and (via the
// `t_static_bound` test) the fact that `String` satisfies `T: 'static`.

// I AM NOT DONE

fn greeting() -> &'static str {
    ???
}

fn lives_forever<T: 'static>(_t: T) -> &'static str {
    ???
}

fn first_static_str(strs: &[&'static str]) -> Option<&'static str> {
    strs.first().???()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn greeting_is_literal() {
        assert_eq!(greeting(), "hello, world");
    }

    // The interesting compile-time check: a String contains no borrowed
    // data, so `String: 'static` is satisfied. This call must compile.
    #[test] fn t_static_bound_accepts_owned() {
        let owned = String::from("anything");
        assert_eq!(lives_forever(owned), "ok");
    }

    #[test] fn t_static_bound_accepts_primitives() {
        assert_eq!(lives_forever(42_i32), "ok");
        assert_eq!(lives_forever((1.0_f64, true)), "ok");
    }

    #[test] fn first_of_static_slices() {
        let arr: &[&'static str] = &["alpha", "beta", "gamma"];
        assert_eq!(first_static_str(arr), Some("alpha"));
    }

    #[test] fn first_of_empty_slice_is_none() {
        let arr: &[&'static str] = &[];
        assert_eq!(first_static_str(arr), None);
    }
}

fn main() {}
