// =============================================================================
//  rc_refcell — Rc<RefCell<T>>: many owners, all of whom can mutate
// =============================================================================
//
// `Rc<T>` gives multiple owners but only `&T` access.
// `RefCell<T>` gives `&mut`-through-shared-reference but only ONE owner.
// Compose them and you get the canonical pattern for shared,
// mutable, single-threaded state:
//
//     Rc<RefCell<T>>
//
// Every owner has an `Rc`; through the `Rc` they reach the `RefCell`;
// through the `RefCell` they `borrow_mut()` to mutate. The Rc handles
// SHARED OWNERSHIP, the RefCell handles MUTATION SAFETY.
//
//     use std::rc::Rc;
//     use std::cell::RefCell;
//
//     let shared = Rc::new(RefCell::new(0));
//     let a = Rc::clone(&shared);
//     let b = Rc::clone(&shared);
//
//     *a.borrow_mut() += 1;   // bumps through a
//     *b.borrow_mut() += 1;   // bumps through b
//     assert_eq!(*shared.borrow(), 2);
//
// READING THE EXPRESSION
//
//     *a.borrow_mut() += 1;
//     // ──┬──┘ ─┬──────┘ ─┬─
//     //   │     │         add-assign
//     //   │     temporary RefMut<i32> — the guard
//     //   deref the RefMut to access the inner i32
//
// PATTERNS THAT NEED THIS
//
//   - Two structs that each hold an `Rc<RefCell<Counter>>` and both want
//     to bump it. (This exercise.)
//   - A graph node referenced by multiple parents/children where any
//     can mutate it. (smart_quiz.)
//   - An observer that registers callbacks holding handles to shared
//     state. (We did a flavor of this in closures_quiz.)
//
// COSTS
//
//   - One heap allocation per `Rc::new` (the control block + T).
//   - One non-atomic refcount bump per `Rc::clone`.
//   - Two counter pokes per borrow / borrow_mut (and a panic check).
//
// In single-threaded code these are tiny. They're real, though, so don't
// reach for `Rc<RefCell<T>>` when a plain `&mut T` would do — keep the
// pattern for cases where ownership is genuinely shared.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Build a tiny example with two `Bumper`s sharing one counter.
//
//   - `Bumper` holds an `Rc<RefCell<i32>>`.
//   - `Bumper::new(shared)` constructs one from a clone of the shared cell.
//   - `Bumper::bump(&self)` adds 1 to the shared count.
//   - `Bumper::value(&self)` returns the current shared value.

// I AM NOT DONE

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bumper {
    shared: Rc<RefCell<i32>>,
}

impl Bumper {
    pub fn new(shared: ???) -> Self {
        Bumper { shared }
    }

    pub fn bump(&self) {
        *self.shared.???() += 1;
    }

    pub fn value(&self) -> i32 {
        *self.shared.???()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn one_bumper_works() {
        let cell = Rc::new(RefCell::new(0));
        let a = Bumper::new(Rc::clone(&cell));
        a.bump();
        a.bump();
        assert_eq!(a.value(), 2);
        assert_eq!(*cell.borrow(), 2);
    }

    #[test] fn two_bumpers_share_state() {
        let cell = Rc::new(RefCell::new(0));
        let a = Bumper::new(Rc::clone(&cell));
        let b = Bumper::new(Rc::clone(&cell));
        a.bump();
        b.bump();
        b.bump();
        // Both should see the same total — they share one counter.
        assert_eq!(a.value(), 3);
        assert_eq!(b.value(), 3);
        assert_eq!(*cell.borrow(), 3);
    }

    #[test] fn strong_count_reflects_owners() {
        let cell = Rc::new(RefCell::new(0));
        assert_eq!(Rc::strong_count(&cell), 1);
        let _a = Bumper::new(Rc::clone(&cell));
        assert_eq!(Rc::strong_count(&cell), 2);
        let _b = Bumper::new(Rc::clone(&cell));
        assert_eq!(Rc::strong_count(&cell), 3);
    }

    #[test] fn dropping_a_bumper_releases_one_owner() {
        let cell = Rc::new(RefCell::new(0));
        let a = Bumper::new(Rc::clone(&cell));
        let b = Bumper::new(Rc::clone(&cell));
        assert_eq!(Rc::strong_count(&cell), 3);
        drop(a);
        assert_eq!(Rc::strong_count(&cell), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&cell), 1);
    }
}

fn main() {}
