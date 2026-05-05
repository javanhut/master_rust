// SOLUTION — box1_basics

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn boxed(x: i32) -> Box<i32> {
    Box::new(x)
}

pub fn unbox(b: Box<i32>) -> i32 {
    *b
}

pub fn from_slice(xs: &[i32]) -> List {
    let mut acc = List::Nil;
    for &x in xs.iter().rev() {
        acc = List::Cons(x, Box::new(acc));
    }
    acc
}

pub fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(x, rest) => x + sum_list(rest),
        List::Nil => 0,
    }
}

// WHY THIS IS OPTIMAL:
//
//   `Box::new(x)` is the only way to put `x` on the heap and get back the
//   owning pointer. There is no `Box::from_value` / `Box::with` — just
//   `new`. The allocator runs once; the inner value is then moved into
//   that allocation by the compiler.
//
//   `*b` for `unbox` works because `i32: Copy`, so dereferencing copies
//   the inner four bytes and lets the Box drop normally. For non-Copy
//   types you would write `*b` to MOVE the inner value out (legal because
//   you own the Box and `Box<T>` implements `DerefMut`/owning-deref via
//   the unstable `DerefMove` semantics — in stable Rust the compiler has
//   a special case that lets `*box_value` move the inner T when the Box
//   is owned). In practice you rarely need to move out — most APIs are
//   happy with `&*b` or just `&b` thanks to deref coercion.
//
//   `from_slice` builds the cons-list right-to-left. The reverse fold
//   makes a single pass and avoids the O(n²) cost of repeatedly walking
//   to the end to append a tail. Each `Box::new(acc)` boxes the previous
//   accumulator, becoming the tail of the new node.
//
//   `sum_list` recurses on `rest`, which has type `&Box<List>`. Thanks
//   to deref coercion, `sum_list(rest)` accepts a `&Box<List>` where a
//   `&List` is expected — `&Box<T>` -> `&T` is the canonical example of
//   coercion in action.
//
// ALTERNATIVES:
//
//   - You could write `from_slice` recursively:
//
//         fn from_slice(xs: &[i32]) -> List {
//             match xs.split_first() {
//                 Some((head, tail)) => List::Cons(*head, Box::new(from_slice(tail))),
//                 None => List::Nil,
//             }
//         }
//
//     Equivalent, slightly more elegant if you like recursion. The
//     iterative reverse-fold avoids stack growth on huge inputs.
//
//   - For `unbox` you could use `Box::into_inner(b)` — but it's nightly-
//     only at time of writing. `*b` is the stable, idiomatic way.
//
//   - For storage of homogeneous data, prefer `Vec<T>` over a hand-rolled
//     cons-list. Cons-lists are pedagogical here, not idiomatic Rust.
