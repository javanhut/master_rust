// =============================================================================
//  intro_quiz — chapter 1 capstone
// =============================================================================
//
// This quiz uses every tool from chapter 1: `let`, `let mut`, shadowing,
// `const`, and type annotations. Take your time — try to predict what the
// compiler will say BEFORE you save.
//
// You are implementing a small "ticket counter" used by a cinema:
//
//   - PRICE_PER_TICKET is fixed and global.
//   - The function takes a string with the number of tickets sold,
//     parses it, computes the total revenue, and returns that revenue.
//   - It also returns how many "free popcorn" tokens to issue —
//     one for every TICKETS_PER_FREE_POPCORN tickets.
//
// =============================================================================

// I AM NOT DONE

// TASK A — declare the two constants. Both are u32.
const PRICE_PER_TICKET: u32 = ???;
const TICKETS_PER_FREE_POPCORN: u32 = ???;

// TASK B — implement the function.
//
// `tickets_sold` arrives as text (e.g. "27").
// Return  (total_revenue, free_popcorn_tokens).
fn till(tickets_sold: &str) -> (u32, u32) {
    // Parse the input. SHADOW the `tickets_sold` binding so the same name
    // refers to a `u32` from now on. (Use a type annotation, not turbofish.)
    let tickets_sold: u32 = tickets_sold.???;

    // Revenue: simple multiplication, no mutation needed — use `let` and
    // let the result be the binding's value.
    let revenue = ???;

    // Number of free-popcorn tokens — integer division.
    let free_popcorn = tickets_sold / ???;

    (revenue, free_popcorn)
}

// =============================================================================
// SET YOUR CONSTANTS SO THESE PASS:
//
//   - 1 ticket costs 12.
//   - 1 free popcorn for every 5 tickets.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn no_tickets()        { assert_eq!(till("0"),  (  0, 0)); }
    #[test] fn one_ticket()        { assert_eq!(till("1"),  ( 12, 0)); }
    #[test] fn five_tickets()      { assert_eq!(till("5"),  ( 60, 1)); }
    #[test] fn dozen_tickets()     { assert_eq!(till("12"), (144, 2)); }
    #[test] fn many_tickets()      { assert_eq!(till("100"), (1200, 20)); }
}

fn main() {}
