// SOLUTION — vec4_iterate

fn sum_borrow(v: &Vec<i32>) -> i32 {
    let mut total = 0;
    for &x in v {
        total += x;
    }
    total
}

fn double_in_place(v: &mut Vec<i32>) {
    for x in &mut *v {
        *x *= 2;
    }
}

fn sum_consume(v: Vec<i32>) -> i32 {
    let mut total = 0;
    for x in v {
        total += x;
    }
    total
}

// WHY THESE ARE OPTIMAL:
//
//   `for &x in v` — `v` here is `&Vec<i32>`; iterating it yields `&i32`.
//   The `&x` PATTERN destructures the reference so `x: i32`. Without `&`,
//   you'd have `x: &i32` and need `total += *x`. Both compile; `&x` reads
//   cleaner.
//
//   `for x in &mut *v` — `v` here is `&mut Vec<i32>`. Writing `&mut v` would
//   re-borrow but `&mut *v` is the explicit form that's bullet-proof against
//   reborrow surprises. Inside, `x: &mut i32`, so we deref with `*x` to
//   assign through the mutable reference. `for x in v.iter_mut()` is also
//   accepted style.
//
//   `for x in v` (no `&`) — this calls `IntoIterator for Vec<T>`, which
//   yields owned `T`. The Vec is consumed; trying to use `v` after the loop
//   is a compile error. This is exactly what you want when you're computing
//   a final value and won't need the collection again.
//
// THE THREE-FLAVOUR PRINCIPLE:
//
//   Anything in std that holds a sequence (Vec, slice, HashMap, BTreeMap,
//   ...) gives you the same trio:
//       .iter()        -> &T
//       .iter_mut()    -> &mut T
//       .into_iter()   -> T
//   Once you've internalised this for Vec, every other collection in std
//   is the same idea.
//
// LOOK-AHEAD TO CHAPTER 7:
//
//   `v.iter().sum()` and `v.iter_mut().for_each(|x| *x *= 2)` do the same
//   work in idiomatic style. We're using explicit `for` loops here so the
//   borrow / deref mechanics stay in your face. Iterators get their own
//   chapter.
