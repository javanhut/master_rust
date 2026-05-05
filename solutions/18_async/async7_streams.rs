// SOLUTION — async7_streams

use runtime::{CounterStream, StreamExt};

async fn sum_stream(n: u32) -> u32 {
    let mut s = CounterStream::new(n);
    let mut total: u32 = 0;
    while let Some(x) = s.next().await {
        total += x;
    }
    total
}

async fn collect_stream(n: u32) -> Vec<u32> {
    let mut s = CounterStream::new(n);
    let mut out = Vec::new();
    while let Some(x) = s.next().await {
        out.push(x);
    }
    out
}

fn main() {
    println!("sum_stream(5) = {}", runtime::block_on(sum_stream(5)));
    println!("collect_stream(3) = {:?}", runtime::block_on(collect_stream(3)));
}

// WHY THIS IS OPTIMAL:
//
//   The pattern `while let Some(x) = s.next().await { ... }` is the
//   async direct translation of `for x in iter { ... }`. Every async
//   library you'll meet — tokio's `tokio_stream`, `futures::StreamExt`,
//   `async-stream` — uses exactly that idiom for consumer code.
//
//   `sum_stream` shows the simplest reducer: a running total. It maps
//   directly to `Iterator::sum` / `Stream::fold` once those are
//   available; the explicit loop is the form you'd write yourself
//   when learning, then refactor into a combinator.
//
//   `collect_stream` is the streaming analogue of `Iterator::collect`.
//   Real Stream collections often use `try_collect` if items are
//   `Result`s, with the same Result short-circuiting trick you saw in
//   the iterators chapter.
//
// HOW THE STREAM MACHINERY HANGS TOGETHER:
//
//   - `Stream` is the trait — one required method `poll_next`, which
//     returns `Poll<Option<Item>>`. Pending = "wait", Ready(Some) =
//     "here's an item", Ready(None) = "stream done."
//
//   - `StreamExt::next(&mut self) -> Next<'_, Self>` returns a small
//     future that, when polled, just calls `poll_next` once. That's
//     the only piece of magic; everything else is a normal future.
//
//   - The `Unpin` bound on `StreamExt` keeps the API ergonomic for our
//     simple stream. Streams that store self-referential state cannot
//     be `Unpin`, and you'd reach for `Pin<&mut S>` projections or
//     `pin!` macros to use them. Out of scope here.
//
// ALTERNATIVES:
//
//   1. With the real `futures` crate:
//
//        use futures::stream::{self, StreamExt};
//        let total = stream::iter(1..=n).fold(0, |acc, x| async move { acc + x }).await;
//
//      Or `try_fold` / `for_each` / `collect`. All combinators on
//      `Stream` mirror their `Iterator` cousins.
//
//   2. The `async-stream` crate gives you `stream! { yield 1; yield 2; }`
//      generator-style syntax — much more pleasant than implementing
//      `Stream` by hand. (Generators in stable Rust will eventually
//      remove that crate's reason to exist.)
//
//   3. For "fan-in": merge multiple streams into one with
//      `futures::stream::select_all` — same idea as `select2` but on
//      streams; emit items from whichever child produces one first.

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

    pub trait Stream {
        type Item;
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<Option<Self::Item>>;
    }

    pub struct CounterStream {
        cur: u32,
        max: u32,
        about_to_yield: bool,
    }
    impl CounterStream {
        pub fn new(max: u32) -> Self {
            CounterStream { cur: 0, max, about_to_yield: false }
        }
    }
    impl Stream for CounterStream {
        type Item = u32;
        fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>)
            -> Poll<Option<u32>>
        {
            if self.cur >= self.max {
                return Poll::Ready(None);
            }
            if !self.about_to_yield {
                self.about_to_yield = true;
                return Poll::Pending;
            }
            self.about_to_yield = false;
            self.cur += 1;
            Poll::Ready(Some(self.cur))
        }
    }
    impl Unpin for CounterStream {}

    pub trait StreamExt: Stream + Unpin {
        fn next(&mut self) -> Next<'_, Self> { Next { stream: self } }
    }
    impl<S: Stream + Unpin + ?Sized> StreamExt for S {}

    pub struct Next<'a, S: ?Sized> { stream: &'a mut S }
    impl<S: Stream + Unpin + ?Sized> Future for Next<'_, S> {
        type Output = Option<S::Item>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Pin::new(&mut *self.stream).poll_next(cx)
        }
    }
}
