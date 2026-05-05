// =============================================================================
//  closure3 — returning closures from functions
// =============================================================================
//
// Every closure has its own anonymous, compiler-generated TYPE. You can't
// write that type in source code. So how do you return one from a function?
//
// TWO TOOLS, TWO TRADE-OFFS:
//
// 1. STATIC DISPATCH: `-> impl Fn(i32) -> i32`
//
//        fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
//            move |x| x + n
//        }
//
//    `impl Trait` says "I'm returning SOME concrete type that implements
//    `Fn(i32) -> i32`, and the compiler will figure out which." The actual
//    closure type is inlined at the call site — zero overhead, no heap.
//
//    Limitation: a function can only return ONE concrete type. You can't
//    return a different closure from each branch:
//
//        fn pick(b: bool) -> impl Fn(i32) -> i32 {
//            if b { |x| x + 1 } else { |x| x - 1 }   // ❌ different types
//        }
//
// 2. DYNAMIC DISPATCH: `-> Box<dyn Fn(i32) -> i32>`
//
//        fn pick(b: bool) -> Box<dyn Fn(i32) -> i32> {
//            if b { Box::new(|x| x + 1) }
//            else { Box::new(|x| x - 1) }            // ✅ both boxed dyn
//        }
//
//    `Box<dyn Trait>` is a heap-allocated trait object. The exact type is
//    erased — calls go through a vtable. Slower (an extra indirection),
//    but flexible: you can store closures of different shapes in one Vec,
//    return them from different match arms, etc.
//
// WHEN TO USE WHICH:
//
//     impl Fn(...)        — default. Hot paths, simple "factory" returns.
//     Box<dyn Fn(...)>    — when you need heterogeneous closures, store
//                           them in a collection, or hide the concrete
//                           type in a struct field.
//
// FnMut and FnOnce work the same way — substitute the trait you need.
//
// =============================================================================
//  YOUR TASK
// =============================================================================
//
//   - `multiplier(factor)` returns `impl Fn(i32) -> i32` that multiplies
//     its argument by `factor`. Use static dispatch.
//
//   - `adjuster(mode)` returns a closure based on a runtime flag:
//        mode == "double" → multiplies by 2
//        mode == "negate" → negates
//        anything else    → identity (returns its input unchanged)
//     The branches return different closure types, so use Box<dyn Fn(...)>.

// I AM NOT DONE

fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    ???
}

fn adjuster(mode: &str) -> Box<dyn Fn(i32) -> i32> {
    match mode {
        "double" => ???,
        "negate" => ???,
        _        => ???,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn mult_by_three() {
        let triple = multiplier(3);
        assert_eq!(triple(0), 0);
        assert_eq!(triple(4), 12);
        assert_eq!(triple(-5), -15);
    }

    #[test] fn mult_independent() {
        let by2 = multiplier(2);
        let by10 = multiplier(10);
        assert_eq!(by2(7), 14);
        assert_eq!(by10(7), 70);
    }

    #[test] fn adjuster_double() {
        assert_eq!(adjuster("double")(5), 10);
    }

    #[test] fn adjuster_negate() {
        assert_eq!(adjuster("negate")(5), -5);
    }

    #[test] fn adjuster_identity() {
        assert_eq!(adjuster("nope")(5), 5);
        assert_eq!(adjuster("")(42), 42);
    }
}

fn main() {}
