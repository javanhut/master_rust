// =============================================================================
//  async7_streams — `Stream` is the async sibling of `Iterator`
// =============================================================================
//
// Iterator yields a sequence of values SYNCHRONOUSLY — calling `.next()`
// either returns the next item right now or `None`. A Stream yields a
// sequence of values ASYNCHRONOUSLY — each call to `poll_next` either
// returns the next item, says "no more," or says "not ready yet, poll
// me again later."
//
// THE TRAIT
//
//   pub trait Stream {
//       type Item;
//       fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
//           -> Poll<Option<Self::Item>>;
//   }
//
// The return type is `Poll<Option<Item>>`:
//   - `Poll::Pending`             — wake me when there might be an item.
//   - `Poll::Ready(Some(item))`   — here's one.
//   - `Poll::Ready(None)`         — stream is finished.
//
// In the std hierarchy this trait lives in the `futures` crate today
// and is being stabilised piece by piece (`futures::Stream`,
// `core::async_iter::AsyncIterator`). We define our own copy in
// `mod runtime` below — it's only six lines and identical in spirit.
//
// CONSUMING A STREAM: HAND-ROLLED `next().await`
//
// Iterator gives you `for x in iter`; Stream doesn't have `for await`
// in stable Rust yet. The idiom is:
//
//     while let Some(item) = stream.next().await {
//         use(item);
//     }
//
// where `next()` is an inherent method that wraps `poll_next` in a
// future. We provide one in `mod runtime` so you don't have to write
// pin gymnastics by hand.
//
// THIS EXERCISE
//
// We ship a `CounterStream` that produces 1, 2, 3, ..., n and then
// signals the end. Your job:
//   - `sum_stream(n)` — drain the stream and return the sum.
//   - `collect_stream(n)` — drain into a `Vec<u32>`.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

use runtime::{CounterStream, StreamExt};

async fn sum_stream(n: u32) -> u32 {
    let mut s = CounterStream::new(n);
    let mut total: u32 = 0;
    while let Some(x) = s.next().??? {
        total += x;
    }
    total
}

async fn collect_stream(n: u32) -> Vec<u32> {
    let mut s = CounterStream::new(n);
    let mut out = ???;
    while let Some(x) = s.next().??? {
        out.???(x);
    }
    out
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

    // Mirror of `futures::Stream` / `core::async_iter::AsyncIterator`.
    pub trait Stream {
        type Item;
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<Option<Self::Item>>;
    }

    /// 1, 2, 3, ..., n and then `None`. Yields once between items so
    /// each item arrives on a distinct poll — same shape as a network
    /// stream that produces one record per round-trip.
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
                // Suspend once before producing the next item.
                self.about_to_yield = true;
                return Poll::Pending;
            }
            self.about_to_yield = false;
            self.cur += 1;
            Poll::Ready(Some(self.cur))
        }
    }
    // The CounterStream has no self-referential state, so it's Unpin.
    impl Unpin for CounterStream {}

    /// Extension trait providing `.next()` — the await-able analogue of
    /// `Iterator::next`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use runtime::block_on;

    #[test] fn sum_zero_is_zero() {
        assert_eq!(block_on(sum_stream(0)), 0);
    }

    #[test] fn sum_five() {
        assert_eq!(block_on(sum_stream(5)), 1 + 2 + 3 + 4 + 5);
    }

    #[test] fn collect_three() {
        assert_eq!(block_on(collect_stream(3)), vec![1, 2, 3]);
    }
}
