// SOLUTION — rc1_basics

use std::rc::Rc;

pub fn make_shared(s: String) -> Rc<String> {
    Rc::new(s)
}

pub fn share(rc: &Rc<String>) -> Rc<String> {
    Rc::clone(rc)
}

pub fn count(rc: &Rc<String>) -> usize {
    Rc::strong_count(rc)
}

pub fn read(rc: &Rc<String>) -> String {
    // Method-resolution on `rc.clone()` would pick `Rc::clone` first (just
    // bumping the refcount), returning `Rc<String>`. We want a fresh owned
    // String, so call String::clone explicitly through the deref:
    String::clone(rc)
}

// WHY THIS IS OPTIMAL:
//
//   `Rc::new(s)` is the only constructor — it allocates a new control
//   block (strong=1, weak=1) and moves `s` into it.
//
//   `Rc::clone(rc)` is preferred over `rc.clone()` purely for SIGNALING.
//   They produce identical bytecode, but the explicit form tells the
//   reader "this is a cheap refcount bump", whereas `.clone()` looks
//   like the universal Clone op (which on a `String`, for example, would
//   deep-copy the buffer). The Rust API guidelines and the Rust Book
//   both recommend the explicit form for this reason.
//
//   `Rc::strong_count(rc)` is an associated function (not a method) so
//   it doesn't conflict with anything `T` might define. Inside, it
//   reads the (non-atomic) counter from the control block.
//
//   `String::clone(rc)` — note we call `String::clone` *explicitly* and
//   pass `rc` (a `&Rc<String>`). Deref coercion turns `&Rc<String>` into
//   `&String` for the call. We DON'T write `rc.clone()` here, because
//   method-resolution would pick `Rc::clone` first (returning another
//   `Rc<String>`, not a `String`) — exactly the kind of mix-up the
//   `Rc::clone(&rc)` convention is meant to prevent.
//
// WHAT IS A "STRONG" COUNT?
//
//   `Rc<T>` keeps two counts: STRONG (owners that prevent the value from
//   being dropped) and WEAK (non-owning observers). The value is dropped
//   when strong hits zero; the control block itself is freed when both
//   strong AND weak hit zero. Weak references show up in smart_quiz.
//
// ALTERNATIVES & PITFALLS:
//
//   - `Rc::ptr_eq(a, b)` — compare allocations, not values. Useful in
//     graph algorithms and the test above.
//
//   - DON'T pass `Rc` everywhere "just to be safe". If a function only
//     reads, take `&T` instead — fewer atomics, no refcount bumps. Reach
//     for `Rc<T>` when you need MULTIPLE INDEPENDENT OWNERS, e.g. a node
//     referenced from several other nodes.
//
//   - `Rc::try_unwrap(rc)` returns `Ok(T)` if you're the only owner, else
//     `Err(rc)`. Handy for "extract the value when nobody else holds it".
//
//   - For threads, swap `Rc` for `Arc` — same API, atomic counter.
