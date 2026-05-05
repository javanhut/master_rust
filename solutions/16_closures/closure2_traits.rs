// SOLUTION — closure2_traits

fn call_twice<F: Fn() -> i32>(f: F) -> i32 {
    f() + f()
}

fn accumulate<F: FnMut() -> i32>(n: u32, mut f: F) -> i32 {
    let mut last = 0;
    for _ in 0..n {
        last = f();
    }
    last
}

fn consume<F: FnOnce() -> String>(f: F) -> String {
    f()
}

// WHY THIS IS OPTIMAL:
//
//   call_twice — we call `f` twice, so we need at least FnMut. But the
//   tests pass closures that don't mutate anything. Choosing `Fn` is the
//   STRICTEST contract from the caller's side, but actually the WEAKEST
//   from ours: `Fn` closures can be called through `&self`, so calling
//   `f()` twice is trivially OK. Since the bodies in this exercise don't
//   mutate, `Fn` is the right fit and documents that intent.
//
//   accumulate — we call `f` repeatedly AND we want callers to be able to
//   mutate captured state (the counter test). So `FnMut`. We bind `f` as
//   `mut f` because calling an FnMut requires `&mut F`. We could also use
//   `Fn` here for the read-only case, but that would reject the counter
//   closure — too strict.
//
//   consume — `FnOnce` is the weakest, most permissive bound. ANY closure
//   satisfies `FnOnce`, including ones that move ownership out of their
//   captures (like `move || s`). We can only call it once, but that's all
//   we need. Asking for `Fn` here would have rejected the test's `move`
//   closure that returns its captured String.
//
// THE KEY RULE OF THUMB:
//
//   "Pick the WEAKEST bound that lets your function body do its job."
//
//   Fn       → if you only call it through `&` and never need to mutate
//   FnMut    → if you need to call it multiple times AND allow mutation
//   FnOnce   → if you only call it once (you can also accept consumers)
//
// ALTERNATIVES:
//
//   Trailing return-position `impl Fn(...)` and `impl FnMut(...)` syntax
//   exists for parameter position too:
//
//       fn call_twice(f: impl Fn() -> i32) -> i32 { f() + f() }
//
//   It's purely sugar for the generic form — pick whichever reads better.
//   For multiple bounds or `where`-clause complexity, prefer the explicit
//   `<F: Fn(...)>` form.
//
// FUNCTION POINTERS vs CLOSURE TRAITS:
//
//   `fn(i32) -> i32` (lowercase fn) is a *function pointer* type — it can
//   only point to non-capturing closures or named fns. Anything that
//   captures must go through Fn/FnMut/FnOnce.
