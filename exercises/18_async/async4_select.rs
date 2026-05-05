// =============================================================================
//  async4_select — racing futures and taking the first to finish
// =============================================================================
//
// `join` waits for ALL of its children. `select` waits for ANY ONE of
// them — whoever crosses the finish line first wins, the rest are
// dropped. This is the canonical primitive for:
//
//   - timeouts:   `select(work, timer(5_secs))`  — bail out if work hangs.
//   - cancellation: `select(server, shutdown_signal)` — exit cleanly.
//   - first-of-N: `select(req_to_mirror_a, req_to_mirror_b)` — take the
//     fastest replica.
//
// The real `tokio::select!` macro is more capable than what we'll build
// (it can `select` over an arbitrary number of branches and run a
// different block per branch), but the SHAPE is the same.
//
// THE OUTPUT TYPE
//
// Two futures with possibly-different output types `A::Output` and
// `B::Output` produce an enum saying "A finished with this value" or
// "B finished with this value." We provide an `Either<A, B>` for that:
//
//     pub enum Either<A, B> { Left(A), Right(B) }
//
// `select2(a, b)` returns `Either<A::Output, B::Output>`.
//
// CANCELLATION IS IMPLICIT
//
// When `select2` returns Ready, the LOSING future is dropped — its
// state machine simply goes away. In real code that's how async
// cancellation works: drop the future, every resource it held (TCP
// socket, file handle, ongoing request) is cleaned up by `Drop` impls
// along the way. There is no "kill" syscall — just stop polling.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `slow()` — yields TWICE before returning the i32  `100`.
//   - `fast()` — yields ONCE before returning the &'static str  "fast".
//   - `race()` — uses `runtime::select2(slow(), fast())` and asserts
//     by returning whichever side won. Fast should win.
//
// Use `runtime::Either::{Left, Right}` to destructure the result.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

async fn slow() -> i32 {
    runtime::yield_once().???;
    runtime::yield_once().???;
    100
}

async fn fast() -> &'static str {
    runtime::yield_once().???;
    "fast"
}

// Returns "fast" if `fast` won, or a panic-string if `slow` somehow won.
async fn race() -> &'static str {
    match runtime::select2(slow(), fast()).??? {
        runtime::Either::Left(_n) => "slow won (unexpected)",
        runtime::Either::Right(s) => ???,
    }
}

fn main() {}

mod runtime {
    use core::future::Future;
    use core::pin::Pin;
    use core::ptr;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn noop_raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        static VTABLE: RawWakerVTable =
            RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(ptr::null(), &VTABLE)
    }

    pub fn block_on<F: Future>(mut fut: F) -> F::Output {
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut ctx = Context::from_waker(&waker);
        let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
        loop {
            match fut.as_mut().poll(&mut ctx) {
                Poll::Ready(v) => return v,
                Poll::Pending => continue,
            }
        }
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

    pub enum Either<A, B> { Left(A), Right(B) }

    // Hand-rolled equivalent of `futures::future::select`. Polls both
    // children on every poll. Whichever returns Ready first WINS — the
    // other is dropped (cancelled).
    pub async fn select2<A: Future, B: Future>(a: A, b: B) -> Either<A::Output, B::Output> {
        struct Select2<A: Future, B: Future> {
            a: Pin<Box<A>>,
            b: Pin<Box<B>>,
        }
        impl<A: Future, B: Future> Unpin for Select2<A, B> {}
        impl<A: Future, B: Future> Future for Select2<A, B> {
            type Output = Either<A::Output, B::Output>;
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let this = self.as_mut().get_mut();
                if let Poll::Ready(v) = this.a.as_mut().poll(cx) {
                    return Poll::Ready(Either::Left(v));
                }
                if let Poll::Ready(v) = this.b.as_mut().poll(cx) {
                    return Poll::Ready(Either::Right(v));
                }
                Poll::Pending
            }
        }
        Select2 { a: Box::pin(a), b: Box::pin(b) }.await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn fast_wins_the_race() {
        assert_eq!(runtime::block_on(race()), "fast");
    }

    #[test] fn select_left_can_win_too() {
        // When the LEFT future is the quicker one, Left should win.
        async fn quick() -> i32 { 7 }
        async fn slow_b() -> i32 {
            runtime::yield_once().await;
            runtime::yield_once().await;
            8
        }
        let r = runtime::block_on(runtime::select2(quick(), slow_b()));
        match r {
            runtime::Either::Left(n)  => assert_eq!(n, 7),
            runtime::Either::Right(_) => panic!("right should not have won"),
        }
    }
}
