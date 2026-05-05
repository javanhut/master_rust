// =============================================================================
//  chan2 — receivers as iterators, and multi-producer fan-in
// =============================================================================
//
// `Receiver<T>` implements `IntoIterator`, so this just works:
//
//     for msg in rx {
//         // each iteration is rx.recv(); loop ends when the channel closes
//     }
//
// The iterator yields `T` (not `Result<T, _>`): it loops until `recv`
// returns Err, then ends silently. This is by far the most ergonomic
// way to drain a channel — no manual `match` on RecvError, no
// `while let Some(_) = ...`, just a `for`.
//
// MULTI-PRODUCER (the M in MPSC)
//
// `Sender<T>` implements `Clone`. EACH clone is an independent send
// handle into the SAME channel. The channel stays open as long as at
// least one Sender clone is alive:
//
//     let (tx, rx) = mpsc::channel();
//     let tx2 = tx.clone();
//     thread::spawn(move || tx.send(1).unwrap());
//     thread::spawn(move || tx2.send(2).unwrap());
//     // both senders dropped after their threads end
//     for n in rx { println!("{n}"); }   // prints 1 and 2 in some order
//
// The receiver collects messages from BOTH producers. The order is
// nondeterministic — whichever thread sends first is read first.
//
// THE "ALL SENDERS DROPPED" SIGNAL
//
// If you keep the ORIGINAL `tx` around in main and only clone for
// workers, the loop will hang forever waiting for messages from a
// sender that never dies. The fix: drop the original `tx` (or just
// don't keep one — clone every handle the workers need from a single
// `tx` and let the original go out of scope) before iterating.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `collect_via_iter(values)` spawns ONE producer that sends every
//     element of `values`, then drains the channel with a `for` loop
//     and returns the collected `Vec<i32>` (in send order).
//   - `fan_in_sum()` spawns THREE producers that send 1, 2, and 3
//     respectively, then sums everything received. Result must be 6.

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;

pub fn collect_via_iter(values: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for v in values {
            tx.send(v).unwrap();
        }
        // tx drops here — channel will close once we drain.
    });

    let mut out = Vec::new();
    // Drain the receiver as an iterator. Replace the ??? with the
    // pattern variable name and drop each item into `out`.
    for ??? in rx {
        out.push(???);
    }

    producer.join().unwrap();
    out
}

pub fn fan_in_sum() -> i32 {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for v in [1, 2, 3] {
        // Each worker needs its OWN sender. Clone tx for the worker
        // and `move` the clone into the closure.
        let txc = tx.???();
        handles.push(thread::spawn(move || {
            txc.send(v).unwrap();
        }));
    }
    // IMPORTANT: drop the original `tx` so the channel can close once
    // all worker clones have been dropped.
    drop(tx);

    let mut total = 0;
    for n in rx {
        total += n;
    }
    for h in handles { h.join().unwrap(); }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn iter_collects_in_send_order() {
        assert_eq!(collect_via_iter(vec![10, 20, 30]), vec![10, 20, 30]);
    }

    #[test] fn iter_handles_empty_input() {
        assert_eq!(collect_via_iter(Vec::new()), Vec::<i32>::new());
    }

    #[test] fn fan_in_sum_is_six() {
        assert_eq!(fan_in_sum(), 6);
    }
}

fn main() {}
