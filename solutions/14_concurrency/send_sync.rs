// SOLUTION — send_sync

use std::sync::Arc;
use std::thread;

pub fn run() {
    let r = Arc::new(7_i32);

    fn assert_send<T: Send + 'static>(_: T) {}
    let r2 = Arc::clone(&r);
    assert_send(r2);

    let r3 = Arc::clone(&r);
    let handle = thread::spawn(move || *r3);
    let v = handle.join().unwrap();
    assert_eq!(v, 7);
}

fn main() {
    run();
}

// WHY THIS IS OPTIMAL:
//
//   The whole exercise hinges on a single distinction: `Rc<T>`'s
//   refcount uses ordinary integer ops, while `Arc<T>`'s uses atomic
//   instructions. The first is faster on a single thread; the second
//   is the only one safe to send between threads. The compiler
//   encodes that fact in the type system:
//
//       Rc<T>  is !Send + !Sync
//       Arc<T> is  Send +  Sync   (when T: Send + Sync)
//
//   `thread::spawn` requires its closure to be `Send + 'static`, and
//   any captured `Rc<T>` makes the closure `!Send` because Rc itself
//   is `!Send`. The fix here is purely the import — swap
//   `std::rc::Rc` for `std::sync::Arc` and the rest of the code
//   already does the right thing (clones the handle per use).
//
//   `assert_send::<T>` is a zero-cost compile-time witness. If we
//   tried to feed it an `Rc<i32>`, the build would fail with the
//   same E0277 you'd see at the `thread::spawn` call. Putting it
//   above the spawn makes the type-system reason for the rejection
//   crisper than the longer error from `spawn`.
//
// THE BIG PICTURE:
//
//   You will basically never WRITE `impl Send for ...` or `impl Sync
//   for ...` yourself. Both are auto-traits — the compiler derives
//   them for any type whose fields are all Send / Sync. The cases
//   where you might `unsafe impl Send` are when you've built a
//   custom synchronisation primitive on top of raw pointers. For
//   ordinary day-to-day code, the rule is just:
//
//     - threads + sharing data?  -> Arc<T>
//     - threads + sharing mutation? -> Arc<Mutex<T>>
//     - single-threaded sharing? -> Rc<T> or Rc<RefCell<T>>
//
//   The compiler will tell you the moment you cross the line.
//
// ALTERNATIVES:
//
//   - For `Send + Sync` primitives, use `std::sync::atomic` types
//     (`AtomicUsize`, `AtomicBool`, ...). They're lock-free and the
//     refcount inside `Arc` is itself implemented this way.
//
//   - Some types are `Send` but `!Sync` — `Cell<T>` and `RefCell<T>`.
//     You can MOVE them to a thread, just not SHARE a `&` of one
//     across threads. `Mutex<T>` is the thread-safe analogue of
//     `RefCell` and is `Sync`.
//
//   - `*mut T` / `*const T` are `!Send + !Sync`; if you genuinely
//     need to send raw pointers across threads, wrap them in a
//     newtype and write `unsafe impl Send`. That's a deliberate "I
//     promise this is safe" — the compiler can no longer help you.
