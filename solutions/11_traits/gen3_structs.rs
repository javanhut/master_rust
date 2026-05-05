// SOLUTION — gen3_structs

use std::fmt::Display;

struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    fn into_tuple(self) -> (T, T) {
        (self.a, self.b)
    }
}

impl<T: Display> Pair<T> {
    fn show(&self) -> String {
        format!("({}, {})", self.a, self.b)
    }
}

// WHY THIS IS OPTIMAL:
//
//   The struct itself is unconditionally generic — `Pair<T>` works for
//   ANY T, with no bounds. That's the right default: don't restrict the
//   type unless a method actually needs the restriction.
//
//   Two impl blocks separate the unconditional API (new, into_tuple)
//   from the conditional API (show, only for `T: Display`). This is the
//   standard Rust pattern: split impls by bound to make it obvious which
//   methods require what.
//
//   `into_tuple(self)` takes ownership and MOVES the fields out into a
//   tuple. The Pair is consumed; the caller now owns `(a, b)`. No clone,
//   no copy bound — moves work for any T.
//
//   `show(&self)` borrows the pair (we want the caller to keep it) and
//   uses `{}` to format `a` and `b`, which requires Display.
//
// FIELD-INIT SHORTHAND:
//
//   `Self { a, b }` works because the parameter names match the field
//   names. Without that, you'd write `Self { a: a, b: b }` — same thing,
//   noisier.
//
// CALLING `.show()` ON A NON-DISPLAY T:
//
//   struct NoDisplay;
//   let p = Pair::new(NoDisplay, NoDisplay);
//   p.show();     // ← error: the trait `Display` is not implemented
//                 //   for `NoDisplay`
//
//   The error fires at the call site of `.show()`, not at the struct
//   definition. This is the conditional-impl experience: define what's
//   true, the compiler enforces it where you try to use it.
//
// EQUIVALENT BUT WORSE:
//
//     struct Pair<T: Display> { a: T, b: T }
//
//   This puts the bound on the STRUCT, which means you can no longer
//   build `Pair<NonDisplayThing>` at all — even if you never wanted to
//   print it. A bound on the struct propagates to every impl block; a
//   bound on a single impl is much more flexible. The general rule:
//   bound the impl, not the type.
//
// PARALLEL TO STD:
//
//   Vec<T>::sort() is `impl<T: Ord> Vec<T>`. Hash maps' `iter` is
//   unconditional, but methods that need ordering are gated. This is the
//   exact same pattern, scaled up.
