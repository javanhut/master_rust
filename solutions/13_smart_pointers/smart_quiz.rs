// SOLUTION — smart_quiz

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<RefCell<Node>>>,
    pub children: RefCell<Vec<Rc<RefCell<Node>>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }))
    }

    pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        *child.borrow().parent.borrow_mut() = Rc::downgrade(parent);
        parent.borrow().children.borrow_mut().push(Rc::clone(child));
    }

    pub fn parent_value(node: &Rc<RefCell<Node>>) -> Option<i32> {
        let node_ref = node.borrow();
        let parent_weak = node_ref.parent.borrow();
        parent_weak.upgrade().map(|p| p.borrow().value)
    }
}

// WHY THIS IS OPTIMAL:
//
//   The shape `Rc<RefCell<Node>>` for owned handles + `Weak<RefCell<Node>>`
//   for the back-pointer is THE pattern for trees and graphs in safe
//   Rust. Every textbook tree, every doubly-linked list, every parent/
//   child DAG you see in the wild looks like this. Memorise it.
//
//   `Rc::downgrade(parent)` produces a `Weak<RefCell<Node>>` — it bumps
//   the WEAK count, not the strong count, so it doesn't keep the parent
//   alive. That's what breaks the cycle:
//
//        root  --strong-->  child            (parent owns child)
//        root  <--weak----  child.parent     (child observes parent)
//
//   When the only strong owner of root drops, root's strong count hits
//   zero and `Node` is dropped — even though child.parent still points
//   at the (now defunct) allocation. The control block sticks around
//   until the weak count also hits zero, but the actual `Node` value is
//   freed promptly.
//
//   `Weak::upgrade()` is the reverse of `Rc::downgrade`. It atomically
//   (well, non-atomically for `Rc::Weak`) bumps the strong count IF the
//   value is still alive, returning a fresh `Rc`. Otherwise it returns
//   `None`. That's why `parent_value` returns `Option<i32>`: the parent
//   may already be gone.
//
//   `add_child` does its two updates in the natural order. Note the
//   borrow gymnastics — each line takes `child.borrow()` (or
//   `parent.borrow()`) once, then drills into a field's RefCell with
//   borrow_mut(). The outer Ref guard drops at the semicolon, so the
//   borrows don't overlap problematically.
//
//   `parent_value` splits the borrow into two named bindings. We must
//   keep the outer `node_ref` alive while we hold the inner
//   `parent_weak`, because the latter borrows from the former. Calling
//   `.upgrade()` does NOT borrow either — it just inspects the weak
//   counter — so no further guard juggling is required.
//
// ANATOMY OF THE STRONG-COUNT TEST
//
//   After building (root -> a, b):
//     - `root` strong count: 1   (only the local `root` binding owns it;
//                                 children hold WEAK references back).
//     - `a` strong count: 2      (local `a` + root.children Vec entry).
//     - `b` strong count: 2      (same).
//
//   Drop `a`'s local binding: count goes from 2 -> 1 (still in Vec).
//   Drop `root`: root's count -> 0; root drops, freeing children Vec,
//   which drops a's and b's last strong handle, freeing the leaves.
//
// ALTERNATIVES & TRADE-OFFS:
//
//   1. Indices instead of pointers. The "arena" pattern uses
//
//          struct Tree { nodes: Vec<NodeData> }
//          struct NodeId(usize);
//
//      and stores parent/child IDs as plain `usize`. No Rc, no RefCell,
//      no Weak. Mutation is simple — `tree.nodes[id]`. The trade-off is
//      stale-ID risk and worse ergonomics for "give me my parent" — you
//      need the arena in scope. For perf-sensitive code (compilers,
//      ECS), arenas usually win.
//
//   2. `petgraph` or `slotmap` crates package the arena pattern with
//      better APIs. Reach for them once your data outgrows hand-rolled
//      pointers.
//
//   3. For thread-safe trees, swap `Rc` -> `Arc`, `Weak` -> `sync::Weak`,
//      `RefCell` -> `Mutex` or `RwLock`. The shape is identical.
//
// KEY TAKEAWAYS FROM CHAPTER 13:
//
//   - `Box<T>`            : single owner, heap, fixed-size pointer to T.
//   - `Box<dyn Trait>`    : owned trait object, heterogeneous storage.
//   - `Rc<T>`             : multiple owners, single-threaded, immutable.
//   - `RefCell<T>`        : interior mutability, runtime borrow checks.
//   - `Rc<RefCell<T>>`    : multiple owners that can mutate.
//   - `Arc<T>`            : Rc + atomic counter — safe across threads.
//   - `Arc<Mutex<T>>`     : multi-threaded shared mutability (chapter 14).
//   - `Weak<T>`           : non-owning observer; breaks Rc cycles.
//
//   Smart pointers compose. The shape of the wrapper stack tells you
//   the access discipline at a glance. Read them outside-in:
//   "Arc-of-Mutex-of-Vec-of-T" = "shared across threads, locked on
//   access, holds a growable list of T."
