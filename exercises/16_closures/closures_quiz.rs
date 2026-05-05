// =============================================================================
//  closures_quiz — capstone: an event-handler registry + a counter factory
// =============================================================================
//
// Two small problems that combine almost everything from this chapter:
// returning closures, capturing state, mutating that state, choosing the
// right closure trait, storing closures in a collection.
//
// PART A — make_counter()
//
// Build a "counter factory": a function that returns a closure. Each call
// to the closure increments an internal counter and returns the new value:
//
//     let mut next = make_counter();
//     assert_eq!(next(), 1);
//     assert_eq!(next(), 2);
//     assert_eq!(next(), 3);
//
// Two factories must be INDEPENDENT — each closure has its own count:
//
//     let mut a = make_counter();
//     let mut b = make_counter();
//     a(); a(); b();
//     // a's next will be 3, b's next will be 2
//
// Hints:
//   - The closure mutates internal state, so it's `FnMut`, not `Fn`.
//   - The state is born inside `make_counter` and must outlive that
//     function frame — use `move`.
//   - Return type: `impl FnMut() -> u32`.
//
// PART B — EventBus
//
// A tiny event registry that stores a list of handlers and lets you
// trigger them by firing an event name (a `&str` payload).
//
// Handlers can mutate captured state (e.g., a counter that records how
// many events arrived), so each handler is an `FnMut(&str)`. They have
// different concrete types, so they must be stored as trait objects:
//
//     pub struct EventBus {
//         handlers: Vec<Box<dyn FnMut(&str)>>,
//     }
//
// API to implement:
//
//     impl EventBus {
//         pub fn new() -> Self { ... }
//         pub fn register<F>(&mut self, handler: F) where F: FnMut(&str) + 'static { ... }
//         pub fn fire(&mut self, event: &str) { ... }
//     }
//
// `register` is generic over `F` so callers can pass any closure; we box
// it inside. The `'static` bound says the handler doesn't borrow anything
// shorter-lived than the bus — pragmatic for an event registry. (Don't
// worry about lifetimes here, just paste the bound; chapter 17 explains.)
//
// `fire` calls every registered handler with the event string in order.

// I AM NOT DONE

pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0u32;
    ???
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
        self.handlers.push(???);
    }

    pub fn fire(&mut self, event: &str) {
        for h in self.handlers.iter_mut() {
            ???
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn counter_starts_at_one() {
        let mut next = make_counter();
        assert_eq!(next(), 1);
    }

    #[test] fn counter_increments() {
        let mut next = make_counter();
        assert_eq!(next(), 1);
        assert_eq!(next(), 2);
        assert_eq!(next(), 3);
        assert_eq!(next(), 4);
    }

    #[test] fn counters_are_independent() {
        let mut a = make_counter();
        let mut b = make_counter();
        a(); a(); a();
        b();
        assert_eq!(a(), 4);
        assert_eq!(b(), 2);
    }

    #[test] fn bus_fires_each_handler() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let mut bus = EventBus::new();

        let log1 = Rc::clone(&log);
        bus.register(move |event| log1.borrow_mut().push(format!("A:{event}")));

        let log2 = Rc::clone(&log);
        bus.register(move |event| log2.borrow_mut().push(format!("B:{event}")));

        bus.fire("click");
        bus.fire("hover");

        assert_eq!(
            *log.borrow(),
            vec!["A:click", "B:click", "A:hover", "B:hover"]
        );
    }

    #[test] fn bus_handler_can_mutate_capture() {
        // A handler that captures a Cell<u32> and bumps it on each event.
        use std::cell::Cell;
        use std::rc::Rc;

        let count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let mut bus = EventBus::new();
        let count_for_handler = Rc::clone(&count);
        bus.register(move |_event| {
            count_for_handler.set(count_for_handler.get() + 1);
        });

        bus.fire("a");
        bus.fire("b");
        bus.fire("c");

        assert_eq!(count.get(), 3);
    }

    #[test] fn bus_with_no_handlers_is_a_noop() {
        let mut bus = EventBus::new();
        bus.fire("nothing-listening"); // should not panic
    }
}

fn main() {}
