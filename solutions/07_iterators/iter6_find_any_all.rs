// SOLUTION — iter6_find_any_all

fn first_negative(xs: &[i32]) -> Option<i32> {
    xs.iter().find(|&&x| x < 0).copied()
}

fn index_of(xs: &[i32], target: i32) -> Option<usize> {
    xs.iter().position(|&x| x == target)
}

fn has_zero(xs: &[i32]) -> bool {
    xs.iter().any(|&x| x == 0)
}

fn all_positive(xs: &[i32]) -> bool {
    xs.iter().all(|&x| x > 0)
}

fn longest_word<'a>(words: &'a [&'a str]) -> Option<&'a str> {
    words.iter().max_by_key(|w| w.len()).copied()
}


// WHY THIS IS OPTIMAL:
//
//   first_negative — `.find` returns the first matching ELEMENT (vs
//   `.position` which returns the index). The closure pattern `|&&x|`
//   peels off both layers: `&` because find passes by reference, and
//   `&` again because `.iter()` yielded `&i32` to begin with. The
//   trailing `.copied()` lifts `Option<&i32>` to `Option<i32>`.
//
//   index_of — `.position` takes the item BY VALUE (not by reference like
//   `find`/`filter`/`any`/`all`). Since `.iter()` yields `&i32`, the closure
//   parameter is `&i32`, so the pattern is `|&x|` (one ampersand, not two).
//   This asymmetry is annoying but worth memorising — the compiler error is
//   loud and the fix is small.
//
//   has_zero / all_positive — `.any` short-circuits at the first `true`,
//   `.all` short-circuits at the first `false`. Both take their argument
//   BY VALUE (unlike `find`/`filter` which take by reference), so the
//   closure pattern is `|&x|` — only one ampersand. On an empty iterator:
//   `any` is `false`, `all` is `true` — vacuous truth in action. The empty
//   tests pin those down.
//
//   longest_word — `max_by_key(|w| w.len())` finds the entry whose key
//   (length) is largest. The closure receives `&&&str` (filter/find/etc.
//   all add the extra reference); we call `.len()` on it directly because
//   method-call auto-derefs all the way through. `.copied()` turns
//   `Option<&&str>` into `Option<&str>`.
//
// ALTERNATIVES & GOTCHAS:
//
//   `xs.contains(&0)` is shorter than `has_zero`, and arguably better. The
//   point here is to drill `any`'s shape.
//
//   For ties on `max_by_key`, std returns the LAST max (`max`/`max_by_key`
//   keep replacing on `>=`). For ties on `min_by_key`, std returns the
//   FIRST. Asymmetric — write a custom fold if it matters.
//
//   `find` vs `filter().next()`: identical behaviour, identical performance
//   (filter is lazy). `find` reads more clearly when you only want one.
