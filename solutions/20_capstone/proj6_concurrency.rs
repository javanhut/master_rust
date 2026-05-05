// SOLUTION — proj6_concurrency

use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl Stats {
    pub fn new() -> Self { Stats { lines: 0, words: 0, chars: 0 } }
    pub fn merge(self, other: Stats) -> Stats {
        Stats {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            chars: self.chars + other.chars,
        }
    }
}

pub fn count_basic(text: &str) -> Stats {
    Stats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
    }
}

pub fn count_parallel(chunks: &[&str]) -> Stats {
    thread::scope(|s| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| s.spawn(move || count_basic(chunk)))
            .collect();

        let mut total = Stats::new();
        for h in handles {
            total = total.merge(h.join().unwrap());
        }
        total
    })
}

// WHY THIS IS OPTIMAL:
//
//   `thread::scope` is the modern idiom for "fan out, then join". The
//   closure passed to `scope` receives a `&Scope`, which has its own
//   `.spawn(...)` that returns a `ScopedJoinHandle`. The compiler
//   guarantees every scoped handle is joined before `scope` returns,
//   so you can borrow data from the parent stack — note `chunk` here
//   is `&&str` captured by value into each closure, which Copies.
//
//   We collect the handles into a Vec FIRST and only then start
//   joining. If we joined inside the map, we'd serialise the work —
//   the first thread would have to finish before the second even
//   started.
//
//   `Stats` is `Copy`, and `merge` takes `self` by value — addition
//   on `usize` is the simplest possible reduce step. No locks, no
//   atomics, just send the partial result back through the join handle.
//
// ALTERNATIVES:
//
//   1. `Arc<Mutex<Stats>>` shared across threads, mutated in place.
//      WORKS but pointless: contention on the lock would dominate. The
//      "split-merge" pattern (each thread owns its result, merge at
//      the end) is the canonical concurrency idiom and avoids all
//      synchronisation cost.
//
//   2. Use the `rayon` crate's parallel iterators:
//
//          chunks.par_iter().map(count_basic).reduce(Stats::new, Stats::merge)
//
//      One line. For real code, use rayon. We hand-rolled to keep the
//      course std-only.
//
//   3. `std::thread::spawn` (non-scoped) — would force `'static`
//      bounds, meaning every chunk would have to be cloned into an
//      `Arc<String>` or similar before the thread could touch it.
//      Older Rust code did exactly this; `thread::scope` removes the
//      ceremony.
//
//   4. CHUNK SIZE TRADE-OFF. Spawning a thread per `&str` is fine for
//      a handful of chunks. For huge inputs you'd split into roughly
//      `num_cpus` chunks; per-thread fixed cost is non-trivial
//      (microseconds). A worker pool with a queue of chunks beats
//      "thread per chunk" once chunks are small.
//
// KEY TAKEAWAYS:
//
//   - Scoped threads (1.63+) make borrowing into worker threads safe
//     and ergonomic. Reach for them by default.
//   - "Split work, accumulate locally, merge at the end" beats
//     "share a counter behind a Mutex" almost always.
//   - The `Copy` on `Stats` is doing real work here — every join
//     literally moves a 24-byte value out of the worker thread.
