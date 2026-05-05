// =============================================================================
//  struct2 — tuple structs and the newtype pattern
// =============================================================================
//
// A TUPLE STRUCT is a struct whose fields have NO names — only positions:
//
//     struct Pair(i32, i32);
//
//     let p = Pair(3, 4);
//     p.0;   // 3
//     p.1;   // 4
//
// It looks like a tuple, but it has its own DISTINCT TYPE. A `Pair` and an
// `(i32, i32)` are not interchangeable, even though their layouts match.
//
// THE NEWTYPE PATTERN
//
// The single most useful flavour of tuple struct is the "newtype" — a
// one-field wrapper that exists purely to give a primitive its own type:
//
//     struct UserId(u64);
//     struct OrderId(u64);
//
// Now a function that takes a `UserId` cannot accidentally be called with an
// `OrderId`, even though both wrap a `u64`. The compiler will refuse the
// mistake. This is a zero-cost abstraction: at runtime the wrapper compiles
// down to exactly the underlying `u64`.
//
// To get the inner value back out, either:
//   - access by position:        `let n = id.0;`
//   - destructure in a pattern:  `let UserId(n) = id;`
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - Define `UserId(u64)` and `OrderId(u64)` as newtypes.
//   - `same_user(a, b) -> bool` returns true iff the two `UserId`s are equal.
//     (PartialEq is derived, so you can use `==`.)
//   - `order_for(user, n) -> OrderId` returns an OrderId built from the
//     user's id XOR'd with `n`. (Just a pretend "mixing" function — the
//     point is that the return type is `OrderId`, not `u64`.)

// I AM NOT DONE

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(???);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(???);

fn same_user(a: UserId, b: UserId) -> bool {
    ???
}

fn order_for(user: UserId, n: u64) -> OrderId {
    // Reach inside `user` with `.0`, mix it with `n` using `^`, wrap it back.
    OrderId(???)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn same_user_equal() {
        assert!(same_user(UserId(7), UserId(7)));
    }
    #[test] fn same_user_different() {
        assert!(!same_user(UserId(7), UserId(8)));
    }
    #[test] fn order_id_is_distinct_type() {
        let o = order_for(UserId(0b1100), 0b1010);
        assert_eq!(o, OrderId(0b0110));
    }
    #[test] fn order_zero_xor() {
        // x ^ 0 == x
        assert_eq!(order_for(UserId(42), 0), OrderId(42));
    }
}

fn main() {}
