// =============================================================================
//  refcell1 — RefCell<T>: interior mutability with RUNTIME borrow checking
// =============================================================================
//
// Rust's borrow rules are usually checked at COMPILE time:
//   - many `&T` OR exactly one `&mut T`,
//   - never both at once,
//   - reference can't outlive the borrowee.
//
// Most code is fine with that. Sometimes it isn't — you have a value
// behind a `&` (e.g. inside an `Rc`) but you genuinely need to mutate
// it. The compiler can't prove the borrow rules in such a case, so we
// move the check from COMPILE TIME to RUNTIME. That's interior
// mutability, and `RefCell<T>` is its workhorse for single-threaded
// code.
//
//     use std::cell::RefCell;
//
//     let cell = RefCell::new(5);
//     *cell.borrow_mut() += 1;
//     assert_eq!(*cell.borrow(), 6);
//
// THE API
//
//     cell.borrow()      -> Ref<'_, T>      (like &T, can have many)
//     cell.borrow_mut()  -> RefMut<'_, T>   (like &mut T, exclusive)
//     cell.try_borrow()  -> Result<Ref, _>  (no panic, returns Err if violated)
//     cell.try_borrow_mut() -> Result<RefMut, _>
//
// `Ref<'_, T>` and `RefMut<'_, T>` are GUARDS. While a guard is alive,
// the cell tracks the borrow in an internal counter. When the guard is
// dropped, the counter goes back down. If you ask for a `borrow_mut()`
// while ANY borrow is active, you get a PANIC at runtime.
//
//     let cell = RefCell::new(0);
//     let a = cell.borrow_mut();
//     let b = cell.borrow_mut();   // 💥 panics: already mutably borrowed
//
// The same rules as before — the difference is purely WHEN they're
// checked. You trade compile-time safety for runtime flexibility.
//
// WHY NOT JUST USE `&mut T` EVERYWHERE?
//
// Some patterns can't satisfy the static rules even though they're safe.
// Classic example: an observable counter that several callers want to
// increment via a shared reference. `Rc<RefCell<T>>` is the fix —
// covered next.
//
// SINGLE-THREADED ONLY
//
// `RefCell<T>` is `!Sync`. For threads use `Mutex<T>` (chapter 14).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `bump(cell)` increments the i32 inside through a shared reference.
//   - `read(cell)` returns a copy of the inner i32.
//   - `would_panic_on_double_mut(cell)` deliberately holds two `borrow_mut`
//     at once and returns whether `try_borrow_mut` saw the conflict.

// I AM NOT DONE

use std::cell::RefCell;

pub fn bump(cell: &RefCell<i32>) {
    // Note the parameter is `&RefCell<i32>` — a SHARED reference — yet
    // we're about to mutate. That's the magic of interior mutability.
    *cell.???() += 1;
}

pub fn read(cell: &RefCell<i32>) -> i32 {
    *cell.???()
}

pub fn would_panic_on_double_mut(cell: &RefCell<i32>) -> bool {
    let _first = cell.borrow_mut();
    // Now ask politely. `try_borrow_mut` returns Err instead of panicking
    // when there's already an active borrow.
    cell.try_borrow_mut().???()
    // ^ returns true if Err (i.e. the second borrow was rejected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn bump_increments() {
        let cell = RefCell::new(10);
        bump(&cell);
        bump(&cell);
        assert_eq!(read(&cell), 12);
    }

    #[test] fn many_immutable_borrows_ok() {
        let cell = RefCell::new(7);
        let a = cell.borrow();
        let b = cell.borrow();
        let c = cell.borrow();
        assert_eq!(*a + *b + *c, 21);
    }

    #[test] fn double_mut_borrow_is_detected() {
        let cell = RefCell::new(0);
        assert!(would_panic_on_double_mut(&cell));
    }

    #[test] fn actual_double_mut_borrow_panics() {
        // Demonstrate the panic for real, then catch it so the test passes.
        use std::panic::{self, AssertUnwindSafe};
        let cell = RefCell::new(0);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let _a = cell.borrow_mut();
            let _b = cell.borrow_mut();   // panics here
        }));
        assert!(result.is_err());
    }

    #[test] fn release_then_reborrow_is_fine() {
        let cell = RefCell::new(0);
        {
            let mut g = cell.borrow_mut();
            *g = 1;
        } // g drops here, counter goes back down
        let mut g = cell.borrow_mut();
        *g = 2;
        drop(g);
        assert_eq!(read(&cell), 2);
    }
}

fn main() {}
