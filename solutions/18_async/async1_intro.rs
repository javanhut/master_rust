// SOLUTION — async1_intro

async fn answer() -> i32 {
    42
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

fn run() -> i32 {
    runtime::block_on(async {
        let n = answer().await;
        n + 1
    })
}

fn main() {
    let v = run();
    println!("run() = {v}");
}

// WHY THIS IS OPTIMAL:
//
//   `async fn answer() -> i32 { 42 }` is the simplest possible future.
//   Calling `answer()` builds a tiny state machine whose only state is
//   "not yet started." The first `poll` runs the body to completion and
//   returns `Poll::Ready(42)`.
//
//   `add_async` is the same shape with two parameters captured by value
//   inside the future. There is no `.await` in either body, so there is
//   no suspension point — these futures complete in a single poll.
//
//   `run()` shows the everyday pattern in async Rust: an OUTER async
//   block stitches together `await`s on inner futures. `runtime::block_on`
//   takes the outer future and polls it to completion. In `tokio` you'd
//   spell exactly the same flow as
//
//     #[tokio::main]
//     async fn main() {
//         let n = answer().await;
//         println!("{}", n + 1);
//     }
//
//   The macro injects a `Runtime::new().block_on(async { ... })` call
//   around your `main` body. We do the equivalent by hand.
//
// ALTERNATIVES:
//
//   1. Drop the outer `async {}` block and just `block_on(answer())`
//      directly when there's only one future — works but doesn't
//      illustrate `.await`. The form above mirrors what you'd write
//      once there's more than one inner future to compose.
//
//   2. Spell the desugaring out: `fn answer() -> impl Future<Output = i32>`
//      with a hand-written struct implementing `Future`. The compiler
//      generates exactly that for you from `async fn` — it just looks
//      uglier and forces you to think about pinning.
//
// MENTAL MODEL TO CARRY FORWARD:
//
//   - `async fn` returns a future; calling it produces a paused state
//      machine.
//   - `.await` is "drive that nested future, suspending mine if needed."
//   - An EXECUTOR provides the outer loop that calls `poll` repeatedly
//      and (in real runtimes) parks the thread between wakeups.

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
}
