// SOLUTION — refcell1_basics

use std::cell::RefCell;

pub fn bump(cell: &RefCell<i32>) {
    *cell.borrow_mut() += 1;
}

pub fn read(cell: &RefCell<i32>) -> i32 {
    *cell.borrow()
}

pub fn would_panic_on_double_mut(cell: &RefCell<i32>) -> bool {
    let _first = cell.borrow_mut();
    cell.try_borrow_mut().is_err()
}

// WHY THIS IS OPTIMAL:
//
//   `bump` is the heart of why `RefCell` exists. The parameter is
//   `&RefCell<i32>` — a SHARED reference — yet inside we mutate. That
//   would be impossible with a plain `&i32`: shared references are
//   read-only by Rust's static rules. `RefCell::borrow_mut()` performs
//   the mutability check at RUNTIME by inspecting an internal counter,
//   panicking if any other borrow is currently outstanding.
//
//   `read` is the trivial dual: ask for an immutable guard (`Ref<i32>`),
//   dereference it to copy the i32 out.
//
//   `would_panic_on_double_mut` shows the safe-probe form of the same
//   check. `try_borrow_mut()` returns `Result<RefMut<T>, BorrowMutError>`
//   instead of panicking, so production code that genuinely might
//   contend can recover gracefully.
//
//   The deliberate-panic test uses `AssertUnwindSafe` because `RefCell`
//   is `!UnwindSafe` (a panic mid-borrow could leave the borrow counter
//   inconsistent — but the closure here doesn't survive the panic, so
//   we assert the contract is upheld).
//
// MENTAL MODEL — "RUNTIME `&` / `&mut`"
//
//   Think of `RefCell` as moving Rust's borrow checker into runtime. The
//   rules don't change: many readers OR one writer, never both. What
//   changes is WHEN violations are caught — and what happens when they
//   are: a panic, not a compile error.
//
// ALTERNATIVES & WHEN TO REACH FOR EACH:
//
//   - `Cell<T>` for `Copy` types (or for moving values out and replacing
//     them) — has no borrow guards, so it's strictly safer (can't panic)
//     but only supports get/set, not "borrow and read in place".
//
//          let c = Cell::new(0);
//          c.set(c.get() + 1);
//
//   - `RefCell<T>` for any `T` when you need temporary `&T` / `&mut T`
//     access through a shared handle. The most common interior-mutability
//     primitive in single-threaded code.
//
//   - `Mutex<T>` / `RwLock<T>` for thread-safe interior mutability
//     (chapter 14). Same idea, but the "lock" can BLOCK rather than
//     panic on contention.
//
// PITFALLS:
//
//   - HOLD GUARDS BRIEFLY. Keeping a `RefMut` alive across a function
//     call that re-enters the cell is the #1 source of panics. Drop
//     guards as soon as you're done.
//
//   - DON'T sprinkle `RefCell` to "make the borrow checker happy." Most
//     of the time the right fix is to restructure the code. Reach for
//     interior mutability when you genuinely have shared, observable
//     mutable state — not as a workaround for a borrow you didn't think
//     through.
