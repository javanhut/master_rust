// SOLUTION — iter2_map_filter

fn doubled(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().map(|x| x * 2).collect()
}

fn evens(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().filter(|&x| x % 2 == 0).collect()
}

fn lazy_does_nothing() {
    let _ = (1u64..=1_000_000_000).map(|x| panic!("never runs! x={}", x));
}


// WHY THIS IS OPTIMAL:
//
//   doubled — `.iter().copied()` turns `&i32` items into `i32` items so the
//   map closure can write the simple `|x| x * 2`. `.collect()` is the
//   terminator that actually drives the chain. The return type `Vec<i32>`
//   tells `collect` what to build — no turbofish needed.
//
//   evens — `.filter` receives the predicate `|&x| x % 2 == 0`. Even after
//   `.copied()`, filter still passes its argument by reference (`&i32`),
//   because that's the trait's signature. The `&x` pattern destructures
//   that reference back into a plain `i32`.
//
//   lazy_does_nothing — building a `.map` adapter is just describing work.
//   No `.collect`, no `.sum`, no `.for_each` → the closure is never called.
//   Compiles, returns immediately, never panics. This is the foundational
//   mental model: an iterator chain is INERT until you terminate it.
//
// ALTERNATIVES:
//
//   doubled without `.copied()`:
//       xs.iter().map(|&x| x * 2).collect()
//   The destructuring pattern `|&x|` does the same job. Pick what reads.
//
//   evens without `.copied()`:
//       xs.iter().filter(|&&x| x % 2 == 0).copied().collect()
//   Here filter sees `&&i32` so the pattern is `|&&x|`. Functionally
//   identical; `.copied()` upfront keeps the rest of the chain simpler.
//
//   For very large inputs, `.filter(...).map(...)` is preferable to
//   `.map(...).filter(...)` because the map closure only runs on survivors.
