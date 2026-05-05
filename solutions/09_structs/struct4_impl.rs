// SOLUTION — struct4_impl

#[derive(Debug)]
struct Counter {
    value: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.value
    }

    fn bump(&mut self) {
        self.value += 1;
    }

    fn add(&mut self, n: u32) {
        self.value += n;
    }

    fn into_value(self) -> u32 {
        self.value
    }
}

fn run_counter() -> u32 {
    let mut c = Counter { value: 0 };
    c.bump();
    c.bump();
    c.add(10);
    c.into_value()
}

// WHY THIS IS OPTIMAL:
//
//   Each method takes the LEAST powerful receiver that does the job:
//
//     - `get` only reads, so `&self` — many readers can coexist.
//     - `bump`/`add` mutate, so `&mut self` — exclusive access, but the
//       caller keeps ownership.
//     - `into_value` truly ends the counter's life: the value is gone,
//       there is nothing to read afterwards. Taking `self` makes that
//       fact unforgeable in the type system.
//
//   `+=` on a `u32` is cheap, in-place arithmetic. There is no clone, no
//   allocation, no extra binding — the method body is one instruction.
//
//   `let mut c` in `run_counter` is required because we call `&mut self`
//   methods on it. Without `mut`, Rust refuses to hand out the exclusive
//   borrow.
//
// EQUIVALENT BUT NOISIER:
//
//   fn bump(&mut self) {
//       self.value = self.value + 1;     // works, but `+=` is idiomatic
//   }
//
//   fn into_value(self) -> u32 {
//       let Counter { value } = self;    // destructure instead of `.value`
//       value
//   }
//   Useful when there are several fields and you want them all named locally.
//
//   You COULD make `into_value` take `&self` and clone the `u32` — but that
//   misses the point. `self` communicates "this method ends the counter",
//   which is sometimes important API documentation in itself.
