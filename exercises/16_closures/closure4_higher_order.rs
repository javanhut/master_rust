// =============================================================================
//  closure4 — taking closures as parameters (higher-order functions)
// =============================================================================
//
// A function that takes another function (or closure) as input is called a
// HIGHER-ORDER function. Rust gives you several ways to spell the parameter:
//
//     // 1. Generic with a trait bound — the classic form:
//     fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
//
//     // 2. Same idea, expressed with `where`:
//     fn apply<F>(f: F, x: i32) -> i32
//     where
//         F: Fn(i32) -> i32,
//     { f(x) }
//
//     // 3. `impl Trait` in argument position — pure sugar for #1:
//     fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }
//
//     // 4. Trait object — dynamic dispatch, the closure is heap-stored:
//     fn apply(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }
//
// Forms 1, 2, and 3 are all STATIC DISPATCH: the compiler creates a
// specialized copy of `apply` for each closure type passed in.
//
// Form 4 is DYNAMIC DISPATCH: one copy of `apply`, calls go through a
// vtable — useful when you have many call sites with many closure types
// and want to keep code size down.
//
// WHEN TO USE `where`:
//
// `where` clauses are usually clearer when:
//   - you have multiple bounds (`F: Fn(i32) -> i32 + Send + 'static`),
//   - you have multiple type parameters,
//   - or the bounds are long enough to wreck the function signature line.
//
// For one short bound, the inline `<F: Fn(...)>` form is fine.
//
// PICKING THE RIGHT TRAIT (recap from closure2):
//
//     Fn       — call through &F. Read-only captures or none.
//     FnMut    — call through &mut F. Closure may mutate captures.
//                The parameter must be `mut f: F`.
//     FnOnce   — call through F (consumes). Closure may move captures out.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - `apply_twice<F>(f, x)` — generic + where-clause form. F: Fn(i32)->i32.
//     Returns f(f(x)).
//
//   - `transform(slice, op)` — uses `impl Fn` in argument position.
//     Returns a new Vec<i32> where each element is `op(element)`.
//     (Iterators were chapter 7; using `.iter().map(...).collect()` is
//      fine — the focus here is the closure-parameter syntax.)
//
//   - `repeat(n, mut f)` — takes an `FnMut()` closure and calls it `n`
//     times. Returns nothing.

// I AM NOT DONE

fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: ???,
{
    f(f(x))
}

fn transform(slice: &[i32], op: impl ???) -> Vec<i32> {
    slice.iter().map(|&n| op(n)).collect()
}

fn repeat<F: ???>(n: u32, mut f: F) {
    for _ in 0..n {
        f();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn twice_increment() {
        assert_eq!(apply_twice(|x| x + 1, 5), 7);
    }

    #[test] fn twice_square() {
        assert_eq!(apply_twice(|x| x * x, 3), 81); // 3^2 = 9, 9^2 = 81
    }

    #[test] fn transform_doubles() {
        let v = transform(&[1, 2, 3, 4], |n| n * 2);
        assert_eq!(v, vec![2, 4, 6, 8]);
    }

    #[test] fn transform_with_capture() {
        let bias = 10;
        let v = transform(&[1, 2, 3], |n| n + bias);
        assert_eq!(v, vec![11, 12, 13]);
    }

    #[test] fn repeat_counts() {
        let mut hits = 0;
        repeat(4, || hits += 1);
        assert_eq!(hits, 4);
    }

    #[test] fn repeat_zero() {
        let mut hits = 0;
        repeat(0, || hits += 1);
        assert_eq!(hits, 0);
    }
}

fn main() {}
