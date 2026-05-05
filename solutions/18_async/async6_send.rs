// SOLUTION — async6_send

use std::sync::Arc;

async fn task() {
    let shared: Arc<u32> = Arc::new(7);
    runtime::yield_once().await;
    let _ = *shared;
}

fn main() {
    runtime::spawn(task());
}

// WHY THIS IS OPTIMAL:
//
//   The fix is a one-line type swap: `Rc` → `Arc`. The API is identical
//   (`Arc::new`, `Arc::clone`, deref to `&T`), but `Arc` uses atomic
//   refcount operations and therefore implements `Send + Sync`. That
//   makes the generated state machine Send too, satisfying the bound
//   on `runtime::spawn` (and on `tokio::spawn` in real code).
//
// WHAT THE COMPILER WAS COMPLAINING ABOUT:
//
//   The original error reads roughly:
//
//     error: future cannot be sent between threads safely
//      ┌─ within `impl Future`,
//      │   the trait `Send` is not implemented for `Rc<u32>`
//      │
//   note: future is not `Send` as this value is used across an await
//   note: required by a bound on `runtime::spawn`
//
//   The key phrase is "value is used across an await." If `shared` had
//   been DROPPED before the `.await`, the future would still be Send —
//   the issue is specifically that the captured Rc lives across a
//   suspension point, so it ends up as a field of the state machine.
//
// MENTAL CHECKLIST FOR "FUTURE NOT SEND" ERRORS:
//
//   1. Find the value the compiler points at.
//   2. Is it held across an `.await`? If you can drop it earlier, do so:
//        let v = compute_thing();
//        let r = use_thing(&v);          // no await between v and r
//        drop(v);
//        do_async_work().await;
//      That keeps `v` out of the state machine.
//   3. If you NEED it across the await, swap to a Send equivalent:
//        Rc → Arc
//        RefCell → Mutex (or RwLock)
//        Cell → Atomic*
//        raw pointer → ...usually wrong, rethink the design
//   4. If the value is fundamentally non-Send (e.g., a !Send GUI handle),
//      run the future on `tokio::task::spawn_local` inside a `LocalSet`
//      — that's tokio's escape hatch for single-thread-pinned futures.
//
// ALTERNATIVES:
//
//   1. `tokio::task::spawn_local(task())` would accept the Rc version
//      because spawn_local doesn't require Send — the task is pinned
//      to the current thread. Useful when you really do need !Send
//      state. Costs you the option of cross-thread load balancing.
//
//   2. If `shared` was for read-only sharing of plain data, sometimes
//      the simplest fix is "don't share at all": clone the inner T
//      into the future before the await. Trades a clone for the
//      bound — usually a great trade for small Copy/Clone data.

mod runtime {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    pub fn spawn<F>(_fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
    }

    pub fn yield_once() -> YieldOnce {
        YieldOnce { yielded: false }
    }
    pub struct YieldOnce { yielded: bool }
    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                Poll::Pending
            }
        }
    }
}
