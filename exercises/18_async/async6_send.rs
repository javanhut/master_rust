// =============================================================================
//  async6_send — why real runtimes demand `Send` futures
// =============================================================================
//
// On a multi-threaded executor like `tokio`, a spawned task may move
// between worker threads as the runtime load-balances. So the future
// itself — its captured state, every variable held across an `.await`
// — has to be `Send`. The signature of `tokio::spawn` enforces this:
//
//     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
//     where F: Future + Send + 'static, F::Output: Send + 'static;
//
// THE TRAP: HOLDING `Rc` ACROSS `.await`
//
// `Rc<T>` is NOT `Send` (its non-atomic refcount is not safe to share
// across threads). If you `.await` while holding an `Rc`, the resulting
// state machine has an `Rc` field — therefore the whole future is
// non-`Send`, and `tokio::spawn` rejects it at compile time.
//
//     async fn bad() {
//         let shared: Rc<u32> = Rc::new(7);
//         yield_once().await;       // <-- shared is held across await
//         println!("{}", *shared);
//     }
//     // tokio::spawn(bad()) — ERROR: future is not Send
//
// THE FIX: USE `Arc<T>` (and `Mutex<T>` for mutation)
//
// `Arc<T>` IS `Send + Sync`. Same API, atomic refcount. Cross-thread
// shared ownership.
//
//     async fn good() {
//         let shared: Arc<u32> = Arc::new(7);
//         yield_once().await;
//         println!("{}", *shared);
//     }
//     // tokio::spawn(good()) — fine.
//
// THIS EXERCISE IS COMPILE-MODE
//
// We provide a `spawn` function whose bound is `F: Future + Send + 'static`,
// matching what tokio requires. Below it, an async function uses an `Rc`
// across an `.await`. The file currently FAILS to compile. Your job is
// to switch to `Arc` so it compiles.
//
// You will NOT add channels or atomics or anything fancy — just the one
// type swap and adjust the import. The point is to feel the trait bound
// bite, then to fix it the way you'd fix it in production.
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

use std::rc::Rc;       // <-- swap this import to make the future Send

async fn task() {
    let shared: ???<u32> = ???::new(7);     // pick the thread-safe shared-ownership type
    runtime::yield_once().await;
    let _ = *shared;                        // hold `shared` across the await
}

fn main() {
    // `runtime::spawn` requires `F: Future + Send + 'static`. If `task()`
    // captures an Rc across an await, this line fails to compile with
    // an error mentioning `Rc<u32> cannot be sent between threads safely`.
    runtime::spawn(task());
}

mod runtime {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    /// Mirrors the Send + 'static bound that `tokio::spawn` requires.
    /// Single-threaded executors like `tokio::task::spawn_local` drop
    /// the `Send` bound; multi-threaded ones keep it.
    pub fn spawn<F>(_fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // For this lesson we don't actually run the future — just
        // accept it. The COMPILE step is what teaches.
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
}
