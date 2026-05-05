// =============================================================================
//  concurrency_quiz — capstone: a tiny thread-pool word counter
// =============================================================================
//
// You'll build a fixed-size thread pool that counts words across many
// input lines. The shape is:
//
//                        +------------------+
//             lines ---> | jobs channel (mpsc) | --->  N worker threads
//                        +------------------+        each loops:
//                                                        for line in jobs {
//                                                            let n = words(line);
//                                                            results.send(n);
//                                                        }
//
//                                                    +-------------------+
//                                                    | results channel   | ---> main
//                                                    +-------------------+
//
// The main thread:
//   1. creates two channels — `jobs` (lines -> workers) and
//      `results` (per-line counts -> main).
//   2. spawns `num_workers` threads. Each owns one CLONE of the
//      jobs receiver (more on this in a moment) and one clone of the
//      results sender.
//   3. sends every input line into `jobs`, then drops its sender so
//      `jobs` will close once the queue is drained.
//   4. drops its own clone of `results_tx` so the results channel
//      will close once every worker has dropped its clone.
//   5. drains the results channel and sums the counts.
//
// THE "ONE RECEIVER" GOTCHA
//
// `mpsc::Receiver<T>` is single-consumer — it does NOT implement
// `Clone`. To let many workers all pull from the same job queue, the
// idiomatic pattern is `Arc<Mutex<Receiver<T>>>`:
//
//     let jobs_rx = Arc::new(Mutex::new(jobs_rx));
//     for _ in 0..num_workers {
//         let rx = Arc::clone(&jobs_rx);
//         thread::spawn(move || {
//             loop {
//                 let line = match rx.lock().unwrap().recv() {
//                     Ok(line) => line,
//                     Err(_)   => break,   // jobs channel closed
//                 };
//                 // ... process line, send to results ...
//             }
//         });
//     }
//
// The mutex is taken only long enough to grab ONE line, then dropped
// before the work runs. Holding the lock across the actual work would
// serialise the workers — defeating the point of the pool.
//
// COUNTING WORDS
//
// `line.split_whitespace().count()` gives the word count. It collapses
// runs of any whitespace and ignores leading / trailing space, which
// matches what most people mean by "words."
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   Implement `count_words_pool(lines, num_workers) -> usize`. It must
//   return the TOTAL number of whitespace-separated words across all
//   `lines`, computed by `num_workers` worker threads.
//
//   The tests use small inputs and small worker counts.

// I AM NOT DONE

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn count_words_pool(lines: Vec<String>, num_workers: usize) -> usize {
    // Edge case: no workers means nothing can ever be counted.
    if num_workers == 0 {
        return 0;
    }

    let (jobs_tx, jobs_rx) = mpsc::channel::<String>();
    let (results_tx, results_rx) = mpsc::channel::<usize>();

    // Wrap the single receiver so workers can share it.
    let jobs_rx = Arc::new(Mutex::new(jobs_rx));

    // Spawn the pool.
    let mut workers = Vec::new();
    for _ in 0..num_workers {
        let jobs_rx = Arc::???(&jobs_rx);
        let results_tx = results_tx.???();
        workers.push(thread::spawn(move || {
            loop {
                // Take the lock, pull ONE line, drop the lock.
                let next = jobs_rx.???.unwrap().recv();
                match next {
                    Ok(line) => {
                        let n = line.split_whitespace().count();
                        results_tx.send(n).unwrap();
                    }
                    Err(_) => break, // jobs channel closed -> exit
                }
            }
        }));
    }

    // Drop the original results sender so the channel can close after
    // every worker has dropped its clone.
    drop(results_tx);

    // Send every job, then drop the original jobs sender so the
    // workers' recv() will return Err once the queue empties.
    for line in lines {
        jobs_tx.send(line).unwrap();
    }
    drop(jobs_tx);

    // Drain results and sum.
    let mut total = 0;
    for n in results_rx {
        total += n;
    }
    for w in workers { w.join().unwrap(); }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(|s| s.to_string()).collect()
    }

    #[test] fn empty_input() {
        assert_eq!(count_words_pool(Vec::new(), 4), 0);
    }

    #[test] fn zero_workers_returns_zero() {
        assert_eq!(count_words_pool(lines("a b c"), 0), 0);
    }

    #[test] fn single_worker_counts_all() {
        let input = lines("hello world\nthe quick brown fox\nrust");
        assert_eq!(count_words_pool(input, 1), 2 + 4 + 1);
    }

    #[test] fn pool_matches_serial_count() {
        let text = "\
the quick brown fox jumps over the lazy dog
sphinx of black quartz judge my vow
pack my box with five dozen liquor jugs
how vexingly quick daft zebras jump";
        let serial: usize = text.lines().map(|l| l.split_whitespace().count()).sum();
        let parallel = count_words_pool(lines(text), 4);
        assert_eq!(parallel, serial);
    }

    #[test] fn handles_blank_and_irregular_whitespace() {
        let input = lines("  one   two\n\nthree\t\tfour five\n");
        // split_whitespace collapses runs and trims edges:
        //   "  one   two"        -> 2
        //   ""                   -> 0
        //   "three\t\tfour five" -> 3
        //   ""                   -> 0
        assert_eq!(count_words_pool(input, 3), 5);
    }
}

fn main() {}
