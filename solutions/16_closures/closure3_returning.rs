// SOLUTION — closure3_returning

fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn adjuster(mode: &str) -> Box<dyn Fn(i32) -> i32> {
    match mode {
        "double" => Box::new(|x| x * 2),
        "negate" => Box::new(|x| -x),
        _        => Box::new(|x| x),
    }
}

// WHY THIS IS OPTIMAL:
//
//   multiplier — only ONE closure shape ever leaves this function, so
//   `impl Fn(i32) -> i32` is ideal. Each call to `multiplier` produces a
//   distinct concrete type at compile time (the closure type captures
//   `factor` as a field), but each call site sees a single, known type.
//   No allocation, no vtable, calls inline beautifully.
//
//   `move |x| x * factor` — `move` because the closure outlives the
//   function frame that owns `factor`. (i32 is Copy, so the "move" is
//   really a copy into the closure's struct.)
//
//   adjuster — the three branches each produce a DIFFERENT closure type
//   (Rust gives every closure a fresh anonymous type). `impl Fn` cannot
//   express "one of three different types"; we need type erasure. So we
//   put each closure on the heap inside a `Box<dyn Fn(i32) -> i32>` and
//   return the trait object. All three branches now have the same return
//   type, so the match compiles.
//
// COST OF Box<dyn Fn>:
//   - One heap allocation when the box is created.
//   - One pointer indirection + vtable lookup per call (`f(x)`).
//   In typical code this is utterly invisible. In a hot inner loop, prefer
//   `impl Fn` if you can.
//
// ALTERNATIVES:
//
//   1. An enum that names each closure variant explicitly:
//
//          enum Adjust { Double, Negate, Identity }
//          impl Adjust { fn apply(&self, x: i32) -> i32 { ... } }
//
//      No allocation, no dyn, but you give up the closure ergonomics and
//      force every variant to be known up front.
//
//   2. A function pointer `fn(i32) -> i32` works here because none of
//      these closures capture anything — non-capturing closures coerce
//      to fn pointers. Cheaper than `Box<dyn Fn>`, but the moment you
//      add a capture, this stops working.
//
//   3. `Rc<dyn Fn(...)>` if you need to share a single closure between
//      multiple owners. Same vtable cost, plus refcount.
