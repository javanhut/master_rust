// SOLUTION — rc_refcell

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bumper {
    shared: Rc<RefCell<i32>>,
}

impl Bumper {
    pub fn new(shared: Rc<RefCell<i32>>) -> Self {
        Bumper { shared }
    }

    pub fn bump(&self) {
        *self.shared.borrow_mut() += 1;
    }

    pub fn value(&self) -> i32 {
        *self.shared.borrow()
    }
}

// WHY THIS IS OPTIMAL:
//
//   The struct STORES `Rc<RefCell<i32>>` directly — not `&Rc<RefCell<i32>>`
//   or `Rc<&RefCell<i32>>`. Storing the owned Rc means each Bumper is an
//   independent owner of the shared cell; cloning the Rc on the way in
//   is the caller's responsibility (and is the standard convention).
//
//   `Bumper::new` takes the Rc by value — receiving an already-cloned
//   handle. This makes the cloning EXPLICIT at every call site:
//
//       let a = Bumper::new(Rc::clone(&cell));
//       let b = Bumper::new(Rc::clone(&cell));
//
//   That's the same pattern you saw in EventBus's tests in chapter 16.
//
//   `bump(&self)` works through a SHARED reference because the mutation
//   goes through the RefCell. The `Rc` part doesn't even need to bump
//   its counter for a borrow — `Rc::deref` is just a pointer load.
//
//   `value(&self)` returns a copy of the i32 by dereferencing the Ref
//   guard. The guard drops at the semicolon, so no borrow lingers.
//
// ANATOMY OF *self.shared.borrow_mut() += 1
//
//   1. `self.shared`           — &Rc<RefCell<i32>>
//   2. `.borrow_mut()`         — auto-derefs Rc -> RefCell, calls borrow_mut(),
//                                 returns RefMut<'_, i32> guard
//   3. `*<guard>`              — derefs to &mut i32
//   4. `+= 1`                  — actual mutation
//   5. (end of statement)      — guard drops, runtime borrow released
//
// ALTERNATIVES:
//
//   - For thread-safe sharing, swap to `Arc<Mutex<i32>>`. Same shape,
//     atomic refcount + blocking lock instead of non-atomic count + panicking
//     borrow. Chapter 14 covers this.
//
//   - If the inner type is `Copy`, `Rc<Cell<T>>` is slightly cheaper —
//     `Cell` has no guards, so it can't panic. `Cell<i32>` would work
//     here too:
//
//          self.shared.set(self.shared.get() + 1);
//
//     We chose `RefCell` for didactic continuity — it's the form you'll
//     see in graph/tree code (next exercise) where the inner type isn't Copy.
//
//   - If multiple bumpers all live in one place and ownership is local,
//     plain `&mut Counter` is simpler. Reach for `Rc<RefCell<_>>` only
//     when distinct owners need to outlive each other arbitrarily.
