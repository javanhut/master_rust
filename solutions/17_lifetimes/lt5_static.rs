// SOLUTION — lt5_static

fn greeting() -> &'static str {
    "hello, world"
}

fn lives_forever<T: 'static>(_t: T) -> &'static str {
    "ok"
}

fn first_static_str(strs: &[&'static str]) -> Option<&'static str> {
    strs.first().copied()
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `greeting` is the canonical `&'static str` example: the body is a
//   bare string literal. The compiler stores "hello, world" in the
//   program's read-only data section, hands you a slice into it, and
//   labels that slice with the longest possible lifetime. Returning
//   `&'static str` is the right move whenever you have a fixed string
//   and want callers to know it'll never invalidate.
//
//   `lives_forever<T: 'static>` is the demonstration of the BOUND form
//   of `'static`. We don't use `t` at all — the test is whether the
//   call compiles. In `t_static_bound_accepts_owned`, we pass a
//   `String`. The compiler asks: "does `String` outlive `'static`?"
//   The answer is yes, because `String` borrows from nothing — its
//   internal heap pointer is owned, not borrowed. Same story for
//   `i32`, `(f64, bool)`, etc.
//
//   `first_static_str` exists to drive home that `&'static str` is
//   just a regular reference type — you can put it in slices, in
//   `Option`, in `Vec`. `strs.first()` gives `Option<&&'static str>`;
//   `.copied()` collapses the outer reference into a `Option<&'static
//   str>` because `&'static str` is `Copy`.
//
// WHAT THE BOUND `T: 'static` PROTECTS
//
//   A function like `std::thread::spawn` requires its closure (and
//   anything captured by the closure) to be `'static`. Why? Because the
//   spawned thread can outlive the spawner. If you captured a `&local`
//   reference in the closure, the local could be dropped before the
//   thread reads it — UAF. The `'static` bound forbids that at compile
//   time: "the captured stuff must not be borrowed from anything
//   shorter-lived than the program."
//
//   Owned values pass trivially. Borrowed values pass only if they're
//   already `&'static`. That's why threading code so often takes
//   ownership via `move` and avoids `&` captures.
//
// COMMON CONFUSIONS
//
//   - "&str is &'static str." — NO. `&str` is shorthand for `&'_ str`,
//     where `'_` is whatever the elision/inference picks. It's `&'static
//     str` only when the source is a literal or `static` item. A
//     reference into a `String` you allocated this morning is NOT
//     `'static` — it dies with the `String`.
//
//   - "String is 'static." — As a value, a `String` IS `T: 'static`
//     because it has no borrowed parts. But a `&String` is NOT
//     `&'static String` unless the `String` itself is in static memory
//     (rare, requires `static` or `OnceCell`/`OnceLock`).
//
// ALTERNATIVES YOU'LL SEE:
//
//   1. Rather than `T: 'static`, threading APIs sometimes take
//      `T: Send + 'static`. The extra `Send` bound says "this type can
//      cross thread boundaries safely." Independent of lifetimes.
//
//   2. `OnceLock<String>` (stable since 1.70) gives you a way to
//      construct a `String` at runtime and then hand out
//      `&'static String` references to it for the rest of the program.
//      The `'static` claim is real — the `OnceLock` is itself a
//      `static`, so its contents will outlive every caller.
//
//   3. To turn a runtime `String` into a `&'static str` at the cost of a
//      one-time leak, `Box::leak(s.into_boxed_str())` is the trick. It
//      hands you a `&'static mut str` because the box is never freed.
//      Use sparingly — leaks accumulate.
