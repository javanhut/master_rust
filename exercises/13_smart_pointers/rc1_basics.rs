// =============================================================================
//  rc1 — Rc<T>: reference-counted shared ownership (single-threaded)
// =============================================================================
//
// `Box<T>` has ONE owner. When the Box drops, the heap allocation is
// freed. That works for most data, but sometimes you want SEVERAL
// places to keep a value alive — and you don't know in advance which
// will be the last to release it. That's what `Rc<T>` is for.
//
//     use std::rc::Rc;
//
//     let a = Rc::new(String::from("hello"));
//     let b = Rc::clone(&a);          // bumps the strong count, no deep copy
//     let c = Rc::clone(&a);
//     // count is now 3
//     drop(b);                        // count is 2
//     drop(c);                        // count is 1
//     // when `a` drops, count hits 0 and the String is freed
//
// HOW IT WORKS
//
// `Rc<T>` is a pointer to a heap allocation that contains:
//   - a strong reference count (usize),
//   - a weak reference count (usize, see smart_quiz),
//   - the value `T`.
//
// `Rc::clone(&rc)` does NOT copy `T`. It increments the strong counter
// and hands you another pointer to the same allocation. That's why it's
// CHEAP — one increment, no allocation, no T-copy. Use the explicit
// `Rc::clone(&rc)` form (not `rc.clone()`) so readers can immediately
// see "this is a reference-count bump, not a deep clone."
//
// IMMUTABLE BY DEFAULT
//
// An `Rc<T>` only gives you `&T` access — you cannot mutate the inner
// value through a clone (multiple owners couldn't agree on writes).
// To mutate shared data, combine with `RefCell<T>` (next two exercises)
// or, in multi-threaded code, with `Mutex<T>` (chapter 14).
//
// NOT THREAD-SAFE
//
// `Rc<T>` increments its counter NON-ATOMICALLY for speed. Sending an
// `Rc` between threads would race on the counter. The compiler refuses:
// `Rc<T>` is `!Send + !Sync`. Use `Arc<T>` for threads (arc1_basics).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_shared(s)` wraps `s: String` in an `Rc`.
//   - `share(rc)` returns another `Rc` pointing to the same allocation
//     (the strong count goes up by one).
//   - `count(rc)` returns the current strong count.
//   - `read(rc)` returns a clone of the inner String for inspection.

// I AM NOT DONE

use std::rc::Rc;

pub fn make_shared(s: String) -> Rc<String> {
    ???
}

pub fn share(rc: &Rc<String>) -> Rc<String> {
    // Use Rc::clone(&rc) — NOT rc.clone() — so the intent is loud.
    ???
}

pub fn count(rc: &Rc<String>) -> usize {
    Rc::???(rc)
}

pub fn read(rc: &Rc<String>) -> String {
    // Tricky: `rc.clone()` resolves to `Rc::clone` (refcount bump), which
    // returns `Rc<String>` — wrong type. Call `String::clone` explicitly,
    // passing the &Rc; deref coercion turns &Rc<String> into &String.
    String::???(rc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn make_then_count_one() {
        let a = make_shared("hi".into());
        assert_eq!(count(&a), 1);
        assert_eq!(read(&a), "hi");
    }

    #[test] fn share_bumps_count() {
        let a = make_shared("hi".into());
        let b = share(&a);
        assert_eq!(count(&a), 2);
        assert_eq!(count(&b), 2);   // both view the same counter
        assert_eq!(read(&b), "hi");
    }

    #[test] fn drop_decrements_count() {
        let a = make_shared("hi".into());
        let b = share(&a);
        let c = share(&a);
        assert_eq!(count(&a), 3);
        drop(b);
        assert_eq!(count(&a), 2);
        drop(c);
        assert_eq!(count(&a), 1);
    }

    #[test] fn shared_data_is_the_same_allocation() {
        let a = make_shared("rust".into());
        let b = share(&a);
        // Rc::ptr_eq compares the inner allocation pointer, not the value.
        assert!(Rc::ptr_eq(&a, &b));
    }
}

fn main() {}
