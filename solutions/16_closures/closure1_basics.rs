// SOLUTION — closure1_basics

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn times_called(k: u32) -> u32 {
    let mut count = 0u32;
    let mut bump = || { count += 1; };
    for _ in 0..k {
        bump();
    }
    count
}

fn greet_owned(name: String) -> String {
    let greeter = move || format!("Hello, {name}!");
    greeter()
}

// WHY THIS IS OPTIMAL:
//
//   make_adder — `move |x| x + n` captures `n` by value (i32 is Copy, so
//   "moving" it is just a copy). We MUST use `move` here because the
//   returned closure outlives the function frame that owns `n`. Without
//   `move`, the closure would try to borrow `n` from a stack frame that's
//   about to disappear — the borrow checker rejects that.
//
//   times_called — the closure mutates `count`, so the compiler infers a
//   `&mut count` capture. That makes `bump` an `FnMut`, and we must bind
//   it with `let mut bump` to call it through a mutable reference.
//   We do NOT use `move` here — we want the closure to borrow `count` so
//   we can still read `count` after the loop.
//
//   greet_owned — `move ||` takes ownership of `name`. Because `String` is
//   not Copy, this transfers the heap-allocated buffer into the closure.
//   `format!("{name}")` re-borrows it inside the closure.
//
// ALTERNATIVES:
//
//   make_adder could return Box<dyn Fn(i32) -> i32> — works, but adds a
//   heap allocation and dynamic dispatch for no reason here. Prefer
//   `impl Fn` for static dispatch when you can.
//
//   times_called could compute `k as u32` directly without a closure —
//   but the point of the exercise is to see the FnMut capture in action.
//
//   greet_owned could take `&str` and avoid the move entirely. The `move`
//   here is pedagogical: you are watching ownership transfer from caller
//   to closure.
//
// MENTAL MODEL:
//   The compiler desugars a closure into an anonymous struct that stores
//   the captured environment, plus an impl of Fn / FnMut / FnOnce that
//   defines `call(&self, ...)` (or &mut self, or self). `move` just means
//   "the struct's fields are owned values, not references".
