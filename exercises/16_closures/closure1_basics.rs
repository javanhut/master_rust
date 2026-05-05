// =============================================================================
//  closure1 — closure basics: syntax, inference, capture, `move`
// =============================================================================
//
// A closure is an anonymous function that can CAPTURE variables from the
// surrounding scope. The simplest form looks almost exactly like a function
// but with vertical bars instead of `fn name(...)`:
//
//     let add_one = |x| x + 1;
//     let y = add_one(4);          // 5
//
// SYNTAX SHAPES
//
//     |x|          x + 1            // one parameter, one-expression body
//     |x, y|       x + y            // multiple parameters
//     |x: i32| -> i32 { x + 1 }     // explicit types + block body
//     ||           42               // zero parameters
//
// TYPE INFERENCE
//
// Closure parameter and return types are usually INFERRED from the first
// call site. That means the very first call locks the type in:
//
//     let id = |x| x;
//     let _ = id(3i32);            // id is now |i32| -> i32
//     // let _ = id("hi");         // ❌ — already inferred as i32
//
// Functions, by contrast, must always have full type signatures. Closures
// are a tighter, more local tool.
//
// CAPTURING THE ENVIRONMENT
//
// A closure can use names from the enclosing scope. The compiler picks the
// LEAST INVASIVE capture mode it can get away with:
//
//     let s = String::from("hi");
//     let say = || println!("{s}");      // captures &s — borrows
//     say(); say();                      // fine, s is still usable
//
//     let mut n = 0;
//     let mut inc = || { n += 1; };      // captures &mut n — mutable borrow
//     inc(); inc();
//     // println!("{n}");                // ❌ while `inc` is alive
//
//     let v = vec![1, 2, 3];
//     let take = || { let _consumed = v; };  // captures by VALUE — `v` moved
//     take();
//     // println!("{v:?}");              // ❌ — moved
//
// THE `move` KEYWORD
//
// Sometimes you want to FORCE capture by value — typically when handing
// the closure to another thread, returning it from a function, or detaching
// it from a short-lived scope. Prefix the closure with `move`:
//
//     let s = String::from("hi");
//     let owned = move || println!("{s}");  // s is now owned by `owned`
//     owned();
//     // println!("{s}");                   // ❌ — moved into the closure
//
// `move` doesn't change WHAT the closure does — it only changes HOW the
// environment is captured (always by value).
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//   - `make_adder(n)` returns a closure that adds `n` to its argument.
//   - `times_called` runs a closure that captures and mutates a counter,
//     returning the final count after calling it `k` times.
//   - `greet_owned(name)` builds a `move` closure that owns `name` and
//     returns the greeting "Hello, <name>!".

// I AM NOT DONE

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // Return a closure of one i32 argument that adds `n` to it.
    ???
}

fn times_called(k: u32) -> u32 {
    let mut count = 0u32;
    // Build a closure that increments `count` by 1 each call.
    let mut bump = ???;
    for _ in 0..k {
        bump();
    }
    count
}

fn greet_owned(name: String) -> String {
    // Build a `move` closure that takes no args and returns
    // format!("Hello, {name}!"), then call it and return its output.
    let greeter = ???;
    greeter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn adder_basic() {
        let add5 = make_adder(5);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-3), 2);
    }

    #[test] fn adder_independent() {
        let add2 = make_adder(2);
        let add100 = make_adder(100);
        assert_eq!(add2(1), 3);
        assert_eq!(add100(1), 101);
    }

    #[test] fn counter_runs() {
        assert_eq!(times_called(0), 0);
        assert_eq!(times_called(1), 1);
        assert_eq!(times_called(7), 7);
    }

    #[test] fn greet_moves_string() {
        assert_eq!(greet_owned(String::from("Ada")), "Hello, Ada!");
        assert_eq!(greet_owned(String::from("Rust")), "Hello, Rust!");
    }
}

fn main() {}
