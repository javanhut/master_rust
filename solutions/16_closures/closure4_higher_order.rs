// SOLUTION — closure4_higher_order

fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn transform(slice: &[i32], op: impl Fn(i32) -> i32) -> Vec<i32> {
    slice.iter().map(|&n| op(n)).collect()
}

fn repeat<F: FnMut()>(n: u32, mut f: F) {
    for _ in 0..n {
        f();
    }
}

// WHY THIS IS OPTIMAL:
//
//   apply_twice — `Fn(i32) -> i32` is exactly right: we only need to call
//   `f` (twice), we don't need to mutate it, and we want to allow callers
//   to pass any closure that just reads its environment. The `where`-clause
//   form keeps the signature line short and is the recommended style as
//   bounds get more complex.
//
//   transform — `impl Fn(i32) -> i32` in argument position is the most
//   ergonomic spelling for a single, simple bound. It's pure sugar for a
//   generic with a `Fn` bound; the compiler still monomorphizes one
//   specialized `transform` per call site, so there is no runtime cost.
//
//   repeat — we must allow callers to pass a *counter-bumping* closure
//   (the test does), which mutates its captured environment. So `FnMut`
//   is the right bound. Calling an `FnMut` requires `&mut F`, and we
//   express that by binding the parameter as `mut f`.
//
// PARAMETER BINDING DETAIL:
//
//   `fn repeat<F: FnMut()>(n: u32, mut f: F)` — the `mut` here is the
//   binding mutability we met in chapter 1. `F` itself isn't a reference;
//   we own `f`, and we make our local binding mutable so we can call
//   `f()` (which under the hood goes through `&mut self`).
//
// ALTERNATIVES:
//
//   apply_twice could use the inline form:
//       fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }
//   Same generated code; `where` is editorial preference.
//
//   transform could be more generic over element type:
//       fn transform<T, U>(slice: &[T], op: impl Fn(&T) -> U) -> Vec<U>
//   ...but the chapter is about closures, not generics gymnastics.
//
//   For the dynamic-dispatch flavor, you could write:
//       fn transform(slice: &[i32], op: &dyn Fn(i32) -> i32) -> Vec<i32>
//   This emits exactly one copy of `transform` regardless of how many
//   different closures get passed in — useful for code-size-sensitive
//   contexts, but slightly slower per element due to the indirect call.
//
// PERFORMANCE NOTE:
//
//   The static-dispatch versions are typically inlined by the optimizer,
//   so a `transform(.., |n| n + 1)` boils down to a tight loop with no
//   call overhead at all. Generics + closures = zero-cost iteration in
//   the literal sense.
