// SOLUTION — chan1_mpsc

use std::sync::mpsc;
use std::thread;

pub fn send_one(value: i32) -> i32 {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send(value).unwrap();
        // tx is dropped here — that's fine; rx already has the value.
    });
    let received = rx.recv().unwrap();
    handle.join().unwrap();
    received
}

pub fn closed_after_drop() -> bool {
    let (tx, rx) = mpsc::channel::<i32>();
    drop(tx);
    rx.recv().is_err()
}

// WHY THIS IS OPTIMAL:
//
//   `mpsc::channel()` returns `(Sender<T>, Receiver<T>)`. The Sender
//   half implements `Send` (and `Clone`); the Receiver half implements
//   `Send` but NOT `Clone` — there is exactly one consumer. Moving
//   `tx` into the spawned thread transfers send-rights there; `rx`
//   stays put on the main thread.
//
//   `recv()` blocks until a value arrives or every Sender clone is
//   dropped and the queue is empty. In `send_one` the worker sends
//   exactly one value, then drops its `tx` at end of scope — `recv()`
//   returns Ok(value), and the join below cleans the thread up.
//
//   `closed_after_drop` deliberately drops the only sender BEFORE
//   calling `recv`. With no senders left the channel is closed, so
//   `recv` returns `Err(RecvError)`, which `is_err()` turns into the
//   `true` we want. This is the standard "did the producer side hang
//   up?" check.
//
// ALTERNATIVES:
//
//   - `mpsc::sync_channel(n)` is the bounded variant. `send` blocks
//     when the queue holds `n` items, providing natural back-pressure.
//     With `n == 0` you get a rendezvous channel: each send waits for
//     a matching recv before either side proceeds.
//
//   - `try_recv()` returns immediately:
//
//         Ok(v)                                — got a value
//         Err(TryRecvError::Empty)             — nothing ready
//         Err(TryRecvError::Disconnected)      — closed for good
//
//     Use it when you want to poll the channel from a thread that
//     also has other work to do.
//
//   - `recv_timeout(Duration)` is `recv` with a deadline.
//
//   - The community crate `crossbeam-channel` offers MPMC (multiple
//     consumers), `select!` over many channels, and lower latency. If
//     you need any of those, reach for it.
