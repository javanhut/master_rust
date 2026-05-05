// SOLUTION — macro5_proc_taste

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

fn main() {
    let p = Point { x: 3, y: 4 };
    let s = Shape::Rect { w: 2.0, h: 5.0 };
    let c = Shape::Circle(1.5);

    println!("{:?}", p);
    println!("{:?}", s);
    println!("{:?}", c);
}

// WHY THIS IS OPTIMAL:
//
//   A reading exercise. The point was to understand what `#[derive(...)]`
//   actually IS — a procedural macro that reads a struct/enum, builds an
//   AST, generates a trait impl, and splices that impl back into your
//   program. The final code is exactly what you would have written by
//   hand if you had implemented `Debug::fmt` for `Point` and `Shape`
//   yourself.
//
// QUICK REFERENCE — THE PROC-MACRO PIPELINE:
//
//   1. Compiler hands the macro a `proc_macro::TokenStream`.
//   2. Convert with `.into()` to a `proc_macro2::TokenStream` (richer API,
//      testable outside the compiler).
//   3. Parse with `syn` into a typed AST (`DeriveInput`, `ItemFn`, etc).
//   4. Walk the AST, decide what code to emit.
//   5. Build the new code with `quote! { ... }`, using `#name` to splice
//      in identifiers/expressions and `#( ... )*` for repetitions.
//   6. Return the resulting `TokenStream`. The compiler splices it in.
//
// GENERATED CODE FOR `#[derive(Debug)] struct Point { x, y }`:
//
//     impl ::core::fmt::Debug for Point {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
//             f.debug_struct("Point")
//                 .field("x", &self.x)
//                 .field("y", &self.y)
//                 .finish()
//         }
//     }
//
//   You can see this on any real Cargo project with `cargo expand` (a
//   third-party subcommand, install with `cargo install cargo-expand`).
//
// WHEN TO USE WHICH MACRO KIND:
//
//   - Token-shape rewrite, no need to inspect type structure?
//     → `macro_rules!`. Cheap, hygienic, no extra crate.
//   - Need to walk the fields of the user's struct or vary output by
//     type structure?
//     → procedural macro (`#[proc_macro_derive]`,
//       `#[proc_macro_attribute]`, or `#[proc_macro]` for
//       function-like).
//   - Need to validate against an external resource at compile time
//     (database schemas, GraphQL schemas, regex syntax)?
//     → procedural macro, often function-like.
//
// COMMON MISTAKES:
//
//   - Reaching for proc macros too early. `macro_rules!` solves most
//     problems and costs almost nothing.
//   - Forgetting that proc macros must live in their own crate with
//     `proc-macro = true` in `Cargo.toml`. You can't define one inline
//     in a binary crate.
//   - Hand-rolling a `Debug` or `PartialEq` impl when `#[derive(...)]`
//     would have produced an equivalent one. Save your code review
//     budget for impls that genuinely differ from the mechanical version.
