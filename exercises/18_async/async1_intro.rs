// =============================================================================
//  async1_intro — what `async fn` actually IS
// =============================================================================
//
// Async Rust solves a specific problem: running thousands of concurrent
// I/O tasks on a small thread pool without the overhead of one OS thread
// per task. The mechanism is co-operative — a task RUNS until it can't
// make progress, then YIELDS so another task can run.
//
// THE CENTRAL FACT
//
//   `async fn foo() -> T` is sugar for a function that returns
//   `impl Future<Output = T>`.
//
// Concretely:
//
//     async fn answer() -> i32 { 42 }
//
//   is, modulo desugaring, the same as
//
//     fn answer() -> impl Future<Output = i32> { /* state machine */ }
//
//   Calling `answer()` does NOT run the body. It builds a state-machine
//   value that is sitting at "step 0, not started." Futures are LAZY.
//
// HOW DOES IT EVER RUN?
//
//   Something has to POLL the future. The trait is:
//
//     pub trait Future {
//         type Output;
//         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//     }
//
//   Each call to `poll` advances the state machine until either:
//     - it produces `Poll::Ready(value)`  — done, here's the result.
//     - it produces `Poll::Pending`        — "I'm stuck on something
//                                            (a socket, a timer, a lock).
//                                            Wake me when it's ready."
//
//   The thing that calls `poll` in a loop is an EXECUTOR (also called a
//   runtime). `tokio`, `async-std`, `smol`, `embassy` are all executors.
//
// `.await` IS POLL-AND-RESUME
//
//   Inside an `async fn`, writing `other_future.await` compiles to roughly:
//
//     loop {
//         match other_future.poll(cx) {
//             Poll::Ready(v)  => break v,
//             Poll::Pending   => yield,           // suspend THIS future too
//         }
//     }
//
//   So `.await` is the suspension point. Between two `.await`s the code
//   runs synchronously; AT each `.await` the task may park.
//
// WHY HAND-ROLL A RUNTIME?
//
//   This course compiles each exercise as a single .rs file with no
//   external crates, so we can't pull in `tokio`. Instead we ship a tiny
//   `block_on` at the bottom of every exercise. It uses a no-op waker
//   and just spins, polling until the future is `Ready`. That's enough
//   to demonstrate every async-Rust concept short of real I/O.
//
//   The teaching value: you'll see EXACTLY what `#[tokio::main]` is
//   hiding. When you reach for a real runtime later, the magic will be
//   gone — you'll know it's just a smarter `block_on`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Write `answer()` as an `async fn` that returns `42i32`.
//   - Write `add_async(a, b)` as an `async fn` that returns `a + b`.
//   - In `run()`, use `runtime::block_on(...)` to drive `answer().await + 1`
//     to completion and return the i32 result.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

async fn answer() -> i32 {
    ???
}

async fn add_async(a: i32, b: i32) -> i32 {
    ???
}

fn run() -> i32 {
    runtime::block_on(async {
        let n = answer().???;
        n + 1
    })
}

fn main() {}

// -------- hand-rolled runtime (not part of the lesson surface) ---------------
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
        // SAFETY: `fut` is a local on this stack frame and never moved
        // again before it's dropped at end of this function.
        let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
        loop {
            match fut.as_mut().poll(&mut ctx) {
                Poll::Ready(v) => return v,
                Poll::Pending => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn answer_is_42() {
        assert_eq!(runtime::block_on(answer()), 42);
    }

    #[test] fn add_async_works() {
        assert_eq!(runtime::block_on(add_async(2, 3)), 5);
    }

    #[test] fn run_returns_43() {
        assert_eq!(run(), 43);
    }

    #[test] fn calling_async_fn_does_nothing() {
        // Building the future MUST NOT execute the body — futures are lazy.
        // We can't easily observe "didn't run" here without side effects,
        // but we CAN observe that the returned value is a real future we
        // have to drive. If `answer()` ran eagerly, the return type would
        // be i32, not impl Future.
        let fut = answer();
        let v = runtime::block_on(fut);
        assert_eq!(v, 42);
    }
}
