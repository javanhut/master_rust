// =============================================================================
//  send_sync — Send and Sync, the auto-traits behind thread safety
// =============================================================================
//
// Two compiler-checked marker traits gate everything in chapter 14:
//
//     Send    "values of this type can be MOVED to another thread"
//     Sync    "values of this type can be SHARED (&T) across threads"
//
// They are AUTO-TRAITS: the compiler implements them automatically for
// any type whose fields are all Send / Sync. You almost never write
// `impl Send` yourself.
//
// The relationship between them:
//
//     T: Sync   <=>   &T: Send
//
// In English: "you can share a `&T` between threads" is the same as
// "the reference type `&T` is itself sendable across threads."
//
// THE BOUNDARIES `thread::spawn` ENFORCES
//
//     pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static,
//
// Whatever the closure CAPTURES must be Send (it's about to move to
// another thread). Whatever the closure RETURNS must be Send (it
// crosses back through `join`).
//
// WHO IS / ISN'T
//
//     Send + Sync       i32, String, Vec<T> (when T is Send+Sync),
//                        Arc<T> (when T is Send+Sync), Mutex<T>
//                        (when T is Send), most types you'll write.
//
//     Send,  not Sync   Cell<T>, RefCell<T> — interior mutability
//                        without locking; sharing a reference would
//                        permit data races.
//
//     not Send, not Sync Rc<T> — its refcount is non-atomic. Cloning
//                         from two threads simultaneously would race
//                         on the counter and corrupt it. The compiler
//                         catches this at compile time.
//
//                        *mut T, *const T (raw pointers).
//
// THE CANONICAL ERROR
//
//     use std::rc::Rc;
//     let r = Rc::new(7);
//     thread::spawn(move || println!("{}", r));
//     // ^^^ error[E0277]: `Rc<i32>` cannot be sent between threads safely
//     //     the trait `Send` is not implemented for `Rc<i32>`
//
// The fix is to switch to `Arc`, which does the refcount math with
// atomic instructions and IS `Send + Sync`:
//
//     use std::sync::Arc;
//     let r = Arc::new(7);
//     thread::spawn(move || println!("{}", r));   // compiles
//
// =============================================================================
//  YOUR TASK (compile-mode exercise)
// =============================================================================
// The file below tries to send an `Rc<i32>` to another thread. As
// written it FAILS to compile with the E0277 error above. Fix it by
// swapping `Rc` for `Arc` (two edits — the import and the
// constructor). The body is otherwise unchanged.
//
// This file is in `compile` mode: passing it means the program builds
// (and runs to completion). There is no test module.

// I AM NOT DONE

use std::rc::Rc;          // <-- swap this import
use std::thread;

pub fn run() {
    // `Rc<i32>` is `!Send`, so the closure capturing it cannot be
    // moved to another thread. Switching to `Arc` fixes both this
    // line and `r` below.
    let r = Rc::new(7_i32);

    // The compile-time witness lives here: this function only
    // accepts a value of a `Send + 'static` type. Right now `Rc<i32>`
    // doesn't satisfy that bound, so this call fails to compile.
    fn assert_send<T: Send + 'static>(_: T) {}
    let r2 = r.clone();
    assert_send(r2);

    // And the canonical "send across threads" smoke test:
    let r3 = r.clone();
    let handle = thread::spawn(move || {
        // `r3` arrives in the new thread fully formed.
        *r3
    });
    let v = handle.join().unwrap();
    assert_eq!(v, 7);
}

fn main() {
    run();
}
