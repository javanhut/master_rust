// =============================================================================
//  gen1 — generic functions
// =============================================================================
//
// A GENERIC function is one parameterised over a TYPE — it works for many
// concrete types as long as those types meet some requirements.
//
//     fn first<T>(xs: &[T]) -> &T {
//         &xs[0]
//     }
//
// Read this as: "for any type `T`, `first` takes a slice of T and returns
// a reference to the first element". `T` is a TYPE PARAMETER; you declare
// it in angle brackets right after the function name.
//
// TRAIT BOUNDS — what the body is allowed to do
//
// Inside a generic function, the compiler ONLY lets you use operations
// that are guaranteed to exist for every possible `T`. By default that's
// almost nothing — you can pass it around, take references to it, drop it.
// You cannot compare it with `<`, you cannot print it, you cannot clone it.
//
// To unlock those, you ADD A BOUND: "T must implement these traits".
//
//     fn largest<T: PartialOrd>(xs: &[T]) -> &T {
//         //          ^^^^^^^^^^^ trait bound
//         let mut best = &xs[0];
//         for x in xs {
//             if x > best { best = x; }
//         }
//         best
//     }
//
// Without `T: PartialOrd`, `x > best` would not compile — `>` is itself a
// trait method, and the bound is what tells the compiler "yes, `>` is
// defined for any T we'll ever be called with".
//
// MONOMORPHIZATION — what the compiler does
//
// Generics in Rust are RESOLVED AT COMPILE TIME. When you call
// `largest(&[1, 2, 3])` (a slice of `i32`) AND `largest(&[1.0, 2.0])` (a
// slice of `f64`), the compiler stamps out TWO copies of `largest`:
//
//     fn largest__i32(xs: &[i32]) -> &i32 { ... }
//     fn largest__f64(xs: &[f64]) -> &f64 { ... }
//
// Each copy is fully specialised to its concrete type. There is NO runtime
// dispatch, NO vtable lookup; calling a generic function is exactly as
// cheap as calling a non-generic function. The cost is a slight increase
// in binary size (one copy per concrete T actually used).
//
// You'll meet `dyn Trait` later in this chapter for the OPPOSITE trade-off
// (one copy of the function, dynamic dispatch on the type).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - Write `largest<T>(xs: &[T]) -> &T` with the right trait bound so that
//     `x > best` compiles. The bound you want is `PartialOrd`.
//
//   - Write `min_pair<T: PartialOrd>(a: T, b: T) -> T` returning whichever
//     of `a` or `b` compares smaller. (Take them BY VALUE — that means we
//     also need to be able to RETURN one of them, which is fine because
//     we own them. No `Clone` bound needed.)

// I AM NOT DONE

fn largest<T: ???>(xs: &[T]) -> &T {
    let mut best = &xs[0];
    for x in xs {
        if x ??? best {
            best = x;
        }
    }
    best
}

fn min_pair<T: ???>(a: T, b: T) -> T {
    if a ??? b { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn largest_ints() {
        let v = vec![10, 25, 3, 99, 7];
        assert_eq!(*largest(&v), 99);
    }

    #[test] fn largest_floats() {
        // Same generic function, different concrete T — monomorphization
        // gives us a separate compiled copy for f64.
        let v = vec![1.5_f64, 2.25, 0.5];
        assert_eq!(*largest(&v), 2.25);
    }

    #[test] fn largest_chars() {
        // And once more for char — char: PartialOrd.
        let v = vec!['a', 'z', 'm'];
        assert_eq!(*largest(&v), 'z');
    }

    #[test] fn min_pair_ints() {
        assert_eq!(min_pair(7, 3), 3);
        assert_eq!(min_pair(-1, 0), -1);
    }

    #[test] fn min_pair_strings() {
        let a = String::from("banana");
        let b = String::from("apple");
        // String: PartialOrd, lexicographic order.
        assert_eq!(min_pair(a, b), "apple");
    }
}

fn main() {}
