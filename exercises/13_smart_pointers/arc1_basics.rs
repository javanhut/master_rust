// =============================================================================
//  arc1 — Arc<T>: ATOMIC reference counting (thread-safe Rc)
// =============================================================================
//
// `Rc<T>` and `Arc<T>` have nearly identical APIs:
//
//     Rc::new(v)         Arc::new(v)
//     Rc::clone(&rc)     Arc::clone(&arc)
//     Rc::strong_count   Arc::strong_count
//     Rc::weak_count     Arc::weak_count
//
// The difference is a single word: ATOMIC. `Arc<T>` increments and
// decrements its reference counter using atomic CPU instructions, so it
// is safe to send between threads. `Rc<T>` uses plain integer ops —
// faster, but a data race waiting to happen if you crossed thread
// boundaries with it. The compiler enforces the distinction:
//
//     Rc<T>  is !Send + !Sync   — cannot cross threads
//     Arc<T> is  Send +  Sync   (when T is Send + Sync)
//
// WHEN TO CHOOSE WHICH
//
//   - Single-threaded code, by default? `Rc<T>`. Slightly cheaper.
//   - Anything involving threads? `Arc<T>`. Always.
//
// You will not pay the atomic cost in normal app code — modern CPUs
// handle uncontended atomic increments in a few nanoseconds. The
// genuine cost is when many threads thrash the SAME atomic counter
// (heavy contention slows everyone down due to cache-line bouncing).
//
// MUTATION ACROSS THREADS — A PREVIEW
//
// `Arc<T>` alone gives shared READ access — `&T`. To mutate from
// multiple threads you pair it with a synchronization primitive:
//
//     Arc<Mutex<T>>      — most common, blocks on contention
//     Arc<RwLock<T>>     — separate read/write locks; many readers OR one writer
//
// `Mutex<T>` looks like a thread-safe `RefCell<T>`: `lock()` returns a
// guard you can deref to `&mut T`. We won't spawn threads in this
// exercise — concurrency is chapter 14. Just remember the pairing:
//
//     Rc<RefCell<T>>    — single-threaded shared mutability
//     Arc<Mutex<T>>     — multi-threaded shared mutability
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_shared(n)` wraps an `i32` in an `Arc`.
//   - `share(arc)` clones the Arc (bumps the atomic counter).
//   - `count(arc)` returns the strong count.
//   - `read(arc)` returns the inner i32 (it's Copy).

// I AM NOT DONE

use std::sync::Arc;

pub fn make_shared(n: i32) -> Arc<i32> {
    ???
}

pub fn share(arc: &Arc<i32>) -> Arc<i32> {
    // Same convention as Rc: prefer Arc::clone(&arc) for clarity.
    ???
}

pub fn count(arc: &Arc<i32>) -> usize {
    Arc::???(arc)
}

pub fn read(arc: &Arc<i32>) -> i32 {
    // Arc<i32> derefs to &i32; *arc copies the i32 out.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn make_then_count_one() {
        let a = make_shared(42);
        assert_eq!(count(&a), 1);
        assert_eq!(read(&a), 42);
    }

    #[test] fn share_increments() {
        let a = make_shared(7);
        let b = share(&a);
        assert_eq!(count(&a), 2);
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test] fn drop_decrements() {
        let a = make_shared(0);
        let b = share(&a);
        let c = share(&a);
        assert_eq!(count(&a), 3);
        drop(c);
        drop(b);
        assert_eq!(count(&a), 1);
    }

    #[test] fn arc_is_send_and_sync() {
        // Compile-time witness: Arc<i32> implements Send + Sync.
        // (Rc<i32> does NOT — replacing Arc with Rc here would fail to
        // compile.)
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Arc<i32>>();
    }
}

fn main() {}
