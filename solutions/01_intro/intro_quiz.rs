// SOLUTION — intro_quiz

const PRICE_PER_TICKET:        u32 = 12;
const TICKETS_PER_FREE_POPCORN: u32 = 5;

fn till(tickets_sold: &str) -> (u32, u32) {
    let tickets_sold: u32 = tickets_sold.parse().unwrap();
    let revenue        = tickets_sold * PRICE_PER_TICKET;
    let free_popcorn   = tickets_sold / TICKETS_PER_FREE_POPCORN;
    (revenue, free_popcorn)
}

// WHY THIS IS OPTIMAL:
//   - Two `const`s give the magic numbers names and a single source of
//     truth. Renaming or changing the price is a one-line edit.
//   - SHADOWING converts `tickets_sold` from `&str` to `u32`. After that
//     line the original `&str` is no longer reachable, which is exactly
//     what you want — the rest of the function only deals with numbers.
//   - No `mut` anywhere: every binding gets its final value at the moment
//     it's introduced. That's a strong signal that this code is easy to
//     reason about.
//
// EVERY DECISION, JUSTIFIED:
//   - revenue is `u32 * u32` → `u32`. Test cases stay in range.
//   - free_popcorn uses INTEGER division on purpose — 7 tickets gives 1
//     token, not 1.4. Integer division is the natural fit here.
//   - Returning a tuple `(u32, u32)` documents both outputs at the
//     signature level. In a larger system you'd promote this to a struct.
