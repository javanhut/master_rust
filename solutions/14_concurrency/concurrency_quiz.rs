// SOLUTION — concurrency_quiz

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn count_words_pool(lines: Vec<String>, num_workers: usize) -> usize {
    if num_workers == 0 {
        return 0;
    }

    let (jobs_tx, jobs_rx) = mpsc::channel::<String>();
    let (results_tx, results_rx) = mpsc::channel::<usize>();

    let jobs_rx = Arc::new(Mutex::new(jobs_rx));

    let mut workers = Vec::new();
    for _ in 0..num_workers {
        let jobs_rx = Arc::clone(&jobs_rx);
        let results_tx = results_tx.clone();
        workers.push(thread::spawn(move || {
            loop {
                let next = jobs_rx.lock().unwrap().recv();
                match next {
                    Ok(line) => {
                        let n = line.split_whitespace().count();
                        results_tx.send(n).unwrap();
                    }
                    Err(_) => break,
                }
            }
        }));
    }

    drop(results_tx);

    for line in lines {
        jobs_tx.send(line).unwrap();
    }
    drop(jobs_tx);

    let mut total = 0;
    for n in results_rx {
        total += n;
    }
    for w in workers { w.join().unwrap(); }
    total
}

// WHY THIS IS OPTIMAL:
//
//   Two channels, in opposite directions, plus an Arc<Mutex<...>>
//   around the single jobs receiver — that's the entire
//   thread-pool-with-results pattern. Each piece is doing exactly
//   one job:
//
//     jobs_tx / jobs_rx     : main -> workers, fan-out queue.
//     results_tx / results_rx : workers -> main, fan-in queue.
//     Arc<Mutex<jobs_rx>>   : lets every worker share the single
//                              receiver. Single-consumer mpsc
//                              wouldn't otherwise be cloneable.
//
//   The CRITICAL detail is what the worker holds the jobs lock for.
//   We lock, recv ONE line, drop the lock, THEN do the work and send
//   the result. If we held the lock across `split_whitespace().count()`
//   the workers would be serialised on the lock and you'd get
//   exactly the throughput of a single thread (plus overhead).
//   Locking only the queue handoff is what makes this a real pool.
//
//   The two `drop()` calls are the load-bearing termination logic.
//   Without `drop(jobs_tx)`, the workers' `recv()` would block
//   forever once the queue empties — there's still a live Sender
//   sitting in main. Without `drop(results_tx)`, the main thread's
//   `for n in results_rx` would block forever once the workers
//   finish — there's still a live Sender (the original) in main. Drop
//   both eagerly, and every channel closes the moment its work is
//   done.
//
// HOW THE TESTS WORK:
//
//   `count_words_pool` is deterministic in its OUTPUT — sum over
//   per-line word counts is order-independent — even though the
//   workers visit lines in nondeterministic order. We compute the
//   serial answer in the test and compare. That style of test is
//   exactly what you want for concurrent code: assert the invariant,
//   not the schedule.
//
// ALTERNATIVES:
//
//   - For real production thread pools reach for `rayon`. Its
//     `par_iter().map(...).sum()` is one line and uses work-stealing
//     to balance load. The hand-rolled pool here is a teaching tool
//     that shows you what rayon is doing under the hood.
//
//   - `crossbeam_channel` provides a Receiver that IS Clone, which
//     would let you skip the Arc<Mutex<...>> dance — every worker
//     just holds its own receiver clone.
//
//   - For very many short tasks, the Arc<Mutex<Receiver>> handoff
//     becomes the bottleneck (every job requires taking a global
//     lock). Solutions: use a lock-free queue (crossbeam's `deque`),
//     pre-partition the work into per-worker queues, or batch
//     multiple jobs per recv.
//
//   - If you ALSO want the results IN INPUT ORDER, send `(index, line)`
//     into the jobs channel, send `(index, count)` back, and sort
//     the collected results before summing/printing. For a sum the
//     order doesn't matter, so we skip it.
