// =============================================================================
//  macro5 — a taste of procedural macros (`#[derive(Debug)]` and friends)
// =============================================================================
//
// `macro_rules!` covers a huge range of "code that writes code," but it
// has limits — pattern-matching on token streams only gets you so far.
// PROCEDURAL macros are the next step up. They are Rust functions that
// take a `TokenStream` in and return a `TokenStream` out; the compiler
// runs them at compile time.
//
// THREE FLAVOURS
//
//   1. DERIVE macros          — `#[derive(Debug)]`, `#[derive(Serialize)]`.
//                                Generate trait `impl`s for the annotated
//                                struct/enum.
//   2. ATTRIBUTE macros       — `#[tokio::main]`, `#[wasm_bindgen]`.
//                                Replace or wrap an item.
//   3. FUNCTION-LIKE macros   — `sqlx::query!(...)`, `html!(...)`.
//                                Look like `name!(...)` calls but are
//                                implemented as procedural macros, not
//                                `macro_rules!`.
//
// All three are implemented inside a SEPARATE crate with
// `proc-macro = true` in its `Cargo.toml`. We can't show you a real one
// in this single-file sandbox, so this exercise is a guided walk-through.
//
// =============================================================================
//  THE PIPELINE — WHAT HAPPENS WHEN THE COMPILER SEES `#[derive(Debug)]`
// =============================================================================
//
//          source code with `#[derive(Debug)]`
//                          │
//                          ▼
//      compiler tokenises the annotated item into a `TokenStream`
//                          │
//                          ▼
//        proc-macro fn(input: TokenStream) -> TokenStream
//                          │
//                          ▼
//     parse `TokenStream` → `proc_macro2::TokenStream` (a friendlier API)
//                          │
//                          ▼
//      `syn::parse2::<DeriveInput>(ts)`  →  AST struct describing the type
//                          │
//                          ▼
//          inspect fields/variants, decide what to generate
//                          │
//                          ▼
//      `quote! { impl Debug for ... { ... } }`  →  TokenStream
//                          │
//                          ▼
//                  return TokenStream
//                          │
//                          ▼
//   compiler splices the result into the program AS IF the user typed it
//
// THE THREE LIBRARIES YOU WILL ALWAYS REACH FOR:
//
//   - `proc_macro` (built-in)  — the `TokenStream` type that the
//                                compiler hands you. Minimal API.
//   - `proc_macro2`            — a parallel `TokenStream` you can
//                                construct, manipulate, and unit-test
//                                outside the compiler. Convert with
//                                `.into()`.
//   - `syn`                    — full Rust parser. Turns a token stream
//                                into a typed AST: `DeriveInput`,
//                                `ItemFn`, `Expr`, `Type`, ...
//   - `quote`                  — the `quote!` macro. Lets you write the
//                                code you want to GENERATE in nearly
//                                normal Rust syntax, with `#variable`
//                                interpolation for inserted pieces.
//
// =============================================================================
//  WHAT `#[derive(Debug)]` ACTUALLY GENERATES
// =============================================================================
//
// Imagine the user writes:
//
//     #[derive(Debug)]
//     struct Point {
//         x: i32,
//         y: i32,
//     }
//
// The compiler runs the built-in `Debug` derive (which behaves like a
// proc macro). The generated code is roughly:
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
// For an enum:
//
//     #[derive(Debug)]
//     enum Shape {
//         Circle(f64),
//         Rect { w: f64, h: f64 },
//     }
//
// becomes approximately:
//
//     impl ::core::fmt::Debug for Shape {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
//             match self {
//                 Shape::Circle(__0) => f.debug_tuple("Circle").field(__0).finish(),
//                 Shape::Rect { w, h } => f
//                     .debug_struct("Rect")
//                     .field("w", w)
//                     .field("h", h)
//                     .finish(),
//             }
//         }
//     }
//
// You can see actual expansions on a real Cargo project with the
// `cargo expand` subcommand:
//
//     cargo install cargo-expand
//     cargo expand            # prints the whole crate after macro expansion
//
// =============================================================================
//  HOW WOULD YOU IMPLEMENT `Debug` AS A PROC MACRO?
// =============================================================================
//
// Sketch — DO NOT try to compile this; it's pseudocode for a function
// that would live in a `proc-macro = true` crate:
//
//     use proc_macro::TokenStream;
//     use quote::quote;
//     use syn::{parse_macro_input, DeriveInput, Data, Fields};
//
//     #[proc_macro_derive(MyDebug)]
//     pub fn my_debug_derive(input: TokenStream) -> TokenStream {
//         let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
//         let name = &ast.ident;
//
//         // For brevity, only handle named-field structs.
//         let body = match &ast.data {
//             Data::Struct(s) => match &s.fields {
//                 Fields::Named(named) => {
//                     let field_names: Vec<_> = named
//                         .named
//                         .iter()
//                         .map(|f| f.ident.as_ref().unwrap())
//                         .collect();
//                     let field_strs: Vec<String> =
//                         field_names.iter().map(|i| i.to_string()).collect();
//
//                     quote! {
//                         f.debug_struct(stringify!(#name))
//                             #( .field(#field_strs, &self.#field_names) )*
//                             .finish()
//                     }
//                 }
//                 _ => unimplemented!("only named-field structs in this demo"),
//             },
//             _ => unimplemented!("only structs in this demo"),
//         };
//
//         let expanded = quote! {
//             impl ::core::fmt::Debug for #name {
//                 fn fmt(
//                     &self,
//                     f: &mut ::core::fmt::Formatter<'_>,
//                 ) -> ::core::fmt::Result {
//                     #body
//                 }
//             }
//         };
//
//         expanded.into()
//     }
//
// Read the structure even if you don't memorise the names:
//
//   1. Parse input tokens into a `DeriveInput` AST.
//   2. Pull out the bits you care about (here: the type name and the
//      list of named fields).
//   3. Use `quote! { ... }` to write the code you want to emit, with
//      `#name` / `#field_names` interpolation. The `#( ... )*` syntax
//      inside `quote!` is a repetition — like `macro_rules!` repeaters
//      but on `quote`'s side.
//   4. Return the resulting `TokenStream`.
//
// =============================================================================
//  WHEN TO REACH FOR A PROC MACRO
// =============================================================================
//
// Use `macro_rules!` for almost everything. Reach for a procedural macro
// when:
//
//   - You need to INSPECT the structure of a type — fields of a struct,
//     variants of an enum, attributes on items. That requires parsing,
//     which `macro_rules!` cannot do.
//   - You're shipping a library and want users to write
//     `#[derive(MyTrait)]` instead of an `impl` block by hand.
//   - You need to validate input at compile time against an external
//     resource (e.g. `sqlx::query!` checks SQL against a real database
//     during compilation).
//
// Procedural macros cost more to write (separate crate, dependency on
// `syn`/`quote`, slower compile times) — only pay that cost when the
// power of an actual AST is genuinely needed.
//
// =============================================================================
//  YOUR (TINY) TASK
// =============================================================================
// Reading exercise. The Rust file below uses `#[derive(Debug)]` — the
// most common procedural macro you'll ever invoke. Once you've digested
// the comments above, delete the `// I AM NOT DONE` line and submit.

// I AM NOT DONE

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

    // The strings printed below are produced by code GENERATED by the
    // `Debug` derive — code we never wrote ourselves.
    println!("{:?}", p);
    println!("{:?}", s);
    println!("{:?}", c);
}
