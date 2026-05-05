# master_rust — build plan

A living checklist of every exercise across the 20-chapter curriculum. Each box gets ticked when the exercise file, its annotated solution, and its `info.toml` entry are all written and verified end-to-end.

**Legend** — `[x]` shipped · `[ ]` to do · `(mode)` is the runner mode the exercise uses.

---

## ✅ Chapter 1 — Intro & Variables  (8 / 8)

- [x] **intro1** — `fn main`, `println!`, the `;`, what a macro is. _(run)_
- [x] **intro2_print** — `{}`, `{:?}`, `{:#?}`, named/positional args, precision specifiers. _(run)_
- [x] **intro3_variables** — `let`, immutability, type inference. _(test)_
- [x] **intro4_mutability** — `let mut`, accumulator pattern, when not to use mut. _(test)_
- [x] **intro5_shadowing** — re-bind with new type, shadowing vs `mut`. _(test)_
- [x] **intro6_constants** — `const`, naming, required type, vs `static`. _(test)_
- [x] **intro7_types** — annotations vs turbofish, when each is required. _(test)_
- [x] **intro_quiz** — capstone combining `let`/`mut`/shadow/`const`. _(test)_

## ✅ Chapter 2 — Primitive types  (6 / 6)

- [x] **types1_integers** — i/u widths, overflow, wrapping/saturating, `as` cast. _(test)_
- [x] **types2_floats** — f32/f64, mixing with ints, `powi`/`powf`, NaN. _(test)_
- [x] **types3_bool_char** — short-circuit ops, char vs str, `to_digit`, `matches!`. _(test)_
- [x] **types4_tuples** — destructuring, field access, returning multiple values. _(test)_
- [x] **types5_arrays** — `[T; N]`, slices `&[T]`, `[v; n]` shorthand, indexing. _(test)_
- [x] **types_quiz** — capstone: implement `stats(&[i32]) -> (min, max, mean)`. _(test)_

## ✅ Chapter 3 — Functions & control flow  (6 / 6)

- [x] **fn1_basic** — signatures, params, return types, trailing-expression rule. _(test)_
- [x] **fn2_expressions** — `if/else` as expression, blocks as values. _(test)_
- [x] **fn3_if** — `if` requires `bool`, type-equal arms, `else if` chains. _(test)_
- [x] **fn4_loops** — `loop`/`while`/`for`, ranges, `break value`, labels. _(test)_
- [x] **fn5_early_return** — `return` for guards, idiomatic trailing expr. _(test)_
- [x] **flow_quiz** — FizzBuzz returning `Vec<String>`. _(test)_

---

## ✅ Chapter 4 — Ownership, borrowing, references  (8 / 8)

- [x] **own1_move** — what "move" means, why `let s2 = s1` invalidates `s1`. _(test)_
- [x] **own2_clone** — `Clone` vs `Copy`, when each applies, the perf cost. _(test)_
- [x] **own3_functions** — passing ownership into fns, returning ownership back. _(test)_
- [x] **own4_borrow** — `&T` shared references, multiple readers. _(test)_
- [x] **own5_mut_borrow** — `&mut T`, the one-writer rule, lexical scope. _(test)_
- [x] **own6_slice** — `&str` and `&[T]` as borrowed views into owned data. _(test)_
- [x] **own7_dangling** — why the borrow checker rejects returning `&local`. _(compile)_
- [x] **own_quiz** — capstone: implement a small "in-place" transform without cloning. _(test)_

## ✅ Chapter 5 — Strings  (7 / 7)

- [x] **str1_owned_vs_borrowed** — `String` vs `&str`, when to choose which. _(test)_
- [x] **str2_create** — `"lit"`, `String::from`, `to_string()`, `to_owned()`, the differences. _(test)_
- [x] **str3_concat** — `+`, `format!`, `push_str`, when each is right. _(test)_
- [x] **str4_iterate** — `chars()`, `bytes()`, why indexing strings is forbidden. _(test)_
- [x] **str5_split_join** — `split`, `splitn`, `lines`, `trim`, `join`. _(test)_
- [x] **str6_parse** — `&str -> i32`/`f64`, `Result`, the role of `unwrap`. _(test)_
- [x] **str_quiz** — capstone: word counter / line reverser. _(test)_

## ✅ Chapter 6 — Collections: Vec & HashMap  (8 / 8)

- [x] **vec1_create** — `Vec::new`, `vec![]`, `with_capacity`, growing semantics. _(test)_
- [x] **vec2_index_get** — `vec[i]` panics, `vec.get(i)` returns Option, `len`/`is_empty`. _(test)_
- [x] **vec3_mutate** — `push`, `pop`, `insert`, `remove`, `swap_remove`, `retain`. _(test)_
- [x] **vec4_iterate** — `iter`, `iter_mut`, `into_iter`, `for &x in`, `for x in &mut`. _(test)_
- [x] **vec5_slicing** — `&v[1..3]`, slice as a borrowed view, slice methods. _(test)_
- [x] **map1_create** — `HashMap::new`, `insert`, `get`, `contains_key`. _(test)_
- [x] **map2_entry** — the `entry().or_insert()` pattern (counter, group-by). _(test)_
- [x] **collections_quiz** — capstone: word-frequency counter. _(test)_

## ✅ Chapter 7 — Iterators  (9 / 9)

- [x] **iter1_basics** — what `Iterator` is, `next()`, `for` desugaring. _(test)_
- [x] **iter2_map_filter** — `map`, `filter`, lazy evaluation. _(test)_
- [x] **iter3_fold_sum** — `sum`, `product`, `count`, `fold` and how they relate. _(test)_
- [x] **iter4_collect** — `collect()`, turbofish on collect, into Vec/HashMap/String. _(test)_
- [x] **iter5_zip_enumerate** — `zip`, `enumerate`, `chain`, `take`, `skip`, `step_by`. _(test)_
- [x] **iter6_find_any_all** — short-circuit terminators, `position`, `min`/`max_by_key`. _(test)_
- [x] **iter7_flat_map** — `flat_map`, `flatten`, building iterators from iterators. _(test)_
- [x] **iter8_custom** — implement `Iterator` for your own struct. _(test)_
- [x] **iter_quiz** — capstone: rewrite the chapter-3 quizzes in iterator style. _(test)_

## ✅ Chapter 8 — Option, Result, and `?`  (8 / 8)

- [x] **opt1_basics** — `Some`/`None`, `is_some`, `is_none`, `unwrap`. _(test)_
- [x] **opt2_match** — pattern matching on `Option<T>`. _(test)_
- [x] **opt3_combinators** — `map`, `and_then`, `or`, `unwrap_or`, `unwrap_or_else`. _(test)_
- [x] **res1_basics** — `Result<T, E>`, recoverable errors, when to panic. _(test)_
- [x] **res2_question_mark** — the `?` operator and how it desugars. _(test)_
- [x] **res3_combinators** — `map`, `map_err`, `and_then`, `ok()`/`err()`. _(test)_
- [x] **res4_from** — `From`/`Into` for error conversion, why `?` "just works". _(test)_
- [x] **errors_quiz** — capstone: parse a config file, propagate errors with `?`. _(test)_

## ✅ Chapter 9 — Structs & methods  (7 / 7)

- [x] **struct1_define** — named-field structs, instantiation, field access. _(test)_
- [x] **struct2_tuple** — tuple structs, newtype pattern. _(test)_
- [x] **struct3_unit** — unit structs, when they're useful (markers). _(test)_
- [x] **struct4_impl** — `impl` blocks, methods, `&self`/`&mut self`/`self`. _(test)_
- [x] **struct5_associated** — `Self::new`, associated functions vs methods. _(test)_
- [x] **struct6_update** — struct-update syntax `..other`, field shorthand. _(test)_
- [x] **struct_quiz** — capstone: build a `Rectangle` with area, perimeter, contains. _(test)_

## ✅ Chapter 10 — Enums & pattern matching  (8 / 8)

- [x] **enum1_define** — variants with no data, with tuple data, with named fields. _(test)_
- [x] **enum2_match** — `match` on enums, exhaustiveness, `_` wildcard. _(test)_
- [x] **enum3_methods** — `impl` on enums, `&self` matching. _(test)_
- [x] **match1_patterns** — `|` alternatives, ranges, bindings (`x @ pat`). _(test)_
- [x] **match2_destructure** — destructure structs, tuples, nested. _(test)_
- [x] **match3_guards** — `if` guards in match arms. _(test)_
- [x] **match4_iflet_whilelet** — `if let`, `let else`, `while let`. _(test)_
- [x] **enum_quiz** — capstone: state machine for a vending machine. _(test)_

## ✅ Chapter 11 — Traits & generics  (9 / 9)

- [x] **trait1_define** — `trait` keyword, required methods, default methods. _(test)_
- [x] **trait2_impl** — implementing a trait for a type. _(test)_
- [x] **trait3_derive** — `#[derive(Debug, Clone, PartialEq, Eq, Hash)]`. _(test)_
- [x] **gen1_functions** — generic functions, `fn foo<T>(x: T)`. _(test)_
- [x] **gen2_bounds** — trait bounds (`T: Display`), multiple bounds, `where`. _(test)_
- [x] **gen3_structs** — generic structs, generic `impl` blocks. _(test)_
- [x] **trait4_objects** — `dyn Trait`, `Box<dyn Trait>`, dynamic dispatch. _(test)_
- [x] **trait5_object_safety** — what makes a trait object-safe (and what doesn't). _(compile)_
- [x] **traits_quiz** — capstone: shape hierarchy with `Area` trait, both static and dynamic dispatch. _(test)_

## ✅ Chapter 12 — Modules, crates, Cargo  (6 / 6)

- [x] **mod1_basics** — `mod`, `pub`, file-vs-inline modules. _(test)_
- [x] **mod2_paths** — `crate::`, `super::`, `self::`, `use` statements. _(test)_
- [x] **mod3_visibility** — `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`. _(test)_
- [x] **mod4_re_export** — `pub use`, the facade pattern. _(test)_
- [x] **cargo1_dependencies** — adding a dep, semver, features (read-only walkthrough). _(compile)_
- [x] **modules_quiz** — capstone: organise a multi-file utility crate. _(test)_

## ✅ Chapter 13 — Smart pointers  (7 / 7)

- [x] **box1_basics** — `Box<T>` heap allocation, recursive types. _(test)_
- [x] **box2_dyn** — `Box<dyn Trait>` revisited from chapter 11. _(test)_
- [x] **rc1_basics** — `Rc<T>` shared ownership, `Rc::clone`, reference counting. _(test)_
- [x] **refcell1_basics** — `RefCell<T>`, interior mutability, `borrow`/`borrow_mut` panics. _(test)_
- [x] **rc_refcell** — `Rc<RefCell<T>>` the classic combo. _(test)_
- [x] **arc1_basics** — `Arc<T>`, when to pick it over `Rc`. _(test)_
- [x] **smart_quiz** — capstone: tree of nodes with shared parents. _(test)_

## ✅ Chapter 14 — Concurrency  (8 / 8)

- [x] **thread1_spawn** — `thread::spawn`, `join`, the `'static` requirement. _(test)_
- [x] **thread2_move** — `move` closures, sending data into threads. _(test)_
- [x] **chan1_mpsc** — `mpsc::channel`, `send`/`recv`, dropping the sender. _(test)_
- [x] **chan2_iter** — receiver as iterator, `for msg in rx`. _(test)_
- [x] **mutex1_basics** — `Mutex<T>`, lock guard, automatic release. _(test)_
- [x] **arc_mutex** — sharing `Arc<Mutex<T>>` across threads. _(test)_
- [x] **send_sync** — `Send` and `Sync` as marker traits, what they mean. _(compile)_
- [x] **concurrency_quiz** — capstone: a thread-pool word counter. _(test)_

## ✅ Chapter 15 — Error handling at scale  (6 / 6)

- [x] **err1_custom_enum** — define your own `Error` enum with variants. _(test)_
- [x] **err2_display_error** — implement `Display` and `std::error::Error`. _(test)_
- [x] **err3_from_chain** — `impl From<ChildError>` to make `?` propagate. _(test)_
- [x] **err4_thiserror** — imitating the `thiserror` crate by hand. _(test)_
- [x] **err5_anyhow** — `anyhow::Result` patterns (manually). _(test)_
- [x] **errors_quiz_at_scale** — capstone: a CLI parser with full custom error types. _(test)_

## ✅ Chapter 16 — Closures & function traits  (6 / 6)

- [x] **closure1_basics** — capture by reference vs by value, `move` keyword. _(test)_
- [x] **closure2_traits** — `Fn`, `FnMut`, `FnOnce` — what each means. _(test)_
- [x] **closure3_returning** — returning a closure (`impl Fn` vs `Box<dyn Fn>`). _(test)_
- [x] **closure4_higher_order** — taking closures as parameters. _(test)_
- [x] **closure5_iterator_combo** — closures + iterators, the canonical pairing. _(test)_
- [x] **closures_quiz** — capstone: build a tiny event-handler registry. _(test)_

## ✅ Chapter 17 — Lifetimes deep-dive  (7 / 7)

- [x] **lt1_intro** — what `'a` actually is (a region of code). _(compile)_
- [x] **lt2_function** — explicit lifetimes on functions, `fn f<'a>(x: &'a T) -> &'a T`. _(test)_
- [x] **lt3_elision** — the three elision rules and when they apply. _(compile)_
- [x] **lt4_struct** — structs that hold references, `struct Foo<'a>`. _(test)_
- [x] **lt5_static** — `'static`, what it does and doesn't mean. _(test)_
- [x] **lt6_subtyping** — covariance, why `&'static T` can be passed for `&'short T`. _(test)_
- [x] **lifetimes_quiz** — capstone: a `Parser<'a>` that holds a borrowed input. _(test)_

## ✅ Chapter 18 — Async / await  (8 / 8)

Note: the runner is single-file, so this chapter teaches async by hand-rolling a minimal executor with `core::task` and `core::future` — every file embeds its own `mod runtime`. Real-world usage with `tokio` / `async-std` is covered in the commentary.

- [x] **async1_intro** — `async fn`, futures are lazy, you must `.await`. _(test)_
- [x] **async2_runtime** — what `block_on` / a Waker is doing under the hood. _(test)_
- [x] **async3_join** — concurrent (interleaved) await, `join2`. _(test)_
- [x] **async4_select** — racing futures with `select2`, async cancellation via Drop. _(test)_
- [x] **async5_spawn** — task queues, multi-task executors. _(test)_
- [x] **async6_send** — why spawning a future requires `Send`. _(compile)_
- [x] **async7_streams** — `Stream` trait, `poll_next`, async iteration. _(test)_
- [x] **async_quiz** — capstone: parallel HTTP-client-style fetch (mocked). _(test)_

## ✅ Chapter 19 — Macros  (6 / 6)

- [x] **macro1_println** — what `println!` is doing under the hood. _(compile)_
- [x] **macro2_macro_rules** — `macro_rules!`, single-rule macros. _(test)_
- [x] **macro3_repetition** — `$( ... ),*` and `+` repeaters. _(test)_
- [x] **macro4_hygiene** — variable hygiene, why macros don't accidentally bind names. _(test)_
- [x] **macro5_proc_taste** — read-only tour of a `#[derive(Debug)]` expansion. _(compile)_
- [x] **macros_quiz** — capstone: implement a `hashmap!{ k => v, ... }` macro. _(test)_

## ✅ Chapter 20 — Capstone project  (8 / 8)

A word-stats CLI tool built across eight exercises. Each file is self-contained (single-file compilation), but each subsequent file is "the previous one plus more". By the end you have a working text analyser exercising every chapter.

- [x] **proj1_skeleton** — bare-bones starter, the project plan. _(run)_
- [x] **proj2_io** — `read_input` that normalises text. _(test)_
- [x] **proj3_types** — domain types (`Stats` struct + `CountUnit` enum). _(test)_
- [x] **proj4_parse** — `count_basic`: lines / words / chars. _(test)_
- [x] **proj5_logic** — `top_words` via HashMap + sort. _(test)_
- [x] **proj6_concurrency** — `count_parallel` with `thread::scope`. _(test)_
- [x] **proj7_cli** — hand-rolled argv parser, subcommand-style flags. _(run)_
- [x] **proj_quiz** — final integration: `summarise(text, k)`. _(test)_

---

## Tally

|                                   |  done  | planned |
|-----------------------------------|:------:|:-------:|
| Chapter 1 — Intro & Variables     |   8    |    8    |
| Chapter 2 — Primitive types       |   6    |    6    |
| Chapter 3 — Functions & flow      |   6    |    6    |
| Chapter 4 — Ownership             |   8    |    8    |
| Chapter 5 — Strings               |   7    |    7    |
| Chapter 6 — Vec & HashMap         |   8    |    8    |
| Chapter 7 — Iterators             |   9    |    9    |
| Chapter 8 — Option / Result / `?` |   8    |    8    |
| Chapter 9 — Structs               |   7    |    7    |
| Chapter 10 — Enums & match        |   8    |    8    |
| Chapter 11 — Traits & generics    |   9    |    9    |
| Chapter 12 — Modules & Cargo      |   6    |    6    |
| Chapter 13 — Smart pointers       |   7    |    7    |
| Chapter 14 — Concurrency          |   8    |    8    |
| Chapter 15 — Errors at scale      |   6    |    6    |
| Chapter 16 — Closures             |   6    |    6    |
| Chapter 17 — Lifetimes deep-dive  |   7    |    7    |
| Chapter 18 — Async / await        |   8    |    8    |
| Chapter 19 — Macros               |   6    |    6    |
| Chapter 20 — Capstone project     |   8    |    8    |
| **Total**                         | **146** | **146** |

---

## Definition of "done" for each exercise

An exercise is only checkable when ALL of the following are true:

1. `exercises/CHAPTER/NAME.rs` exists with:
   - top-of-file teaching block,
   - `// I AM NOT DONE` marker,
   - `???` blanks the learner must fill in,
   - `#[cfg(test)] mod tests` block (for `test`-mode exercises).
2. `solutions/CHAPTER/NAME.rs` exists with:
   - the optimal answer in full,
   - commentary explaining *why* it's optimal,
   - alternatives a senior Rust dev would consider.
3. The solution, merged with the exercise's test module, compiles and all tests pass (the script at `/tmp/verify_solutions.sh` from chapter 1–3 verification can be re-used).
4. An entry exists in `info.toml` with `name`, `path`, `mode`, and a hint.
5. `master_rust list` shows the new exercise in the right order.
