// SOLUTION — enum2_match

enum Coin { Penny, Nickel, Dime, Quarter }

fn value_in_cents(c: &Coin) -> u32 {
    match c {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}

fn is_copper(c: &Coin) -> bool {
    match c {
        Coin::Penny => true,
        _ => false,
    }
}

// WHY THIS IS OPTIMAL:
//
//   `value_in_cents` enumerates EVERY variant explicitly — that's the
//   signal that says "if I add a half-dollar tomorrow, the compiler must
//   make me come back here". Skipping that with a `_` arm would silently
//   give half-dollars a value of 0.
//
//   `is_copper` is the opposite case: there's a single "interesting"
//   variant and the rest collapse to one answer. `_ => false` is the
//   idiomatic shape and it scales — adding a new coin gives the right
//   default automatically.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `matches!(c, Coin::Penny)` for `is_copper`
//     - Actually NICER for this exact case! `fn is_copper(c: &Coin) -> bool
//       { matches!(c, Coin::Penny) }` is one line. We wrote the explicit
//       match here to drill the wildcard pattern, but `matches!` is what
//       you'd reach for in real code.
//
//   if/else chains on enums
//     - You'd need `if let Coin::Penny = c { ... }` style. Loses
//       exhaustiveness. Don't.
//
// SUBTLETY:
//   The order of arms only matters when patterns OVERLAP (e.g. `_` must
//   come last, otherwise it shadows everything below it). For
//   non-overlapping enum variants, order is purely cosmetic.
