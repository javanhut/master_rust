// SOLUTION — async_quiz

async fn fetch_alpha() -> &'static str {
    runtime::yield_once().await;
    "alpha"
}

async fn fetch_beta() -> &'static str {
    runtime::yield_once().await;
    "beta"
}

async fn fetch_gamma() -> &'static str {
    runtime::yield_once().await;
    "gamma"
}

async fn fetch_all() -> Vec<&'static str> {
    let futures: Vec<runtime::BoxFuture<&'static str>> = vec![
        Box::pin(fetch_alpha()),
        Box::pin(fetch_beta()),
        Box::pin(fetch_gamma()),
    ];
    runtime::join_all(futures).await
}

fn main() {
    let results = runtime::block_on(fetch_all());
    println!("results: {results:?}");
}

// WHY THIS IS OPTIMAL:
//
//   Each `fetch_*` is the simplest possible model of an async I/O call:
//   yield once (modelling "request goes out, response comes back"),
//   then return a value. In tokio you'd swap `yield_once().await` for
//   the real call:
//
//     let resp = client.get(URL).send().await?;
//     let body = resp.text().await?;
//
//   The shape is identical — `.await` on a future, get a value back.
//
//   `fetch_all` is the punchline of the chapter. The three boxed
//   futures go into one `Vec`, `join_all` polls them concurrently, and
//   the final `Vec<&'static str>` lands in the same submission order.
//   On a real multi-threaded runtime these three calls would happen
//   in parallel; here we get only the concurrency, but the SOURCE
//   CODE IS THE SAME. That's the win async Rust delivers: you write
//   one program, the runtime gives you whichever concurrency model is
//   available.
//
// WHY BOX-PIN THE FUTURES?
//
//   Each `async fn` has a UNIQUE anonymous return type — `fetch_alpha`'s
//   future and `fetch_beta`'s future are not the same type, even
//   though they share the same `Output`. To put them in a single Vec
//   you need a common type:
//
//     Pin<Box<dyn Future<Output = &'static str>>>     // BoxFuture<T>
//
//   The `Box::pin` allocates the future on the heap and pins it
//   there; `dyn Future` erases the per-async-fn type so they all fit
//   in the same `Vec` slot. That's the canonical "heterogeneous
//   collection of futures" pattern.
//
//   `join3!` in tokio avoids the boxing because it takes a fixed
//   number of futures by name (the macro builds a state machine with
//   the right concrete types inline). Use that when you have few,
//   known futures; use a `Vec<BoxFuture<_>>` when the count is
//   dynamic.
//
// ALTERNATIVES:
//
//   1. tokio:
//
//        let (a, b, c) = tokio::join!(fetch_alpha(), fetch_beta(), fetch_gamma());
//        let results = vec![a, b, c];
//
//      Cleaner for fixed-arity. No Box. No type erasure.
//
//   2. tokio + dynamic count:
//
//        let futures: Vec<_> = ids.iter().map(|i| fetch(i)).collect();
//        let results = futures::future::join_all(futures).await;
//
//      Same shape as our `join_all`. Note `join_all` accepts any
//      `IntoIterator<Item: Future>` in the real crate.
//
//   3. For "fail fast": `try_join!` / `try_join_all` short-circuit on
//      the first `Err` from `Result`-yielding futures. Correct
//      default for HTTP fetches that may individually fail.
//
//   4. For LIMITING concurrency (don't fire 10,000 requests at once):
//      `futures::stream::iter(...).buffer_unordered(N)` — a windowed
//      version of join_all that keeps at most N futures in flight.

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

    pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

    pub async fn join_all<T>(futs: Vec<BoxFuture<T>>) -> Vec<T> {
        struct JoinAll<T> {
            futs: Vec<Option<BoxFuture<T>>>,
            out: Vec<Option<T>>,
        }
        impl<T> Unpin for JoinAll<T> {}
        impl<T> Future for JoinAll<T> {
            type Output = Vec<T>;
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Vec<T>> {
                let this = self.as_mut().get_mut();
                let mut all_done = true;
                for i in 0..this.futs.len() {
                    if let Some(f) = this.futs[i].as_mut() {
                        match f.as_mut().poll(cx) {
                            Poll::Ready(v) => {
                                this.out[i] = Some(v);
                                this.futs[i] = None;
                            }
                            Poll::Pending => all_done = false,
                        }
                    }
                }
                if all_done {
                    let outs = this.out.iter_mut().map(|o| o.take().unwrap()).collect();
                    Poll::Ready(outs)
                } else {
                    Poll::Pending
                }
            }
        }
        let n = futs.len();
        let mut out: Vec<Option<T>> = Vec::with_capacity(n);
        for _ in 0..n { out.push(None); }
        JoinAll {
            futs: futs.into_iter().map(Some).collect(),
            out,
        }
        .await
    }
}
