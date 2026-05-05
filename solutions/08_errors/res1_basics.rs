// SOLUTION — res1_basics

fn parse_u32(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(n)  => Ok(n),
        Err(_) => Err(format!("not a number: {}", s)),
    }
}

fn succeeded(r: &Result<u32, String>) -> bool {
    r.is_ok()
}

fn failed(r: &Result<u32, String>) -> bool {
    r.is_err()
}

fn force_ok(r: Result<u32, String>) -> u32 {
    r.unwrap()
}

fn force_ok_msg(r: Result<u32, String>) -> u32 {
    r.expect("must be Ok")
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `parse_u32` uses match because it's the most explicit way to show
//   the two-variant shape. We REPACKAGE the error: `parse::<u32>()`
//   gives us a `std::num::ParseIntError`, but our function's contract
//   says the error type is `String`, so we build a `String` describing
//   the failure. That's a totally normal thing to do — the standard
//   library's error types are diagnostic; your domain's error type
//   should speak your domain's language.
//
//   `is_ok` / `is_err` mirror `is_some` / `is_none`. Cheap discriminant
//   checks. Take the `Result` by reference (`&Result<...>`) when you
//   only need to inspect — no need to move it.
//
//   `unwrap` and `expect` on `Result` print the `E` value when they
//   panic. That's a HUGE difference from `Option::unwrap` — when a
//   `Result::unwrap` blows up in a logfile, you usually have enough
//   context to fix the bug.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   parse_u32 via combinators (next exercise!):
//       s.parse::<u32>().map_err(|_| format!("not a number: {}", s))
//     One line. We deliberately wrote the match form here to make the
//     two arms obvious.
//
//   parse_u32 returning `Result<u32, &'static str>`:
//     Doesn't compile — the error string includes `s`, which is borrowed
//     for the duration of the call only. You either own (`String`) or
//     return a static literal that doesn't reference inputs. We chose
//     `String` because the extra context is worth the allocation.
//
//   succeeded via match:
//       match r { Ok(_) => true, _ => false }
//     Three lines vs one. `is_ok` exists for a reason.
//
// SUBTLETY:
//   `.unwrap()` and `.expect()` MOVE the inner value out — `r` is
//   consumed. If you need the value AND want to keep the `Result`
//   around, reach for `r.as_ref()` first to borrow into it without
//   moving (you'll meet `as_ref` later in the chapter implicitly via
//   error-conversion patterns).
//
//   `.expect()` panics with the format `MSG: <Err value debug>`. So
//   `Err("boom").expect("must be Ok")` panics with
//   `must be Ok: "boom"` — including the inner value automatically.
