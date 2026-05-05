// SOLUTION — async2_runtime

async fn double(x: i32) -> i32 {
    x * 2
}

async fn chained(x: i32) -> i32 {
    let a = double(x).await;
    let b = double(a).await;
    b
}

fn kick_off() -> i32 {
    runtime::block_on(chained(5))
}

fn main() {
    println!("kick_off() = {}", kick_off());
}

// WHY THIS IS OPTIMAL:
//
//   The body of `chained` shows what `.await` actually buys you: the
//   ability to CALL the next async step using the return value of the
//   previous one, written as straight-line code instead of a callback
//   chain. The compiler turns this into a state machine where each
//   `.await` is a "save my locals and return Pending" point.
//
//   `kick_off` is the bridge from sync to async. Every async program has
//   to start with a sync `main` and use SOME executor entry point —
//   tokio's `#[tokio::main]`, smol's `block_on`, or our hand-rolled one.
//
// THE EXECUTOR LOOP, RESTATED:
//
//   loop {
//       match fut.as_mut().poll(&mut ctx) {
//           Poll::Ready(v) => return v,
//           Poll::Pending  => /* a real executor would park here */,
//       }
//   }
//
//   The only reason a SPIN loop is acceptable in this course is that
//   our futures never depend on external events. Every Pending is
//   immediately followed by a poll that progresses the state machine.
//   In real code with timers and sockets this would burn 100% CPU
//   waiting for something that hasn't happened yet — that's why real
//   runtimes integrate with the OS (epoll, kqueue, IOCP) to sleep until
//   a relevant event arrives.
//
// ALTERNATIVES:
//
//   1. `chained` could be one line:  `double(double(x).await).await * 1`
//      — same semantics, less obvious. The two-step form is what most
//      real async code looks like: bind, transform, bind again.
//
//   2. The body of `kick_off` could spell the future inline:
//
//          runtime::block_on(async { chained(5).await })
//
//      Equivalent but redundant; passing the future directly is fine.
//
//   3. A real runtime would let you spawn `chained(5)` as a task that
//      runs concurrently with other tasks (chapter exercise 5 covers
//      that). With a single-future executor like ours, "spawn" and
//      "block_on" are the same operation.

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
