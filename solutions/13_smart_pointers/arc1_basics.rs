// SOLUTION — arc1_basics

use std::sync::Arc;

pub fn make_shared(n: i32) -> Arc<i32> {
    Arc::new(n)
}

pub fn share(arc: &Arc<i32>) -> Arc<i32> {
    Arc::clone(arc)
}

pub fn count(arc: &Arc<i32>) -> usize {
    Arc::strong_count(arc)
}

pub fn read(arc: &Arc<i32>) -> i32 {
    **arc
    // ^ &Arc<i32> -> Arc<i32> (deref &) -> i32 (Arc::deref returns &i32, then *).
    //   You could also write `*arc.as_ref()` or simply rely on Copy: `*&**arc`.
    //   The simplest spelling is `**arc`.
}

// WHY THIS IS OPTIMAL:
//
//   Same shape as `Rc`, but every refcount operation goes through
//   atomic instructions (`fetch_add`, `fetch_sub`) so concurrent
//   clones and drops are race-free. The public API is intentionally
//   identical so swapping `Rc` for `Arc` (or vice versa) is mostly a
//   find-and-replace.
//
//   `**arc` strips both the outer `&` and the inner `Arc` deref to
//   reach the `i32`, which is `Copy`. We could equivalently write
//   `*Arc::as_ref(arc)` — same machine code.
//
//   `assert_send_sync::<Arc<i32>>()` is a zero-cost compile-time
//   check. If you swapped in `Rc<i32>` it would fail to compile,
//   demonstrating that the difference between `Rc` and `Arc` lives in
//   the type system, not in runtime behavior alone.
//
// THE COST OF "ATOMIC"
//
//   On x86, `Arc::clone` is roughly a `lock xadd` on the strong count —
//   a few nanoseconds, fully fenced across cores. Uncontended, you'll
//   never notice. Contended (many threads bumping the same counter
//   simultaneously), the cache line bounces and throughput drops.
//   Profile before worrying. For most apps, just use `Arc<T>` whenever
//   threads might be involved and move on.
//
// MUTATION (preview, chapter 14):
//
//   `Arc<T>` alone is read-only. To mutate across threads, use
//
//       Arc<Mutex<T>>           // exclusive locking
//       Arc<RwLock<T>>          // many-readers / one-writer
//       Arc<AtomicUsize>        // for primitives, lock-free
//
//   `Mutex::lock()` returns a `MutexGuard<T>` you can deref to `&mut T`,
//   blocking other threads until the guard drops. It's the natural
//   thread-safe analogue of `RefCell::borrow_mut` — but blocks instead
//   of panicking on contention.
//
// ALTERNATIVES:
//
//   - `Arc::try_unwrap(arc)` returns the inner T if you're the sole
//     owner, otherwise gives the Arc back. Useful for "consume when
//     no one else is looking".
//
//   - `Arc::get_mut(&mut arc)` returns `Option<&mut T>` — Some when the
//     count is 1, None otherwise. Lets you mutate without locks when
//     you can prove uniqueness.
//
//   - `Arc::make_mut(&mut arc)` is "clone-on-write": clones the inner
//     T if there are other owners, then returns `&mut T`. Very handy
//     for persistent-data-structure patterns.
