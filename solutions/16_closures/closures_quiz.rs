// SOLUTION — closures_quiz

pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0u32;
    move || {
        count += 1;
        count
    }
}

pub struct EventBus {
    handlers: Vec<Box<dyn FnMut(&str)>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus { handlers: Vec::new() }
    }

    pub fn register<F>(&mut self, handler: F)
    where
        F: FnMut(&str) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn fire(&mut self, event: &str) {
        for h in self.handlers.iter_mut() {
            h(event);
        }
    }
}

// WHY THIS IS OPTIMAL:
//
//   make_counter — three things to notice:
//
//     1. `move` is mandatory. Without it, the closure would borrow `count`
//        from the stack frame of `make_counter`, which dies the moment the
//        function returns. With `move`, `count` becomes a field of the
//        anonymous closure struct and lives as long as the closure.
//
//     2. The closure mutates `count`, so it implements `FnMut`, not `Fn`.
//        The return type `impl FnMut() -> u32` advertises that. Callers
//        therefore must bind the result with `let mut next = ...`.
//
//     3. Each call to `make_counter` produces a NEW closure with its own
//        `count` field — that's why two counters are independent. The
//        closure type is the same, but each instance has its own state.
//
//   EventBus — the design choices in detail:
//
//     - `Vec<Box<dyn FnMut(&str)>>` because each registered handler has
//        its own anonymous closure type. Static dispatch (`impl FnMut`)
//        can't store heterogeneous closure types in one Vec, so we erase
//        them via `dyn` trait objects, and box each one to give it a
//        stable address on the heap.
//
//     - `register<F: FnMut(&str) + 'static>` keeps the public API ergonomic
//        — callers pass a bare closure and we box it for them. The
//        `'static` bound means "the handler does not borrow anything that
//        could outlive the bus," which is what you want for a long-lived
//        registry. (Lifetimes proper are chapter 17.)
//
//     - `fire(&mut self, event: &str)` takes `&mut self` because calling
//        an `FnMut` requires mutable access to it. Iterating with
//        `self.handlers.iter_mut()` gives us `&mut Box<dyn FnMut(&str)>`,
//        and `h(event)` auto-derefs and calls the boxed closure mutably.
//
// ALTERNATIVES:
//
//   1. make_counter could return a struct with a `next()` method instead
//      of a closure:
//
//          pub struct Counter { count: u32 }
//          impl Counter { pub fn next(&mut self) -> u32 { self.count += 1; self.count } }
//
//      Same behavior; the closure form is shorter and demonstrates that a
//      closure is essentially "a struct with a single method" in disguise.
//
//   2. EventBus could use `Vec<Box<dyn Fn(&str)>>` (note: Fn, not FnMut)
//      if you want to forbid handlers from mutating captures. Then
//      `fire(&self, ...)` would only need a shared reference. But you'd
//      lose the (very common) "handler that bumps a counter" pattern,
//      which is why FnMut is the right default for an event bus.
//
//   3. For multi-threaded buses, you'd add `+ Send + Sync` bounds so
//      handlers could be shared across threads safely.
//
//   4. A handler-removal API typically returns a token (e.g., a usize)
//      from `register` and accepts it back in an `unregister` method.
//      Out of scope here.
