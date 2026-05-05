// SOLUTION — enum_quiz (vending machine)

#[derive(Debug, Clone, Copy, PartialEq)]
enum Drink { Cola, Water, Coffee }

#[derive(Debug, Clone, Copy, PartialEq)]
enum VendingMachine {
    Idle,
    Selected(Drink),
    Dispensing,
}

impl VendingMachine {
    fn insert(self, d: Drink) -> VendingMachine {
        match self {
            VendingMachine::Idle => VendingMachine::Selected(d),
            other => other,
        }
    }

    fn dispense(self) -> VendingMachine {
        match self {
            VendingMachine::Selected(_) => VendingMachine::Dispensing,
            other => other,
        }
    }

    fn finish(self) -> VendingMachine {
        match self {
            VendingMachine::Dispensing => VendingMachine::Idle,
            other => other,
        }
    }

    fn current_drink(&self) -> Option<Drink> {
        match self {
            VendingMachine::Selected(d) => Some(*d),
            _ => None,
        }
    }
}

// WHY THIS IS OPTIMAL:
//
//   Each transition method takes `self` BY VALUE and returns the next
//   state. This pattern — "owned old state in, owned new state out" —
//   makes invalid transitions structurally impossible: the old value
//   is consumed, so the caller can't accidentally hold onto a stale
//   reference. The reassign idiom `let m = m.insert(d);` shadows
//   cleanly.
//
//   The "ignore invalid transitions" arm is `other => other`. By naming
//   the rest `other` we get exhaustiveness (Rust verifies we covered
//   every variant) AND we can return it unchanged in one line.
//
//   `current_drink` takes `&self` because it only inspects. Inside, the
//   match yields `d: &Drink`; we `*d` to copy it (Drink is `Copy`).
//
//   Deriving `Copy` on both enums makes ownership-passing style cheap:
//   `self` moves are essentially free for these types. We could remove
//   `Copy` and the API would still be sound — just slightly less
//   ergonomic.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `&mut self` mutating the machine in place
//     - Works, but mutating an enum variant in place requires `*self =
//       VendingMachine::Selected(d)` style writes that read worse than
//       the by-value approach.
//
//   Separate types for each state (`struct Idle`, `struct Selected { d }`,
//   ...) plus methods that return the next state's TYPE
//     - The "typestate pattern" — enforces transitions at the TYPE level,
//       not the runtime level. Strictly stronger guarantees, but more
//       boilerplate. Worth it for safety-critical machines; an enum is
//       the right size for our needs here.
//
// SUBTLETY:
//   `Drink` derives `Copy`, so `Some(*d)` copies the inner value out of
//   the borrow. Without `Copy` you'd have to `.clone()` or rework the
//   API. For enums that are fundamentally cheap (no heap data), `Copy`
//   plus `Clone` is the right default.
