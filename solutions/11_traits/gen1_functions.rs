// SOLUTION — gen1_functions

fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    let mut best = &xs[0];
    for x in xs {
        if x > best {
            best = x;
        }
    }
    best
}

fn min_pair<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

// WHY THIS IS OPTIMAL:
//
//   `largest` returns `&T` — a borrow into the slice — so it works without
//   requiring `T: Clone`. The body only uses `>`, which is a method on
//   `PartialOrd`, so that's the only bound we need.
//
//   `min_pair` takes `T` by value, returns `T` by value. We OWN `a` and
//   `b`, we move whichever wins out. No clone, no borrow gymnastics. Once
//   again, `PartialOrd` is the minimum bound that lets `<` compile.
//
//   The same source compiles into specialised machine code for `i32`,
//   `f64`, `char`, `String` — that's monomorphization. At a call site like
//   `largest(&v)` where `v: Vec<i32>`, the compiler picks the i32 copy at
//   no runtime cost.
//
// WHY `PartialOrd` AND NOT `Ord`:
//
//   `Ord` is the stricter "total order" trait — every pair is comparable.
//   `PartialOrd` allows partial orderings, the most famous example being
//   `f64::NAN`, which is incomparable with everything (including itself).
//
//   Choosing `PartialOrd` here means our function ACCEPTS f64 slices.
//   Choosing `Ord` would have rejected f64. As an API author, prefer the
//   weakest bound that still makes the code correct — it lets your callers
//   pass more types.
//
// WHY RETURN `&T` IN `largest`:
//
//   The slice owns its elements; we're only inspecting them. Returning
//   `&T` is borrow-friendly: no `T: Clone` bound, no allocation. The
//   caller writes `*largest(&v)` (when T: Copy) or `largest(&v).clone()`
//   if they want an owned copy.
//
// EQUIVALENT BUT WORSE:
//
//   fn largest<T: PartialOrd + Clone>(xs: &[T]) -> T { ... }
//   Adds an unnecessary `Clone` bound and one allocation/copy per call.
//   Only use this shape if you genuinely need an owned T at the call site.
//
//   fn largest<T: Ord>(xs: &[T]) -> &T { ... }
//   Same body works (Ord: PartialOrd), but rejects f64. Strictly worse for
//   no payoff.
//
// MIN_PAIR — `<` vs `<=`:
//
//   `if a < b { a } else { b }` returns `b` on ties. `if a <= b { a }
//   else { b }` returns `a` on ties. Either is fine — pick the
//   tie-breaking rule your API needs and document it.
