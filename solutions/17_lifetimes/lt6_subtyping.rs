// SOLUTION — lt6_subtyping

fn apply_to_str(s: &str, f: impl Fn(&str) -> usize) -> usize {
    f(s)
}

fn count_bytes(s: &str) -> usize {
    s.len()
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The two functions are minimal on purpose. The interesting code is
//   the test cases — they show that calling `apply_to_str` with a
//   `&'static str` and with a `&'short str` (borrowed from a local
//   `String`) both work, even though the signature is just `&str`.
//
//   What's actually happening: when `apply_to_str` takes `s: &str`, the
//   elided form is `apply_to_str<'a>(s: &'a str, f: ...)`. At each call
//   site, the compiler picks `'a` to be whatever the input is — the
//   literal's `'static`, or the local's short scope. There's no fixed
//   "required lifetime" for the parameter; the function adapts.
//
//   For the bonus closure test, the closure captures a `&'static str`
//   (`prefix`) but is called with `&str` arguments of varying
//   lifetimes. The closure's `Fn(&str) -> usize` impl quantifies over
//   any input lifetime — `for<'a> Fn(&'a str) -> usize` under the hood.
//   That's known as a "higher-ranked trait bound," and it's exactly what
//   makes callbacks taking `&str` ergonomic to write.
//
// THE COVARIANCE CONNECTION
//
//   "Covariant in lifetime" is the property that `&'long T` is
//   acceptable wherever `&'short T` is. Most reference-like Rust types
//   are covariant: `&T`, `&mut T` (in T but not in lifetime — careful),
//   `Box<T>`, `Vec<T>`, `Option<T>`. The exceptions are mutable
//   references in their inner type and a few cells.
//
//   You don't need to memorize the variance table to write Rust. Just
//   internalize: a longer-lived reference can stand in for a
//   shorter-lived one, never the reverse.
//
// WHY THE OTHER DIRECTION FAILS
//
//   Suppose `apply_to_str` instead REQUIRED a `&'static str` and you
//   tried to pass `&owned` (with a local lifetime). The compiler would
//   refuse: `owned` is dropped at the end of the function, but the
//   `'static` requirement says "valid forever". Once the parameter
//   demands `'static`, only literals (and `static` items, leaks,
//   `OnceLock`-shared data, …) qualify.
//
// ALTERNATIVES:
//
//   1. Explicit higher-ranked bound — usually unnecessary because the
//      compiler infers it, but you can write:
//
//          fn apply_to_str<F: for<'a> Fn(&'a str) -> usize>(s: &str, f: F) -> usize {
//              f(s)
//          }
//
//      The `for<'a>` is the explicit form of "any lifetime works." If
//      you ever see HRTB errors mentioning `for<'r>`, this is what
//      they're referring to.
//
//   2. Function pointers instead of `impl Fn`:
//
//          fn apply_to_str(s: &str, f: fn(&str) -> usize) -> usize { f(s) }
//
//      Same covariance behavior; you trade away the ability to pass
//      capturing closures.
//
//   3. For mutable references, variance flips: `&mut T` is INVARIANT in
//      `T`. The lifetime is still covariant, though. The full variance
//      table only matters when you're authoring container types — most
//      day-to-day Rust never bumps into it.
