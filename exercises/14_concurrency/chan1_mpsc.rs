// =============================================================================
//  chan1 — channels: std::sync::mpsc
// =============================================================================
//
// `std::sync::mpsc` provides MULTI-PRODUCER, SINGLE-CONSUMER channels
// for sending values between threads. Build one with:
//
//     use std::sync::mpsc;
//
//     let (tx, rx) = mpsc::channel::<i32>();
//
// `tx` (`Sender<T>`) sends values; `rx` (`Receiver<T>`) reads them.
//
// SENDING
//
//     tx.send(value)    -> Result<(), SendError<T>>
//
// `send` succeeds unless the receiver has been DROPPED, in which case
// it returns `Err(SendError(value))` — your value back. For the happy
// path you can `.unwrap()` it; in real code you usually log and stop.
//
// RECEIVING
//
//     rx.recv()         -> Result<T, RecvError>
//
// `recv` BLOCKS the current thread until a value arrives. It returns
// `Err(RecvError)` once every `Sender` clone has been dropped AND the
// queue is empty — i.e. "no more messages will ever come, and there
// are none left." That's the canonical signal that the channel is
// closed.
//
//     rx.try_recv()     -> Result<T, TryRecvError>
//
// Non-blocking; returns `TryRecvError::Empty` immediately if there's
// nothing ready, or `TryRecvError::Disconnected` if the senders are
// all gone.
//
// CLOSING THE CHANNEL
//
// You don't close a channel explicitly — it closes when the LAST
// `Sender` is dropped. `recv` then returns `Err(RecvError)`. So if you
// want a worker to stop reading from the channel, drop your senders.
//
// CHANNEL FLAVOURS
//
//   - `mpsc::channel()`        — unbounded; `send` never blocks. The
//                                 queue grows in memory.
//   - `mpsc::sync_channel(n)`  — bounded with capacity `n`; `send`
//                                 blocks when the queue is full. n=0
//                                 is a "rendezvous" channel.
//
// We'll stick with the unbounded one in this exercise.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `send_one(value)` spawns a thread, sends `value` once, and
//     returns whatever the receiver got back.
//   - `closed_after_drop()` creates a channel, drops the sender, and
//     returns `true` if `rx.recv()` reports the channel is closed.

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;

pub fn send_one(value: i32) -> i32 {
    let (tx, rx) = mpsc::???();
    let handle = thread::spawn(move || {
        tx.send(value).unwrap();
    });
    let received = rx.???.unwrap();
    handle.join().unwrap();
    received
}

pub fn closed_after_drop() -> bool {
    let (tx, rx) = mpsc::channel::<i32>();
    drop(tx);
    // recv() should return Err now — every sender is gone and the
    // queue is empty. Make this expression evaluate to `true` exactly
    // when the channel is closed.
    rx.recv().???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn round_trip() {
        assert_eq!(send_one(7), 7);
    }

    #[test] fn round_trip_negative() {
        assert_eq!(send_one(-42), -42);
    }

    #[test] fn dropping_sender_closes_channel() {
        assert!(closed_after_drop());
    }
}

fn main() {}
