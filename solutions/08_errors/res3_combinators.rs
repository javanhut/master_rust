// SOLUTION — res3_combinators

fn double(r: Result<i32, String>) -> Result<i32, String> {
    r.map(|n| n * 2)
}

fn tag_error(r: Result<i32, String>) -> Result<i32, String> {
    r.map_err(|e| format!("error: {}", e))
}

fn to_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

fn to_err_option(r: Result<i32, String>) -> Option<String> {
    r.err()
}

fn recover(r: Result<i32, String>) -> Result<i32, String> {
    r.or_else(|e| e.parse::<i32>().map_err(|_| e))
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `map` on Result is exactly `map` on Option, just with the extra Err
//   case threading through unchanged. Use it whenever you have a pure
//   function that turns `T` into `U` and you want to apply it to the
//   value if it's there.
//
//   `map_err` is the dual — transform the error while leaving success
//   alone. This is the CANONICAL place to translate a low-level error
//   type into your domain's error type. It pairs naturally with `?`:
//
//       fn load() -> Result<Config, MyError> {
//           let s = std::fs::read_to_string("c.toml")
//               .map_err(|e| MyError::Io(e.to_string()))?;
//           ...
//       }
//
//   `.ok()` and `.err()` pivot between Result and Option. Use `.ok()`
//   when you've decided the failure reason doesn't matter — for
//   example, "best effort, fall back if any failure". Use `.err()`
//   when you only want to inspect or log the failure.
//
//   `recover` is the showcase for `or_else`. We accept a Result that
//   already has a value (great, return as-is) OR an error string we
//   try to parse as an integer. The closure gets the original error,
//   tries `.parse::<i32>()`, and on inner failure passes through the
//   ORIGINAL error string with `.map_err(|_| e)`. The result is:
//     Ok(3)         -> Ok(3)         pass-through.
//     Err("42")     -> Ok(42)        recovered by parsing the message.
//     Err("xx")     -> Err("xx")     parse failed, original error wins.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   double via match:
//       match r { Ok(n) => Ok(n*2), Err(e) => Err(e) }
//     `map` exists to retire that.
//
//   to_option via match:
//       match r { Ok(v) => Some(v), Err(_) => None }
//     `.ok()` is one method call.
//
//   recover via if let:
//       if let Err(e) = r {
//           match e.parse::<i32>() {
//               Ok(n)  => Ok(n),
//               Err(_) => Err(e),
//           }
//       } else { r }
//     Functional, but eight lines vs one. The combinator chain reads
//     as a sentence: "or else, try parsing the error, mapping inner
//     failures back to the original message".
//
// SUBTLETY:
//   `or_else` requires the closure to return `Result<T, F>` where T
//   matches the original `T`. The new Err type `F` does NOT have to
//   match the old `E`. That's why "translate-on-recover" patterns are
//   typically `.or_else(|e| ... .map_err(|_| e.into_my_error()))`.
//
//   `.ok()` discards information. If a caller might want the failure
//   reason, prefer to keep the Result and let them decide. Reach for
//   `.ok()` only at boundaries where "give me Some or nothing" is
//   genuinely the right shape.
