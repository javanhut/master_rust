// SOLUTION — own5_mut_borrow

fn append_bang(s: &mut String) {
    s.push('!');
}

fn double_in_place(n: &mut i32) {
    *n = *n * 2;
}

fn swap_first_two(v: &mut Vec<i32>) {
    v.swap(0, 1);
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `s.push('!')` mutates a `String` in place — it's the cheapest way to
//   tack a single char on the end. (For a whole string you'd use
//   `.push_str("...")`.) Through the `&mut String` we can call any method
//   that takes `&mut self`.
//
//   `*n = *n * 2` makes both the READ and the WRITE through the reference
//   explicit. Equivalently: `*n *= 2;` — sugar for the compound-assign.
//
//   `v.swap(0, 1)` is the standard slice helper for swapping by index. It
//   handles the borrow choreography internally — writing it yourself would
//   need `std::mem::swap(&mut v[0], &mut v[1])`, which the borrow checker
//   rightly rejects (you can't have two mut borrows into the same Vec at
//   once at the index level). `Vec::swap` exists precisely for this case.
//
// THE BORROW-CHECKER RULES IN ONE PARAGRAPH:
//
//   For any value, at any given moment, you can have either:
//     - any number of `&T` (shared / read-only),  OR
//     - exactly ONE `&mut T` (exclusive / read-write).
//   Never both. Never two `&mut`. The compiler enforces this purely
//   statically, with no runtime cost. That's how Rust prevents data races.
//
// COMMON COMPILE ERRORS YOU'LL HIT:
//
//   error[E0596]: cannot borrow `s` as mutable, as it is not declared
//                 as mutable
//       → you forgot `let mut s = ...` at the call site.
//
//   error[E0502]: cannot borrow `v` as mutable because it is also
//                 borrowed as immutable
//       → a `&v[..]` is still alive when you tried `&mut v`. Use the
//         shared borrow before the mutable one and let NLL end it.
//
//   error[E0499]: cannot borrow `*v` as mutable more than once at a time
//       → you tried to take two `&mut` to the same value simultaneously.
//
// Memorise those error codes — you'll meet them often.
