// SOLUTION — trait5_object_safety

trait Logger {
    fn log(&self, msg: &str);

    fn make_copy(&self) -> Self
    where
        Self: Sized;
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, msg: &str) {
        println!("{msg}");
    }

    fn make_copy(&self) -> Self {
        StdoutLogger
    }
}

fn main() {
    let _boxed: Box<dyn Logger> = Box::new(StdoutLogger);
    _boxed.log("ready");
}

// WHY THIS IS OPTIMAL:
//
//   We had two object-safety violations to deal with, and they're treated
//   differently because the underlying problems are different:
//
//   1. `make_copy(&self) -> Self`
//
//      A vtable slot needs ONE function pointer with ONE concrete return
//      type, but `Self` resolves to a different concrete type for every
//      implementor. Adding `where Self: Sized` tells the compiler "this
//      method is only callable when Self is a concrete sized type" —
//      i.e. through static dispatch, never through `dyn Logger`. That
//      excludes it from the vtable and lets the trait become object-safe
//      again. You can still call `concrete.make_copy()` on a concrete
//      `StdoutLogger`, you just can't call it through `dyn Logger`.
//
//   2. `log_any<T: Display>(&self, x: T)`
//
//      Generic methods on the trait itself can't be put in a vtable —
//      one method declaration would need infinitely many vtable slots
//      (one per concrete T the method is ever instantiated with). There
//      is no "where clause" rescue for generics; you just can't have
//      them in an object-safe trait. We delete the method.
//
//      If we genuinely needed that capability, we'd move it OUTSIDE the
//      trait:
//
//          fn log_any<T: Display>(logger: &dyn Logger, x: T) {
//              logger.log(&format!("{x}"));
//          }
//
//      Same surface for the caller, but the generic parameter belongs to
//      a free function, not to the vtable.
//
// THE OBJECT-SAFETY CHECKLIST (memorise it):
//
//   For a trait `T` to be `dyn`-able, every method that the vtable
//   contains must satisfy:
//     - the receiver is one of: &self, &mut self, self: Box<Self>,
//       self: Pin<&mut Self>, self: Rc<Self>, self: Arc<Self>;
//     - `Self` does NOT appear elsewhere in the signature
//       (no `-> Self`, no `Self` argument);
//     - the method has NO generic type parameters of its own.
//
//   Methods that violate any of these can survive in the trait IF you
//   gate them with `where Self: Sized` — they vanish from the vtable but
//   remain callable on concrete types.
//
// COMPILER ERROR YOU SAW:
//
//     error[E0038]: the trait `Logger` cannot be made into an object
//        |
//        |   fn make_copy(&self) -> Self;
//        |                          ^^^^ ...because method `make_copy`
//        |                               references the `Self` type
//        |                               in its return type
//
//   The error message even suggests `where Self: Sized`. Read those E0038
//   diagnostics carefully — they spell out the violation by name.
