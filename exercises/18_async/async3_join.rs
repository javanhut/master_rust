// =============================================================================
//  async3_join — running two futures CONCURRENTLY in one task
// =============================================================================
//
// Async Rust gives you concurrency without parallelism. One thread can
// drive thousands of futures by interleaving their `poll` calls — when
// one future is `Pending`, the executor moves on and polls another.
//
// Concurrency vs parallelism (the slogan, restated for async):
//
//   - PARALLEL   — two futures making progress AT THE SAME INSTANT, on
//                  different OS threads. Requires a multi-threaded runtime.
//   - CONCURRENT — two futures both alive and being polled, but only one
//                  executes at any one time. Single thread is enough.
//
// `tokio::join!`, `futures::join!`, and `futures::future::join` all
// produce a single future that POLLS its component futures and only
// reports `Ready` when ALL of them are. We provide a tiny version,
// `join2(a, b)`, in `mod runtime` below — read its body if you're
// curious; you only need to USE it here.
//
// HOW IT WORKS, IN ONE LINE
//
//   `join2` polls future A. If Ready, stash the value. Polls future B.
//   If Ready, stash. When both are stashed, return the tuple. Otherwise
//   return Pending so the executor will poll us again later.
//
// WHY ALTERNATING POLLS BUYS ANYTHING
//
// In our spinning executor we don't really gain wall-clock time —
// neither future is doing real I/O. But the SHAPE is identical to what
// you'd write with tokio: `join!` lets you fire off two requests and
// wait for both, instead of doing them sequentially with two `.await`s.
//
//   sequential:  let a = req1().await; let b = req2().await; (a + b)
//                ▶━━━━━━━━━▶━━━━━━━━━━━▶                    total ≈ 2T
//
//   concurrent:  let (a, b) = join2(req1(), req2()).await;
//                ▶━━━━━━━━━━━━━━━━━━━━━▶                    total ≈ T
//                ▶━━━━━━━━━━━━━━━━━━━━━▶
//
// We also provide `yield_once()` — a trivially-suspending future that
// returns Pending exactly once before becoming Ready. We use it to
// FORCE the join machinery to interleave its polls (otherwise both
// futures would complete in their first poll and the test would not
// observe any concurrency).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - In `fetch_a()`, write an async fn that calls `yield_once().await`
//     and then returns the &'static str  "alpha".
//   - In `fetch_b()`, do the same but return  "beta".
//   - In `both()`, use `runtime::join2(fetch_a(), fetch_b()).await` to
//     drive both concurrently, returning the resulting tuple.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

async fn fetch_a() -> &'static str {
    runtime::yield_once().???;
    ???
}

async fn fetch_b() -> &'static str {
    runtime::yield_once().???;
    ???
}

async fn both() -> (&'static str, &'static str) {
    runtime::join2(???, ???).???
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

    // A future that returns Pending the first time it's polled and
    // Ready the second time. Lets us force a real suspension point
    // without any external event source.
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

    // Hand-rolled equivalent of `futures::future::join(a, b)`. Polls
    // each child future until both are Ready, then returns the pair.
    pub async fn join2<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
        struct Join2<A: Future, B: Future> {
            a: Option<Pin<Box<A>>>,
            b: Option<Pin<Box<B>>>,
            a_out: Option<A::Output>,
            b_out: Option<B::Output>,
        }
        impl<A: Future, B: Future> Unpin for Join2<A, B> {}
        impl<A: Future, B: Future> Future for Join2<A, B> {
            type Output = (A::Output, B::Output);
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let this = self.as_mut().get_mut();
                if let Some(fa) = this.a.as_mut() {
                    if let Poll::Ready(v) = fa.as_mut().poll(cx) {
                        this.a_out = Some(v);
                        this.a = None;
                    }
                }
                if let Some(fb) = this.b.as_mut() {
                    if let Poll::Ready(v) = fb.as_mut().poll(cx) {
                        this.b_out = Some(v);
                        this.b = None;
                    }
                }
                if this.a_out.is_some() && this.b_out.is_some() {
                    Poll::Ready((this.a_out.take().unwrap(), this.b_out.take().unwrap()))
                } else {
                    Poll::Pending
                }
            }
        }
        Join2 {
            a: Some(Box::pin(a)),
            b: Some(Box::pin(b)),
            a_out: None,
            b_out: None,
        }
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn fetch_a_alone() {
        assert_eq!(runtime::block_on(fetch_a()), "alpha");
    }

    #[test] fn fetch_b_alone() {
        assert_eq!(runtime::block_on(fetch_b()), "beta");
    }

    #[test] fn join_returns_both() {
        assert_eq!(runtime::block_on(both()), ("alpha", "beta"));
    }
}
