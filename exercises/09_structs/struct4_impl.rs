// =============================================================================
//  struct4 — `impl` blocks: methods with &self, &mut self, and self
// =============================================================================
//
// Methods are functions attached to a type. You write them inside an `impl`
// block:
//
//     struct Counter { value: u32 }
//
//     impl Counter {
//         fn get(&self) -> u32        { self.value }
//         fn bump(&mut self)          { self.value += 1; }
//         fn into_value(self) -> u32  { self.value }
//     }
//
// Three flavours of `self` — and which one to pick is the heart of method
// design in Rust:
//
//   `&self`       — read-only borrow of the receiver. Most common form.
//                   The caller keeps owning the value; multiple `&self`
//                   methods can be called in sequence.
//
//   `&mut self`   — exclusive, mutable borrow. The method may change the
//                   value's fields. Caller keeps ownership.
//
//   `self`        — takes ownership of the value (it is MOVED into the
//                   method). Use this when the method consumes the
//                   receiver — converting it into something else, or
//                   deliberately ending its life. After the call the
//                   original binding can no longer be used.
//
// CALLING METHODS
//
//     let c = Counter { value: 0 };
//     c.get();             // calls `Counter::get(&c)` — auto-borrow
//     let mut c = c;
//     c.bump();            // calls `Counter::bump(&mut c)`
//     let n = c.into_value();   // moves c
//     // c is no longer usable here — into_value consumed it.
//
// Rust automatically inserts `&` or `&mut` so you don't have to write them
// at the call site. You DO need `let mut` on the binding to call a `&mut
// self` method, just like any other mutation.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Finish `Counter`:
//   - `get(&self) -> u32`        — return the current value (read-only).
//   - `bump(&mut self)`          — increment `self.value` by 1.
//   - `add(&mut self, n: u32)`   — add `n` to `self.value`.
//   - `into_value(self) -> u32`  — consume the counter, returning the value.
//
// Then complete `run_counter` to drive it.

// I AM NOT DONE

#[derive(Debug)]
struct Counter {
    value: u32,
}

impl Counter {
    fn get(???) -> u32 {
        self.value
    }

    fn bump(???) {
        self.value ??? 1;
    }

    fn add(???, n: u32) {
        self.value ??? n;
    }

    fn into_value(???) -> u32 {
        self.value
    }
}

// Build a Counter at 0, bump it twice, add 10, then consume it and return
// the final value.
fn run_counter() -> u32 {
    let ??? c = Counter { value: 0 };
    c.bump();
    c.bump();
    c.add(10);
    c.???()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn get_reads_value() {
        let c = Counter { value: 7 };
        assert_eq!(c.get(), 7);
    }
    #[test] fn bump_increments() {
        let mut c = Counter { value: 0 };
        c.bump();
        c.bump();
        c.bump();
        assert_eq!(c.get(), 3);
    }
    #[test] fn add_works() {
        let mut c = Counter { value: 5 };
        c.add(8);
        assert_eq!(c.get(), 13);
    }
    #[test] fn into_value_consumes() {
        let c = Counter { value: 42 };
        let n = c.into_value();
        // `c` is no longer usable here — that is the whole point of `self`.
        assert_eq!(n, 42);
    }
    #[test] fn run_counter_correct() {
        assert_eq!(run_counter(), 12);
    }
}

fn main() {}
