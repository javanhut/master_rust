// =============================================================================
//  closure2 — the three closure traits: Fn, FnMut, FnOnce
// =============================================================================
//
// Every closure automatically implements one or more of three traits, based
// on HOW it uses its captured environment:
//
//     FnOnce   — can be called AT LEAST ONCE. May consume (move out of) the
//                captured environment, so calling it a second time may be
//                impossible. Every closure is at least FnOnce.
//
//     FnMut    — can be called MULTIPLE TIMES, and may MUTATE the captured
//                environment. Requires `&mut` access to the closure.
//                Every FnMut is also FnOnce.
//
//     Fn       — can be called multiple times and only READS the captured
//                environment (or doesn't capture anything). Requires only
//                `&` access to the closure — safe to share.
//                Every Fn is also FnMut, and therefore FnOnce.
//
// Hierarchy:    Fn  ⊂  FnMut  ⊂  FnOnce        (smaller = more permissive)
//
// HOW THE COMPILER PICKS
//
// The compiler looks at the closure body and chooses the most permissive
// trait it can:
//
//     |x| x + 1                         — Fn  (no capture)
//     |x| x + n                         — Fn  (reads `n`)
//     |x| { *counter += x; *counter }   — FnMut (mutates `counter`)
//     || drop(s)                        — FnOnce (moves `s` out — once)
//
// WHY YOU CARE
//
// When you write a function that ACCEPTS a closure, the trait you require
// is a contract:
//
//     fn run_once<F: FnOnce()>(f: F)        // accepts ANY callable
//     fn run_many<F: FnMut()>(mut f: F)     // accepts Fn or FnMut
//     fn run_shared<F: Fn()>(f: F)          // accepts ONLY Fn — safe to share
//
// Pick the WEAKEST bound that fits your needs. Asking for less means the
// caller can give you more.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
// Three small APIs. Each takes a closure with a specific trait bound; pick
// the right one for the job.
//
//   1. `call_twice(f)` — runs `f` exactly two times, and `f` does NOT mutate
//      anything. Must accept a plain `Fn() -> i32` and return the SUM of
//      the two return values.
//
//   2. `accumulate(n, f)` — runs `f()` `n` times. Each call may MUTATE its
//      captured state. Returns the LAST value produced.
//      (Test passes a counter-style closure.)
//
//   3. `consume(f)` — runs `f` exactly ONCE. The closure is allowed to move
//      values out of its environment. Returns whatever `f` returns.

// I AM NOT DONE

fn call_twice<F: ???>(f: F) -> i32 {
    f() + f()
}

fn accumulate<F: ???>(n: u32, mut f: F) -> i32 {
    let mut last = 0;
    for _ in 0..n {
        last = f();
    }
    last
}

fn consume<F: ???>(f: F) -> String {
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn fn_no_capture() {
        // pure closure, captures nothing — implements Fn
        assert_eq!(call_twice(|| 21), 42);
    }

    #[test] fn fn_reads_capture() {
        let n = 7;
        // closure reads `n` — still Fn
        assert_eq!(call_twice(|| n * 3), 42);
    }

    #[test] fn fnmut_counter() {
        let mut count = 0;
        let bump = || { count += 1; count };
        assert_eq!(accumulate(5, bump), 5);
    }

    #[test] fn fnmut_independent_runs() {
        let mut total = 0;
        assert_eq!(accumulate(3, || { total += 10; total }), 30);
    }

    #[test] fn fnonce_moves_string() {
        let s = String::from("hello");
        // `move` + the body consumes `s` — FnOnce only
        let f = move || s;
        assert_eq!(consume(f), "hello");
    }
}

fn main() {}
