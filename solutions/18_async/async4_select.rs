// SOLUTION — async4_select

async fn slow() -> i32 {
    runtime::yield_once().await;
    runtime::yield_once().await;
    100
}

async fn fast() -> &'static str {
    runtime::yield_once().await;
    "fast"
}

async fn race() -> &'static str {
    match runtime::select2(slow(), fast()).await {
        runtime::Either::Left(_n) => "slow won (unexpected)",
        runtime::Either::Right(s) => s,
    }
}

fn main() {
    let winner = runtime::block_on(race());
    println!("winner: {winner}");
}

// WHY THIS IS OPTIMAL:
//
//   `slow` yields twice before producing its value. `fast` yields once.
//   In our spinning executor each yield costs one extra `poll`, so on
//   the second poll of `select2`, `fast` is Ready while `slow` still
//   needs another round-trip — Right wins.
//
//   The match against `Either<i32, &'static str>` is the natural way to
//   handle "I don't know which side will win." In `tokio::select!` the
//   same shape is written as multiple branch arms inside the macro, and
//   the macro picks whichever branch's pattern fires first.
//
// CANCELLATION IS DROP:
//
//   When `select2` returns `Either::Right(...)`, the `Select2` future
//   is dropped, which drops its `a: Pin<Box<A>>` field, which drops the
//   `slow()` state machine. Anything `slow` was holding (heap data,
//   open files, pending sockets) gets cleaned up by the regular Drop
//   chain. There is NO "kill the task" call — cancellation in async
//   Rust is simply "stop polling the future and drop it."
//
//   This is why holding non-droppable side effects across `.await` (an
//   uncommitted DB transaction, say) needs extra care: a select-loser
//   could be cancelled MID-transaction and your code never gets to run
//   the commit/rollback. The standard workaround is RAII guards or
//   `tokio::select! { biased; }` + explicit cancellation tokens.
//
// ALTERNATIVES:
//
//   1. With tokio:
//
//        let winner = tokio::select! {
//            n = slow() => Either::Left(n),
//            s = fast() => Either::Right(s),
//        };
//
//      Same outcome, more arms supported, per-branch pattern matching.
//
//   2. `futures::future::select` returns a more elaborate type
//      (`Either<(A::Output, B), (B::Output, A)>`) that hands you back
//      the LOSING future as well, so you can `.await` it later if you
//      change your mind. Our `Either` drops the loser to keep the
//      lesson simple.
//
//   3. `select_all(Vec<F>)` for an arbitrary number of futures — same
//      shape, returns `(winner, index, remaining)`.

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
