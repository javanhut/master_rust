// SOLUTION — async3_join

async fn fetch_a() -> &'static str {
    runtime::yield_once().await;
    "alpha"
}

async fn fetch_b() -> &'static str {
    runtime::yield_once().await;
    "beta"
}

async fn both() -> (&'static str, &'static str) {
    runtime::join2(fetch_a(), fetch_b()).await
}

fn main() {
    let (a, b) = runtime::block_on(both());
    println!("{a} {b}");
}

// WHY THIS IS OPTIMAL:
//
//   `fetch_a` and `fetch_b` each `yield_once().await` to model "did
//   some async work, will resume on the next poll." The exact pattern
//   you'd see with a real network call: the future suspends until the
//   response arrives, then resumes and produces a value.
//
//   `both` is the lesson. Compare:
//
//     // sequential — runs B only after A finishes
//     let a = fetch_a().await;
//     let b = fetch_b().await;
//
//     // concurrent — both alive at once, polled in interleaving steps
//     let (a, b) = runtime::join2(fetch_a(), fetch_b()).await;
//
//   In tokio/futures, the same pattern is `join!(fetch_a(), fetch_b())`
//   (a macro, supports more than two) or `futures::future::join(a, b)`
//   (a function, two arguments). The semantics are identical to our
//   `join2`: alternate polls, return when both are done.
//
// HOW `join2` IS BUILT:
//
//   - It owns the two children behind `Pin<Box<F>>`. Boxing pins them
//     to the heap so we can hold them across `.await`s in the parent
//     state machine without worrying about pin-projection.
//   - On each poll: if a child future is still alive, poll it. If it
//     reports Ready, stash the value and drop the future.
//   - When BOTH outputs are stashed, return Ready with the tuple.
//
//   That's the whole story. A general `join_all` over a Vec of futures
//   uses the same pattern with a `Vec<Option<Pin<Box<F>>>>` and a
//   matching `Vec<Option<F::Output>>`.
//
// ALTERNATIVES:
//
//   1. With tokio:
//
//          let (a, b) = tokio::join!(fetch_a(), fetch_b());
//
//      Same semantics, no boxing — the macro generates a state machine
//      with the children inline as fields, which is a measurable
//      win in hot code. We Box here because writing pin-projection by
//      hand is far more code than the lesson is worth.
//
//   2. `tokio::try_join!` short-circuits on the first Err — handy when
//      both futures return `Result`. The non-short-circuiting form
//      always waits for both.
//
//   3. For "race them, take whichever finishes first" you want
//      `select`, not `join` — that's the next exercise.

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
