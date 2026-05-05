// SOLUTION — mod3_visibility

mod accounts {
    pub struct Account {
        pub name: String,
        balance: i64,
    }

    impl Account {
        pub fn new(name: String) -> Account {
            Account { name, balance: 0 }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: i64) {
            self.balance += amount;
        }
    }

    pub(crate) fn audit_tag() -> &'static str {
        "AUDIT"
    }
}

fn open(name: &str) -> accounts::Account {
    accounts::Account::new(name.to_string())
}

fn total_after_deposits(deposits: &[i64]) -> i64 {
    let mut acct = open("user");
    for d in deposits {
        acct.deposit(*d);
    }
    acct.balance()
}

// WHY THIS IS OPTIMAL:
//
//   `name` is `pub` because it's harmless to read or change directly — there
//   is no invariant tied to it. `balance` is private because every change
//   to it must go through `deposit`. That's where you'd later add rules
//   like "amount must be positive" or "log every transaction." Private
//   fields are how a struct earns the right to enforce its own contract.
//
//   The getter `balance(&self) -> i64` returns the value by Copy (i64 is
//   Copy), which gives callers a read-only snapshot. Returning `&i64`
//   would also work but adds noise; returning `&mut i64` would defeat the
//   whole encapsulation we just set up.
//
//   `pub(crate) fn audit_tag()` is the most common non-`pub` visibility in
//   real codebases: an internal helper shared between sibling modules,
//   never exposed to downstream users. In a single-file teaching example
//   it behaves like `pub`, but in a library crate the difference is
//   load-bearing — `pub(crate)` items don't appear in `cargo doc` and
//   can't be named by `extern crate` users.
//
// EQUIVALENT BUT WEAKER:
//
//   pub struct Account { pub name: String, pub balance: i64 }
//     — works, but anyone can write `acct.balance = -1_000_000;` and the
//       struct can't stop them. Encapsulation is gone.
//
//   pub fn balance(&self) -> &i64 { &self.balance }
//     — fine for non-Copy types where you really want a reference. For
//       i64 it's needless indirection.
//
// FILE-BASED EQUIVALENT:
//
//   In a multi-file crate this would be `src/accounts.rs`, declared in
//   `lib.rs` (or `main.rs`) as `pub mod accounts;`. The struct, its
//   methods, and `audit_tag()` look exactly the same — only the source
//   text moves into its own file.
