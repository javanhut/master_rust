// =============================================================================
//  box1 — Box<T>: heap allocation with single ownership
// =============================================================================
//
// Most values in Rust live on the STACK. The stack is fast and bounded —
// every function frame has a fixed size known at compile time. When you
// write
//
//     let n: i32 = 42;
//     let p: (i32, i32) = (3, 4);
//
// those four bytes (and eight bytes) live right inside the current frame.
//
// `Box<T>` is the simplest SMART POINTER. It allocates a `T` on the HEAP
// and stores a single-word pointer to it on the stack. The pointer OWNS
// the heap allocation: when the Box is dropped, the heap memory is freed
// automatically. There is no garbage collector — `Drop` runs the moment
// the Box goes out of scope.
//
//     let b: Box<i32> = Box::new(42);   // 42 lives on the heap
//     println!("{}", *b);               // explicit deref
//     println!("{}", b);                // works too — Display is forwarded
//     // b is dropped here; the heap allocation is freed
//
// THREE REASONS YOU REACH FOR BOX<T>
//
//   1. The value is BIG and you want to move it cheaply. Moving a Box
//      copies one pointer; moving the inner value would memcpy all of it.
//
//   2. The size of the value is NOT KNOWN AT COMPILE TIME — for example,
//      a recursive type. The classic case is a cons-list:
//
//          enum List {
//              Cons(i32, Box<List>),   // <-- Box breaks the cycle in size
//              Nil,
//          }
//
//      Without the Box, `List` would contain a `List` directly, and the
//      compiler can't compute a finite size for it (E0072: "recursive
//      type has infinite size"). A `Box<List>` is just a pointer — fixed
//      size — so the recursion terminates at the type level.
//
//   3. You need a TRAIT OBJECT (`Box<dyn Trait>`). Covered in box2.
//
// DEREF COERCION
//
// Box<T> implements `Deref<Target = T>`, which means `&Box<T>` coerces
// to `&T` automatically wherever a `&T` is expected. So a function
// taking `&i32` will happily accept `&b` for `b: Box<i32>`, and you can
// call methods of `T` directly on a `Box<T>` without writing `(*b).foo()`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `boxed(x)` — put `x` on the heap and return the Box.
//   - `unbox(b)` — take a `Box<i32>` and return the i32 inside (move it out).
//   - Finish the cons-list builder `from_slice` so it produces:
//       Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
//     for the input `&[1, 2, 3]`.
//   - `sum_list` walks the list and returns the sum of its values.

// I AM NOT DONE

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn boxed(x: i32) -> Box<i32> {
    ???
}

pub fn unbox(b: Box<i32>) -> i32 {
    // Hint: dereferencing a Box moves the inner value out (when T: !Copy
    // you'd need *b only inside an owning context). For i32 (Copy), `*b`
    // works directly.
    ???
}

pub fn from_slice(xs: &[i32]) -> List {
    // Build right-to-left: start from Nil and wrap each element on the front.
    let mut acc = List::Nil;
    for &x in xs.iter().rev() {
        acc = List::Cons(x, Box::new(???));
    }
    acc
}

pub fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(x, rest) => x + sum_list(???),
        List::Nil => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn boxed_holds_value() {
        let b = boxed(7);
        assert_eq!(*b, 7);
    }

    #[test] fn unbox_returns_inner() {
        assert_eq!(unbox(Box::new(99)), 99);
    }

    #[test] fn from_slice_empty() {
        assert_eq!(from_slice(&[]), List::Nil);
    }

    #[test] fn from_slice_builds_cons() {
        let list = from_slice(&[1, 2, 3]);
        let expected = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(3, Box::new(List::Nil))),
            )),
        );
        assert_eq!(list, expected);
    }

    #[test] fn sum_list_works() {
        assert_eq!(sum_list(&from_slice(&[1, 2, 3, 4])), 10);
        assert_eq!(sum_list(&List::Nil), 0);
    }

    #[test] fn deref_coercion_to_ref() {
        // &Box<i32> coerces to &i32 — a function taking &i32 accepts &b.
        fn takes_ref(n: &i32) -> i32 { *n + 1 }
        let b = boxed(41);
        assert_eq!(takes_ref(&b), 42);
    }
}

fn main() {}
