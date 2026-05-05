// =============================================================================
//  enum_quiz — capstone: a tiny vending machine state machine
// =============================================================================
//
// State machines are where Rust enums truly shine. The compiler can prove
// you've handled every (state, event) pair — invalid transitions become
// impossible to write, not just impossible to test.
//
// THE MACHINE
//
//     ┌──────┐  insert(d) ┌──────────────┐  dispense  ┌────────────┐
//     │ Idle │ ──────────▶│ Selected(d)  │ ──────────▶│ Dispensing │
//     └──────┘            └──────────────┘            └────────────┘
//        ▲                                                  │
//        └──────────────────── finish ─────────────────────-┘
//
// States:
//   - Idle             — waiting for a customer
//   - Selected(Drink)  — drink chosen, waiting to dispense
//   - Dispensing       — currently dispensing
//
// Events (= methods on the machine):
//   - insert(d)  : Idle              → Selected(d)        (else: stay)
//   - dispense() : Selected(d)       → Dispensing         (else: stay)
//   - finish()   : Dispensing        → Idle               (else: stay)
//
// Any event in a state that doesn't accept it MUST leave the machine
// unchanged. We model "stay put" by having each method take `self` and
// return `Self`, returning the same value when the transition isn't
// allowed.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Fill in:
//   - The `Drink` enum (data-less variants Cola, Water, Coffee).
//   - The `VendingMachine` enum (Idle, Selected(Drink), Dispensing).
//   - The three transition methods on `VendingMachine`.
//   - `current_drink(&self) -> Option<Drink>` returning the chosen drink
//     when in `Selected`, otherwise `None`.
//
// Each transition method takes `self` (consumes the old state) and returns
// the new `VendingMachine`. Use `match self { ... }` inside.
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

#[derive(Debug, Clone, Copy, PartialEq)]
enum Drink { Cola, Water, Coffee }

#[derive(Debug, Clone, Copy, PartialEq)]
enum VendingMachine {
    ???,
    ???(Drink),
    ???,
}

impl VendingMachine {
    fn insert(self, d: Drink) -> VendingMachine {
        match self {
            VendingMachine::Idle => VendingMachine::???(d),
            other => other,
        }
    }

    fn dispense(self) -> VendingMachine {
        match self {
            VendingMachine::Selected(_) => VendingMachine::???,
            other => other,
        }
    }

    fn finish(self) -> VendingMachine {
        match self {
            VendingMachine::Dispensing => ???,
            other => other,
        }
    }

    fn current_drink(&self) -> Option<Drink> {
        match self {
            VendingMachine::Selected(d) => ???,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn happy_path() {
        let m = VendingMachine::Idle;
        let m = m.insert(Drink::Coffee);
        assert_eq!(m, VendingMachine::Selected(Drink::Coffee));
        assert_eq!(m.current_drink(), Some(Drink::Coffee));

        let m = m.dispense();
        assert_eq!(m, VendingMachine::Dispensing);
        assert_eq!(m.current_drink(), None);

        let m = m.finish();
        assert_eq!(m, VendingMachine::Idle);
    }

    #[test] fn invalid_transitions_are_noops() {
        // dispense from Idle → still Idle
        let m = VendingMachine::Idle.dispense();
        assert_eq!(m, VendingMachine::Idle);

        // finish from Idle → still Idle
        let m = VendingMachine::Idle.finish();
        assert_eq!(m, VendingMachine::Idle);

        // insert while already Selected → unchanged
        let m = VendingMachine::Selected(Drink::Cola).insert(Drink::Water);
        assert_eq!(m, VendingMachine::Selected(Drink::Cola));

        // finish while Selected → unchanged
        let m = VendingMachine::Selected(Drink::Cola).finish();
        assert_eq!(m, VendingMachine::Selected(Drink::Cola));

        // insert while Dispensing → unchanged
        let m = VendingMachine::Dispensing.insert(Drink::Water);
        assert_eq!(m, VendingMachine::Dispensing);
    }

    #[test] fn drinks_round_trip() {
        for d in [Drink::Cola, Drink::Water, Drink::Coffee] {
            let m = VendingMachine::Idle.insert(d);
            assert_eq!(m.current_drink(), Some(d));
        }
    }
}

fn main() {}
