// =============================================================================
//  mod3 — visibility: pub, pub(crate), pub(super), and field privacy
// =============================================================================
//
// `pub` is not a single setting — it's a SCOPE you can dial in:
//
//     pub                — visible everywhere the parent is visible
//                          (effectively "exported from this crate" when
//                          applied at the crate root).
//
//     pub(crate)         — visible anywhere in the CURRENT crate, but NOT
//                          re-exported to other crates that depend on us.
//                          This is the workhorse for "internal helpers
//                          shared across the codebase."
//
//     pub(super)         — visible only to the parent module. A child says
//                          "my parent may use me, no one else."
//
//     pub(in path)       — visible within a specific module path. Rare;
//                          handy for tightly coupled cousin modules.
//
// (No `pub` at all = PRIVATE. Visible only inside the defining module and
// its descendants.)
//
// STRUCT FIELD PRIVACY
//
// `pub` on a struct makes the TYPE visible. Each FIELD's visibility is
// independent and defaults to private:
//
//     pub struct Account {
//         pub name:    String,    // anyone can read/write
//         balance:     i64,       // private — invisible outside the module
//     }
//
// Outside `Account`'s module you can:
//   - construct via constructors / builders   (NOT with the `Account { ... }`
//     literal, because that would touch `balance`)
//   - read / mutate `name` directly
//   - reach `balance` ONLY through methods the module chooses to provide.
//
// This is the classic ENCAPSULATION pattern: the struct controls its own
// invariants because nobody outside can poke at the private fields.
//
// THE GETTER PATTERN
//
//     impl Account {
//         pub fn new(name: String) -> Self { Self { name, balance: 0 } }
//         pub fn balance(&self) -> i64    { self.balance }      // read-only view
//         pub fn deposit(&mut self, n: i64) { self.balance += n; } // checked mutator
//     }
//
// The getter `balance(&self) -> i64` returns a Copy of the field — callers
// can read but not write. There is no public setter; mutation goes through
// `deposit`, which is the ONLY place to enforce rules (e.g., "no negative
// deposits"). That's the payoff of private fields.
//
// FILE-BASED NOTE
//
// In a real project this module would live at `src/accounts.rs`; the
// public API surface (`Account`, `Account::new`, `Account::balance`,
// `Account::deposit`) would be exactly what you see here. The `pub(crate)`
// helper would be reachable from any other file in the same crate but not
// from downstream users.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Inside `mod accounts`:
//
//   - `pub struct Account` with a `pub name: String` and a PRIVATE `balance: i64`.
//   - `pub fn new(name: String) -> Account` — start at balance 0.
//   - `pub fn balance(&self) -> i64` — getter returning the current balance.
//   - `pub fn deposit(&mut self, amount: i64)` — adds to balance.
//   - `pub(crate) fn audit_tag() -> &'static str` — returns "AUDIT". This is
//     visible elsewhere in our crate but NOT re-exported to downstream
//     users in a real library context.
//
// At the top level:
//
//   - `fn open(name: &str) -> accounts::Account` builds an account.
//   - `fn total_after_deposits(deposits: &[i64]) -> i64`:
//        opens an account named "user", deposits each amount, returns the
//        final balance via the getter.

// I AM NOT DONE

mod accounts {
    pub struct Account {
        pub name: String,
        // private — outside this module nobody may read OR write this directly.
        ???: i64,
    }

    impl Account {
        pub fn new(name: String) -> Account {
            Account { name, balance: ??? }
        }

        // Read-only window into the private field.
        pub fn balance(&self) -> i64 {
            self.???
        }

        // Mutator — the only sanctioned way to change `balance` from outside.
        pub fn deposit(&mut self, amount: i64) {
            self.balance ??? amount;
        }
    }

    // Visible across the crate but not part of the EXPORTED public API.
    ???(crate) fn audit_tag() -> &'static str {
        "AUDIT"
    }
}

fn open(name: &str) -> accounts::Account {
    accounts::Account::???(name.to_string())
}

fn total_after_deposits(deposits: &[i64]) -> i64 {
    let mut acct = open("user");
    for d in deposits {
        acct.???(*d);
    }
    acct.???()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn open_starts_at_zero() {
        let a = open("alice");
        assert_eq!(a.balance(), 0);
        assert_eq!(a.name, "alice");        // public field — direct access OK
    }

    #[test] fn deposits_accumulate() {
        assert_eq!(total_after_deposits(&[10, 20, 30]), 60);
    }

    #[test] fn audit_tag_visible_in_crate() {
        // pub(crate) means: usable from anywhere inside this crate,
        // including this test module.
        assert_eq!(accounts::audit_tag(), "AUDIT");
    }
}

fn main() {}
