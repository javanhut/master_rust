// SOLUTION — async5_spawn

use std::cell::Cell;
use std::rc::Rc;

fn run_two() -> u32 {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    let c1 = Rc::clone(&counter);
    runtime::spawn(async move {
        runtime::yield_once().await;
        c1.set(c1.get() + 1);
    });

    let c2 = Rc::clone(&counter);
    runtime::spawn(async move {
        runtime::yield_once().await;
        c2.set(c2.get() + 1);
    });

    runtime::run_all();

    counter.get()
}

fn main() {
    println!("counter ended at {}", run_two());
}

// WHY THIS IS OPTIMAL:
//
//   `spawn` plus `run_all` mirror exactly what real runtimes do:
//
//     tokio::spawn(async move { /* task body */ });
//     // ... later, the runtime drains its task queue.
//
//   Each spawned future is `'static` (no borrows that could outlive the
//   queue) with `Output = ()`. We share state across tasks via
//   `Rc<Cell<u32>>` because we're single-threaded; on a multi-threaded
//   runtime that becomes `Arc<Mutex<u32>>` or `Arc<AtomicU32>`.
//
//   `yield_once().await` inside each task is what makes the example
//   visibly multi-task: each task suspends after the first poll, so the
//   executor moves on to the next task before coming back. Without the
//   yield, both tasks would finish in a single poll and the
//   "interleaving" would be invisible — true even with tokio, where a
//   task that never awaits anything is just a synchronous function.
//
// HOW THE EXECUTOR HANDLES SPAWNS-DURING-RUN:
//
//   `run_all` takes the queue ownership for each pass, polls every
//   task, and merges (1) tasks that returned Pending with (2) any new
//   tasks spawned by those polls. It keeps going until the merged
//   queue is empty after a full pass. This is the same algorithm a
//   single-threaded executor like `smol::block_on` uses internally.
//
// ALTERNATIVES:
//
//   1. With tokio:
//
//        let h1 = tokio::spawn(async move { ... });
//        let h2 = tokio::spawn(async move { ... });
//        h1.await.unwrap();
//        h2.await.unwrap();
//
//      `JoinHandle` lets the spawner await the result. To replicate it
//      we'd need a one-shot channel (oneshot) per task. Out of scope.
//
//   2. For "spawn N homogeneous tasks and wait for all," `join_all` (or
//      our `join2` generalised) is often a better fit than `spawn` — it
//      keeps the futures inline and gives you back their outputs.
//
//   3. A real executor uses a wait/notify primitive instead of busy-
//      polling Pending tasks. The Waker is the hook: each task's waker
//      records "I'm in the ready set," and the executor only polls
//      tasks that are ready. We skip ready-tracking here because every
//      Pending in this chapter is "ready on the next tick" anyway.

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

    pub fn spawn<F>(fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        QUEUE.with(|q| q.borrow_mut().push(Box::pin(fut)));
    }

    pub fn run_all() {
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut ctx = Context::from_waker(&waker);

        loop {
            let mut tasks: Vec<Task> = QUEUE.with(|q| std::mem::take(&mut *q.borrow_mut()));
            if tasks.is_empty() {
                return;
            }

            let mut still_running: Vec<Task> = Vec::new();
            for mut task in tasks.drain(..) {
                match task.as_mut().poll(&mut ctx) {
                    Poll::Ready(()) => {}
                    Poll::Pending   => still_running.push(task),
                }
            }

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
