// SOLUTION — opt3_combinators

fn add_one(opt: Option<i32>) -> Option<i32> {
    opt.map(|n| n + 1)
}

fn default_zero(opt: Option<i32>) -> i32 {
    opt.unwrap_or(0)
}

fn or_compute(opt: Option<i32>) -> i32 {
    opt.unwrap_or_else(|| 100 - 1)
}

fn default_t(opt: Option<String>) -> String {
    opt.unwrap_or_default()
}

fn pipeline(s: &str) -> i32 {
    s.parse::<i32>().ok()
        .map(|n| n * 2)
        .unwrap_or(-1)
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `map` is the workhorse. "Apply this function to the value if there
//   is one; do nothing if there isn't." Reading `opt.map(|n| n + 1)`
//   takes a fraction of the cognitive load of the equivalent match.
//
//   `unwrap_or(0)` is pure substitution: the fallback value is already
//   computed, plug it in if needed. Cheap, eager, simple.
//
//   `unwrap_or_else(|| ...)` defers the fallback until it's actually
//   needed. The literal `100 - 1` here is trivial — but in real code
//   the fallback might be `read_config_file()` or `Vec::with_capacity(
//   1024)`. Don't pay for it on the happy path.
//
//   `unwrap_or_default()` quietly leans on the `Default` trait. Every
//   primitive number defaults to 0, `String` to "", `Vec<T>` to empty,
//   `bool` to false. Use it when "the type's zero value" is exactly
//   what you'd write anyway — saves a literal and survives type changes.
//
//   `pipeline` shows the real win: a Result-or-Option pipeline reads
//   left-to-right as a sequence of transformations.
//     `.parse()`     produces `Result<i32, _>`
//     `.ok()`        discards the error, becomes `Option<i32>`
//     `.map(|n|...)` doubles inside Some
//     `.unwrap_or(-1)` extracts or substitutes the sentinel
//   The match version of this is a 6-line nested mess; the combinator
//   chain is one logical sentence.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   add_one via match:
//       match opt { Some(n) => Some(n+1), None => None }
//     Three times longer. The whole point of `map` is to retire that.
//
//   default_zero via .unwrap_or_default():
//       opt.unwrap_or_default()
//     Works (`i32::default() == 0`), but `unwrap_or(0)` is more
//     explicit at the call site — readers don't have to know what the
//     default is. Use `_default` when "type's zero" is the *concept*
//     you mean, not just a value that happens to equal it.
//
//   or_compute via unwrap_or:
//       opt.unwrap_or(100 - 1)
//     Equivalent here because `100 - 1` is a const expression. With a
//     real expensive fallback you'd want `_else` and only `_else`.
//
// SUBTLETY:
//   `.and_then(f)` (not exercised here, but mentioned in the lesson)
//   is `map` for cases where `f` itself returns `Option<U>`. Without
//   `and_then` you'd get `Option<Option<U>>`; `and_then` flattens.
//   Same trick `Result` plays with its own `.and_then`. You'll meet
//   it again as `flatMap` in any FP language and as the monadic bind
//   in mathematics — same idea, different name.
