// =============================================================================
//  iter2 — `map`, `filter`, and the laziness rule
// =============================================================================
//
// Two adapters do most of the work in real iterator chains:
//
//     .map(|x| transform(x))       // change each element
//     .filter(|x| keep_it(x))      // drop elements where the closure is false
//
// They are ADAPTERS — they wrap an existing iterator and return a new one.
// They DO NOT run anything yet.
//
// THE LAZINESS RULE
//
// An iterator chain does no work until a TERMINATOR is called. Terminators
// are methods that drive `next` to extract a final answer:
//
//     .collect()    .sum()    .product()    .count()
//     .for_each()   .fold()   .last()       .max()
//
// Without one of those, the chain is just sitting there describing work
// that will never happen.
//
//     let _ = (1..=3).map(|x| { println!("hi {}", x); x });
//     // prints NOTHING — `map` was never driven.
//
//     let v: Vec<i32> = (1..=3).map(|x| x * 10).collect();
//     // prints nothing because the closure has no side effects, but `collect`
//     // forced every element through the `map` closure. v == [10, 20, 30].
//
// FILTER RECEIVES A REFERENCE
//
// The closure signature for `filter` is `FnMut(&Self::Item) -> bool`. So if
// the iterator yields `i32`, the closure parameter is `&i32`:
//
//     (1..=10).filter(|&x| x % 2 == 0)        // pattern-match through &
//     (1..=10).filter(|x| *x % 2 == 0)        // explicit deref
//
// If the iterator yields `&i32` (e.g. from `slice.iter()`), the closure gets
// `&&i32` — hence the `|&&x|` pattern you'll see a lot.
//
// ORDER OF OPERATIONS
//
// `.filter(...).map(...)` — filter first, then transform. Faster: only
// surviving items pay the map cost.
// `.map(...).filter(...)` — also legal, sometimes necessary, occasionally
// wasteful. Pick what reads.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `doubled(xs)`: return a Vec where every element of `xs` has been
//     multiplied by 2. Use `.iter().copied().map(...).collect()`.
//   - `evens(xs)`: return a Vec containing only the even numbers from `xs`,
//     in order.
//   - `lazy_does_nothing()`: this function must compile. Build a chain of
//     `.map(...)` over `1..=1_000_000_000` that WOULD panic if it ran, and
//     just drop it without a terminator. Returns `()`. The point: prove to
//     yourself that `map` on its own never executes.

// I AM NOT DONE

fn doubled(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().???(|x| ???).collect()
}

fn evens(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().???(|&x| ???).collect()
}

fn lazy_does_nothing() {
    // This `map` body would explode if it ever ran. Build it anyway. Because
    // we never call a terminator, NOTHING happens.
    let _ = (1u64..=1_000_000_000).???(|x| panic!("never runs! x={}", x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn doubled_basic() { assert_eq!(doubled(&[1, 2, 3]), vec![2, 4, 6]); }
    #[test] fn doubled_empty() { assert_eq!(doubled(&[]),        Vec::<i32>::new()); }
    #[test] fn doubled_neg()   { assert_eq!(doubled(&[-1, 0, 4]), vec![-2, 0, 8]); }

    #[test] fn evens_basic() { assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]); }
    #[test] fn evens_none()  { assert_eq!(evens(&[1, 3, 5]),          Vec::<i32>::new()); }
    #[test] fn evens_empty() { assert_eq!(evens(&[]),                 Vec::<i32>::new()); }

    #[test] fn lazy_runs() {
        // If the chain in `lazy_does_nothing` were eager, this test would panic.
        lazy_does_nothing();
    }
}

fn main() {}
