// =============================================================================
//  async5_spawn — multiple tasks on one executor
// =============================================================================
//
// `block_on(fut)` runs ONE future to completion. Real async programs
// have hundreds of independent tasks: an HTTP server has one task per
// connection, a worker pool has one per job. The executor is in charge
// of fairly polling all of them.
//
// SPAWN, RESTATED
//
//   `spawn(fut)` says "add this future to the task queue and return."
//   The executor will eventually poll it. The CALLER does not wait for
//   the task to finish — it runs in the background. (Real `tokio::spawn`
//   returns a `JoinHandle` you can `.await` to wait for the result;
//   we'll skip that here to keep the runtime small.)
//
//   On a multi-threaded runtime, "spawn" can put the task on ANY worker
//   thread. We're single-threaded — spawn just pushes to a Vec — but
//   the API shape is the same.
//
// TASK ERASURE
//
//   `spawn` accepts futures of any concrete type, but the queue has to
//   hold them in one Vec. That means trait objects:
//
//       type Task = Pin<Box<dyn Future<Output = ()>>>;
//
//   Each spawned future must therefore have `Output = ()` (no return
//   value) — anything you want to communicate back goes through shared
//   state (Rc<Cell>, channels, ...). On a real runtime, `JoinHandle`
//   does this for you.
//
// THE EXECUTOR'S OUTER LOOP
//
//   while !queue.is_empty() {
//       for each task: poll it; if Ready, drop it; if Pending, keep.
//   }
//
//   The runtime in `mod runtime` below is exactly that, plus the
//   waker plumbing. Read it; you'll see it's ~30 lines.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Inside `run_two()`, SPAWN two tasks that each bump a shared
//     `Rc<Cell<u32>>` counter and yield once.
//   - After spawning, call `runtime::run_all()` to drive the executor
//     until the task queue is empty.
//   - Return the final counter value (should be 2).
//
// In real code you'd use a runtime like tokio or async-std —
// `cargo add tokio --features=full` and write `#[tokio::main] async fn main()`.
// We hand-roll the runtime here so you can see what it's doing.

// I AM NOT DONE

use std::cell::Cell;
use std::rc::Rc;

fn run_two() -> u32 {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    let c1 = Rc::clone(&counter);
    runtime::spawn(async move {
        runtime::yield_once().???;
        c1.set(c1.get() + 1);
    });

    let c2 = Rc::clone(&counter);
    runtime::spawn(async move {
        runtime::yield_once().???;
        c2.set(c2.get() + 1);
    });

    runtime::???();           // drive the executor until the queue is empty

    counter.get()
}

fn main() {}

mod runtime {
    use core::future::Future;
    use core::pin::Pin;
    use core::ptr;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use std::cell::RefCell;

    fn noop_raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        static VTABLE: RawWakerVTable =
            RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(ptr::null(), &VTABLE)
    }

    type Task = Pin<Box<dyn Future<Output = ()>>>;

    thread_local! {
        static QUEUE: RefCell<Vec<Task>> = RefCell::new(Vec::new());
    }

    /// Push a future onto the executor's task queue. The future runs
    /// later when something calls `run_all` (or `block_on`). The shape
    /// matches `tokio::spawn(fut)` minus the JoinHandle return.
    pub fn spawn<F>(fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        QUEUE.with(|q| q.borrow_mut().push(Box::pin(fut)));
    }

    /// Drain the task queue, polling each task until it reports Ready.
    /// Tasks may spawn more tasks; we keep going until the queue stays
    /// empty after a full pass.
    pub fn run_all() {
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut ctx = Context::from_waker(&waker);

        loop {
            // Take ownership of the current queue so tasks can spawn
            // more without re-entering the borrow.
            let mut tasks: Vec<Task> = QUEUE.with(|q| std::mem::take(&mut *q.borrow_mut()));
            if tasks.is_empty() {
                return;
            }

            let mut still_running: Vec<Task> = Vec::new();
            for mut task in tasks.drain(..) {
                match task.as_mut().poll(&mut ctx) {
                    Poll::Ready(())  => { /* task done */ }
                    Poll::Pending    => still_running.push(task),
                }
            }

            // Put unfinished tasks back on the queue for the next round,
            // followed by anything spawned during this pass.
            QUEUE.with(|q| {
                let mut q = q.borrow_mut();
                let newly_spawned = std::mem::take(&mut *q);
                *q = still_running;
                q.extend(newly_spawned);
            });
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn two_tasks_both_increment() {
        assert_eq!(run_two(), 2);
    }

    #[test] fn many_spawns_all_complete() {
        let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        for _ in 0..10 {
            let c = Rc::clone(&counter);
            runtime::spawn(async move {
                runtime::yield_once().await;
                c.set(c.get() + 1);
            });
        }
        runtime::run_all();
        assert_eq!(counter.get(), 10);
    }
}
