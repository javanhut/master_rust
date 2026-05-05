// =============================================================================
//  async2_runtime — what the executor is actually doing
// =============================================================================
//
// In `async1_intro` we used `runtime::block_on` as a black box. This
// exercise opens the box. Read the code in `mod runtime` carefully — it
// is the SHORTEST POSSIBLE Rust executor that drives a future to
// completion. Every real runtime (tokio, async-std, smol, embassy) is a
// supercharged version of this.
//
// THE FUTURE TRAIT
//
//   pub trait Future {
//       type Output;
//       fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//   }
//
// The executor is the thing that calls `poll`. It has to:
//   1. Construct a `Waker` to pass into `Context`. The waker is the
//      callback the future uses to say "I'm ready, poll me again."
//   2. Pin the future (futures are self-referential — if you move them
//      between polls, the internal pointers go bad).
//   3. Loop: call `poll`. If `Ready(v)`, return v. If `Pending`, wait
//      (in real life, park the thread until the waker fires).
//
// OUR `block_on` IS A SPINNING EXECUTOR
//
// Real runtimes block the thread on a parking primitive when poll
// returns Pending, and rely on the waker to unblock them. Ours doesn't —
// it just retries `poll` immediately. That's wasteful but FINE here
// because every future in this chapter eventually returns Ready without
// any external event (no timers, no sockets, no real I/O).
//
// THE NO-OP WAKER
//
// `Waker` has a vtable with four function pointers (clone / wake /
// wake_by_ref / drop). Ours does nothing for all four. Since we never
// actually wait on the waker, "doing nothing" is correct. A real waker
// would, e.g., flip a bit on the executor's "ready queue" and unpark a
// thread.
//
// PIN, BRIEFLY
//
// `async fn` blocks generate a state machine that may store references
// to its OWN local variables across `.await` points. If you moved that
// state machine in memory, those internal references would dangle. `Pin`
// is the type-level promise "I will not move this value again." Our
// `block_on` keeps the future on the stack and pins it via
// `Pin::new_unchecked` — we promise we won't move it because it's a
// local that nothing else touches.
//
// `core::pin::pin!(expr)` is the safe macro form: it pins a value to
// the current stack frame. We use it sparingly here.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Write `double(x)` as an async fn returning `x * 2`.
//   - Write `chained(x)` as an async fn that AWAITS `double(x)` twice
//     (so input 3 → 12).
//   - In `kick_off()`, build the `chained(5)` future and drive it with
//     `runtime::block_on`. Return the result.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

async fn double(x: i32) -> i32 {
    ???
}

async fn chained(x: i32) -> i32 {
    let a = double(x).???;
    let b = double(a).???;
    b
}

fn kick_off() -> i32 {
    runtime::block_on(???)
}

fn main() {}

mod runtime {
    use core::future::Future;
    use core::pin::Pin;
    use core::ptr;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    // The four-function vtable for our no-op waker.
    //   clone — return a fresh RawWaker (a real impl would Arc-bump a refcount)
    //   wake / wake_by_ref — signal the executor that this future is ready
    //   drop — release any resources held by the waker
    fn noop_raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        static VTABLE: RawWakerVTable =
            RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(ptr::null(), &VTABLE)
    }

    pub fn block_on<F: Future>(mut fut: F) -> F::Output {
        // 1. Build a Waker from the no-op vtable. SAFETY: the vtable is
        //    'static and the data pointer is null, which is allowed
        //    because none of our vtable functions dereference it.
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut ctx = Context::from_waker(&waker);

        // 2. Pin the future on this stack frame. SAFETY: `fut` lives as
        //    a local for the rest of this function and is not moved.
        let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

        // 3. Spin until the future reports Ready.
        loop {
            match fut.as_mut().poll(&mut ctx) {
                Poll::Ready(v) => return v,
                Poll::Pending => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn double_works() {
        assert_eq!(runtime::block_on(double(7)), 14);
    }

    #[test] fn chained_works() {
        assert_eq!(runtime::block_on(chained(3)), 12);
    }

    #[test] fn kick_off_works() {
        assert_eq!(kick_off(), 20);
    }
}
