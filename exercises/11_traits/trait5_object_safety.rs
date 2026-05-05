// =============================================================================
//  trait5 — object safety: when can a trait become `dyn Trait`?
// =============================================================================
//
// Not every trait can be turned into a trait object. The compiler enforces
// a property called OBJECT SAFETY. If a trait isn't object-safe, you can
// `impl` it for concrete types just fine, but you cannot say `dyn Trait`,
// `&dyn Trait`, or `Box<dyn Trait>` anywhere — the compiler refuses.
//
// THE INTUITION
//
// A trait object stores its data behind a pointer plus a vtable. The
// vtable holds one slot per method. For that to make sense, every method
// has to:
//
//   1. Have a SELF TYPE that's compatible with a pointer
//      (`&self`, `&mut self`, `self: Box<Self>`, `self: Pin<&mut Self>` …).
//   2. NOT mention `Self` in its signature outside of the receiver
//      (no `-> Self`, no `(&self, other: Self)`).
//   3. NOT have GENERIC type parameters of its own
//      (`fn foo<T>(&self, x: T)` is forbidden — the vtable would need
//      infinitely many entries, one per T).
//
// EXAMPLES OF UN-SAFE METHODS
//
//     trait Bad {
//         fn copy(&self) -> Self;          // returns Self — no good
//         fn store<T>(&self, x: T);        // generic method — no good
//     }
//
// `Bad::copy`: every implementor returns its OWN concrete type. There's
// no single function pointer that fits in a vtable slot.
//
// `Bad::store`: a vtable would need a slot per concrete T the method is
// ever called with — that's not a finite table.
//
// HOW TO FIX A TRAIT FOR OBJECT SAFETY
//
//   - Replace `-> Self` with a more specific return (e.g. `Box<dyn Bad>`
//     or `String` or any other concrete type).
//   - Replace generic methods with non-generic ones. If the method really
//     needs to be generic, MOVE that method out of the trait into a free
//     function that takes `&dyn Trait` plus the generic.
//   - You can also keep an offending method but mark it `where Self:
//     Sized` — that excludes it from the vtable; `dyn Trait` becomes
//     legal but the method can only be called via static dispatch.
//
// DOWNGRADE-TO-OBJECT-SAFE WITH `where Self: Sized`
//
//     trait Logger {
//         fn log(&self, msg: &str);
//
//         fn boxed(self) -> Box<dyn Logger>
//         where Self: Sized,
//         {
//             Box::new(self)
//         }
//     }
//
// Here `boxed` returns `Self`, but the `where Self: Sized` clause excludes
// it from the vtable, so `dyn Logger` is still allowed. You can still
// call `.boxed()` on a concrete logger (statically), just not through
// a `&dyn Logger`.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Below is `Logger`, a trait that the compiler currently REJECTS as a
// trait object because of TWO problems:
//
//   (A) `make_copy` returns `Self`.
//   (B) `log_any` is a generic method (`<T>`).
//
// Your fixes:
//
//   1. Add `where Self: Sized` to `make_copy` so it stays in the trait
//      but is excluded from the vtable.
//
//   2. DELETE `log_any` from the trait entirely. (The exercise file already
//      shows where to delete it — replace the whole method, including its
//      header line, with a blank line or just remove it.)
//
// `log` itself (the simple `&self`, no Self, no generics) is fine and
// stays exactly as it is.
//
// This is a "compile mode" exercise: PASS just means the file compiles
// (and the `Box::new(...)` line at the bottom of the file works because
// `Logger` is now object-safe). There are no `#[cfg(test)]` blocks.

// I AM NOT DONE

trait Logger {
    // Object-safe: takes `&self`, no Self in the signature, no generics.
    fn log(&self, msg: &str);

    // CURRENTLY UN-SAFE because it returns `Self`.
    // Add a `where Self: Sized` clause so this method is excluded from
    // the vtable and `dyn Logger` becomes legal again.
    fn make_copy(&self) -> Self ???;

    // CURRENTLY UN-SAFE because it has a generic type parameter `<T>`.
    // DELETE this whole method. (The fix is removing it, not adding a
    // bound — generics can't be turned into vtable slots, period.)
    fn log_any<T: std::fmt::Display>(&self, x: T);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, msg: &str) {
        println!("{msg}");
    }

    fn make_copy(&self) -> Self {
        StdoutLogger
    }
    // After you delete `log_any` from the trait, also delete this impl.
    fn log_any<T: std::fmt::Display>(&self, x: T) {
        println!("{x}");
    }
}

fn main() {
    // This line proves `Logger` is now object-safe.
    let _boxed: Box<dyn Logger> = Box::new(StdoutLogger);
    _boxed.log("ready");
}
