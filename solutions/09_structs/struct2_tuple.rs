// SOLUTION — struct2_tuple

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(u64);

fn same_user(a: UserId, b: UserId) -> bool {
    a == b
}

fn order_for(user: UserId, n: u64) -> OrderId {
    OrderId(user.0 ^ n)
}

// WHY THIS IS OPTIMAL:
//
//   The newtype pattern is the textbook answer to "I keep mixing up these
//   two `u64`s". By making `UserId` and `OrderId` distinct types, the
//   compiler does the discipline for us — `same_user(some_order, ...)` will
//   not even compile.
//
//   Deriving `Copy` here is appropriate: an id is a small, immutable
//   identifier that should behave like a primitive number. Without `Copy`,
//   ergonomic things like `same_user(uid, uid)` would surprise newcomers
//   with a move error.
//
//   `user.0 ^ n` reaches into the wrapper, performs the bitwise XOR, and
//   `OrderId(...)` wraps the result back up. This is the canonical
//   "unwrap-compute-rewrap" shape.
//
// EQUIVALENT BUT NOISIER:
//
//   fn same_user(a: UserId, b: UserId) -> bool {
//       a.0 == b.0      // works, but `a == b` is shorter and reads better
//   }
//
//   fn order_for(user: UserId, n: u64) -> OrderId {
//       let UserId(raw) = user;     // destructure instead of `.0`
//       OrderId(raw ^ n)
//   }
//   The destructuring form is preferred when you reach inside multiple
//   times in the same function — naming the inner value once beats writing
//   `.0` repeatedly.
