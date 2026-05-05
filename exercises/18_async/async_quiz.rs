// =============================================================================
//  async_quiz — capstone: parallel "HTTP fetches" with join
// =============================================================================
//
// You're modelling a service that needs to call THREE downstream APIs
// for the same request, then assemble their responses. A naive
// implementation calls them one after another:
//
//     let a = api_a().await;       // 100ms
//     let b = api_b().await;       // 100ms
//     let c = api_c().await;       // 100ms
//     // total ≈ 300ms
//
// The async-y thing is to fire all three off CONCURRENTLY:
//
//     let (a, b, c) = join3(api_a(), api_b(), api_c()).await;
//     // total ≈ 100ms (limited by the slowest)
//
// We provide `runtime::join_all(Vec<F>) -> Vec<F::Output>` in `mod runtime`
// — same idea as `futures::future::join_all` on the real ecosystem.
// You'll feed it three "fetch" futures.
//
// THE FAKE FETCHES
//
// Each `fetch_*` is an async fn that:
//   1. Calls `runtime::yield_once().await` (modelling network latency).
//   2. Returns its tag string ("alpha" / "beta" / "gamma").
//
// THIS IS THE FINAL EXERCISE
//
// You're putting together every concept from the chapter:
//
//   - `async fn` — the source of futures.
//   - `.await`  — the suspension point inside an async block.
//   - `join_all` — concurrent composition.
//   - `block_on` — the bridge from sync `main` into async.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Implement the three async fetches: each yields once, then returns
//     its name.
//   - Implement `fetch_all()` so it concurrently runs all three and
//     returns a `Vec<&'static str>` containing the responses in
//     submission order: ["alpha", "beta", "gamma"].

// I AM NOT DONE

async fn fetch_alpha() -> &'static str {
    runtime::yield_once().???;
    "alpha"
}

async fn fetch_beta() -> &'static str {
    runtime::yield_once().???;
    ???
}

async fn fetch_gamma() -> &'static str {
    runtime::yield_once().???;
    ???
}

async fn fetch_all() -> Vec<&'static str> {
    // Build the Vec of futures. Each element has the same concrete
    // future type (it's `impl Future<Output = &'static str>` from an
    // async fn) — but join_all is generic over a uniform F, so we
    // box them as `Pin<Box<dyn Future<Output = &'static str>>>` to
    // erase the per-async-fn type difference.
    let futures: Vec<runtime::BoxFuture<&'static str>> = vec![
        Box::pin(fetch_alpha()),
        Box::pin(???),
        Box::pin(???),
    ];
    runtime::join_all(futures).???
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

    /// Type alias so the exercise body stays readable.
    pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

    /// Hand-rolled equivalent of `futures::future::join_all`. Polls
    /// every child on each pass; returns a Vec of outputs in the same
    /// order the inputs were provided once all are Ready.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn each_fetch_alone() {
        assert_eq!(runtime::block_on(fetch_alpha()), "alpha");
        assert_eq!(runtime::block_on(fetch_beta()),  "beta");
        assert_eq!(runtime::block_on(fetch_gamma()), "gamma");
    }

    #[test] fn fetch_all_returns_three_in_order() {
        let v = runtime::block_on(fetch_all());
        assert_eq!(v, vec!["alpha", "beta", "gamma"]);
    }

    #[test] fn fetch_all_returns_three() {
        let v = runtime::block_on(fetch_all());
        assert_eq!(v.len(), 3);
    }
}
