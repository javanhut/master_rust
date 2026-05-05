// SOLUTION — vec3_mutate

fn push_zero(v: &mut Vec<i32>) {
    v.push(0);
}

fn pop_last(v: &mut Vec<i32>) -> Option<i32> {
    v.pop()
}

fn insert_front(v: &mut Vec<i32>, x: i32) {
    v.insert(0, x);
}

fn swap_remove_at(v: &mut Vec<i32>, i: usize) -> i32 {
    v.swap_remove(i)
}

fn keep_positives(v: &mut Vec<i32>) {
    v.retain(|x| *x > 0);
}

// WHY THESE ARE OPTIMAL:
//
//   `push` / `pop` work on the END of the Vec. They never move other
//   elements, so they're O(1) amortised. `pop` returning `Option<T>` saves
//   you from a separate "is empty?" branch.
//
//   `insert(0, x)` is the most expensive insert — every element shifts one
//   slot to the right. If you find yourself doing this in a hot loop, the
//   right answer is usually `VecDeque<T>`, which gives O(1) push_front /
//   pop_front. We'll meet VecDeque later; for now, knowing the cost is the
//   point.
//
//   `swap_remove(i)` is the killer feature when ORDER DOESN'T MATTER. It
//   takes the element at `i`, drops in the last element to fill the hole,
//   then shrinks the length by 1 — no shifting. This is the right tool for
//   "delete this entity from a flat array" in games, ECS, etc.
//
//   `retain` walks the Vec exactly once and rewrites the keepers in place.
//   Equivalent to a manual `for` loop with two indices, but you don't have
//   to write the bookkeeping.
//
// ALTERNATIVES:
//
//   - `v.truncate(n)` cuts the Vec down to the first n elements (drops the
//     rest). Cheaper than repeated `pop()`.
//   - `v.clear()` empties the Vec but keeps the allocation, ready for reuse.
//   - `v.drain(range)` removes a range and yields the removed values; useful
//     when you want to PROCESS what you're removing.
//
// COMMON PITFALL:
//
//   `retain` takes `|&T| -> bool` (the closure receives a reference). The
//   `*x > 0` deref is required, or use the pattern `|&x| x > 0`.
