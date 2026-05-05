// SOLUTION — chan2_iter

use std::sync::mpsc;
use std::thread;

pub fn collect_via_iter(values: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for v in values {
            tx.send(v).unwrap();
        }
        // tx is dropped at end of scope — closes the channel for rx.
    });

    let mut out = Vec::new();
    for v in rx {
        out.push(v);
    }

    producer.join().unwrap();
    out
}

pub fn fan_in_sum() -> i32 {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for v in [1, 2, 3] {
        let txc = tx.clone();
        handles.push(thread::spawn(move || {
            txc.send(v).unwrap();
        }));
    }
    drop(tx);

    let mut total = 0;
    for n in rx {
        total += n;
    }
    for h in handles { h.join().unwrap(); }
    total
}

// WHY THIS IS OPTIMAL:
//
//   `for v in rx` is the canonical drain. It calls `rx.recv()` under
//   the hood and ends the moment recv returns Err — i.e. when every
//   Sender (the original and all clones) has been dropped AND the
//   queue is empty. No manual match, no sentinel value, no extra
//   bookkeeping.
//
//   In `fan_in_sum` we MUST drop the original `tx` after spawning the
//   workers; otherwise the channel still has a live Sender (the one
//   sitting in main), so the receiver loop would block forever after
//   the three workers finish. Each worker takes its own `txc` clone
//   into its closure; when the worker thread ends, that clone drops.
//   When the third worker drops its clone and we've also dropped the
//   original `tx`, the channel closes and the loop exits.
//
//   The send order of `[1, 2, 3]` doesn't matter for the SUM — we
//   only assert the total equals 6 — so the test is robust against
//   the inherent nondeterminism of multi-threaded sends.
//
// ALTERNATIVES:
//
//   - `rx.iter()` is the explicit form of the same thing — useful
//     when you want to chain combinators:
//
//         let total: i32 = rx.iter().sum();
//
//   - `rx.try_iter()` yields whatever is currently buffered without
//     blocking, then ends. Handy for non-blocking polls.
//
//   - To wait for all workers AND collect their messages, you can
//     skip the manual `drop(tx)` by keeping every Sender inside its
//     worker closure (clone before spawn, never bind a long-lived
//     Sender in main). The channel closes automatically once the
//     last worker exits.
//
//   - If you need many consumers (MPMC), reach for crossbeam's
//     `Receiver`, which is `Clone`.
