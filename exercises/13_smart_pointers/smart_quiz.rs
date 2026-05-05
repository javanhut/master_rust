// =============================================================================
//  smart_quiz — chapter 13 capstone: a tree with parent pointers
// =============================================================================
//
// You will build a small tree where each node knows its CHILDREN and its
// PARENT. The shape is the canonical "graph in safe Rust" pattern:
//
//     Node {
//         value:    i32,
//         parent:   RefCell<Weak<RefCell<Node>>>,    // upward link, NON-OWNING
//         children: RefCell<Vec<Rc<RefCell<Node>>>>, // downward, OWNING
//     }
//
// Why these specific wrappers:
//
//   - `Rc<RefCell<Node>>` for children — multiple references to a node
//     (the parent's children Vec, plus any other handles you keep) can
//     all OWN the node, and through the RefCell they can mutate it.
//
//   - `RefCell<Vec<Rc<RefCell<Node>>>>` for the children FIELD — we
//     need to add children AFTER constructing the node, through a
//     shared Rc, so the Vec itself must support interior mutability.
//
//   - `Weak<RefCell<Node>>` for the parent — a NON-OWNING reference.
//
//     Why weak? If parent stored an `Rc<RefCell<Node>>`, we'd have a
//     CYCLE: parent owns child, child owns parent. Refcounts never hit
//     zero, the allocation never frees. That's a reference-count leak.
//
//     `Weak<T>` points to the same allocation but doesn't contribute to
//     the strong count. To use a Weak you call `.upgrade()`, which
//     returns `Option<Rc<T>>` — `Some` if the value is still alive,
//     `None` if it has been dropped. Cycles are broken because at least
//     one direction is weak.
//
//   - The parent FIELD is wrapped in `RefCell<...>` so we can SET the
//     parent after-the-fact (when we attach a child to its parent).
//
// THIS IS THE PATTERN you'll see in every Rust tree, doubly-linked
// list, and graph implementation in pure safe code. It looks heavy at
// first; once you've built one tree you'll recognise it instantly.
//
// API YOU IMPLEMENT
//
//     Node::new(value)               -> Rc<RefCell<Node>>
//     Node::add_child(parent, child) — link `child`'s parent to `parent`,
//                                       then push `child` into parent's
//                                       children Vec.
//     Node::parent_value(node)       -> Option<i32>
//                                       Some(parent.value) if alive, else None.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Fill in the four `???`s below.

// I AM NOT DONE

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
            // A "dangling" Weak — points to nothing yet. Created with
            // Weak::new(); upgrade()-ing it returns None until we wire
            // the parent in.
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }))
    }

    /// Attach `child` to `parent`. Two updates:
    ///   1. set child's parent slot to a Weak handle of `parent`
    ///   2. push child (cloned Rc) into parent's children Vec
    pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        // Step 1 — replace child's parent with a downgrade of `parent`.
        // Hint: Rc::downgrade(rc) returns a Weak<T> pointing at the same
        // allocation. Then store it in the child's parent RefCell.
        *child.borrow().parent.borrow_mut() = Rc::???(parent);

        // Step 2 — push a clone of `child` into parent's children list.
        parent.borrow().children.borrow_mut().push(Rc::???(child));
    }

    /// Return the value of `node`'s parent if it is still alive.
    pub fn parent_value(node: &Rc<RefCell<Node>>) -> Option<i32> {
        // node.borrow().parent.borrow() is &Weak<RefCell<Node>>.
        // Call .upgrade() on it — Option<Rc<RefCell<Node>>>.
        // Then read .borrow().value.
        let parent_weak = node.borrow();
        let parent_weak = parent_weak.parent.borrow();
        parent_weak.???().map(|p| p.borrow().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_simple_tree() -> (Rc<RefCell<Node>>, Rc<RefCell<Node>>, Rc<RefCell<Node>>) {
        // root
        // ├── a
        // └── b
        let root = Node::new(0);
        let a = Node::new(1);
        let b = Node::new(2);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        (root, a, b)
    }

    #[test] fn root_has_no_parent() {
        let root = Node::new(10);
        assert_eq!(Node::parent_value(&root), None);
    }

    #[test] fn children_are_attached() {
        let (root, a, b) = build_simple_tree();
        let kids = root.borrow().children.borrow().len();
        assert_eq!(kids, 2);
        assert_eq!(a.borrow().value, 1);
        assert_eq!(b.borrow().value, 2);
    }

    #[test] fn parent_pointers_resolve() {
        let (root, a, b) = build_simple_tree();
        assert_eq!(Node::parent_value(&a), Some(0));
        assert_eq!(Node::parent_value(&b), Some(0));
        // root's value via the root handle directly:
        assert_eq!(root.borrow().value, 0);
    }

    #[test] fn mutate_a_leaf() {
        let (_root, a, _b) = build_simple_tree();
        a.borrow_mut().value = 99;
        assert_eq!(a.borrow().value, 99);
    }

    #[test] fn parent_dropped_means_upgrade_fails() {
        // If the only strong handle to the parent is dropped, the child's
        // Weak parent pointer can no longer upgrade.
        let leaf = Node::new(5);
        {
            let parent = Node::new(10);
            Node::add_child(&parent, &leaf);
            assert_eq!(Node::parent_value(&leaf), Some(10));
            // `parent` goes out of scope here. The leaf doesn't own it.
        }
        // Parent allocation has been freed because the only strong owner
        // (the local `parent`) was dropped. Weak::upgrade returns None.
        assert_eq!(Node::parent_value(&leaf), None);
    }

    #[test] fn strong_counts_are_sane() {
        let (root, a, b) = build_simple_tree();
        // root is owned by: the local `root` binding only — children hold
        // weak refs back to it. Strong count = 1.
        assert_eq!(Rc::strong_count(&root), 1);
        // each child is owned by: the local `a`/`b` binding + the root's
        // children Vec. Strong count = 2.
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 2);
    }
}

fn main() {}
