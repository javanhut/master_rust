// SOLUTION — match4_iflet_whilelet

fn double_or_zero(opt: Option<i32>) -> i32 {
    if let Some(x) = opt {
        x * 2
    } else {
        0
    }
}

fn parse_or_zero(s: &str) -> i32 {
    let Ok(n) = s.parse::<i32>() else {
        return 0;
    };
    n
}

fn pop_all(v: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    while let Some(x) = v.pop() {
        sum += x;
    }
    sum
}

// WHY THIS IS OPTIMAL:
//
//   Each form is the right tool for its shape:
//
//   - `if let Some(x) = opt { ... } else { ... }` — exactly two paths,
//     one needs the bound value and the other doesn't. A full `match`
//     here would just add visual noise.
//
//   - `let Ok(n) = ... else { return 0; };` — the canonical "extract or
//     bail" guard. After this line, `n` is in scope for the rest of the
//     function with NO indentation pyramid. This is the crown jewel of
//     `let else`.
//
//   - `while let Some(x) = v.pop()` — `Vec::pop` returns `Option<T>`:
//     `Some(last)` while there's an element, `None` when empty. The
//     loop exits on the first `None` automatically. No manual length
//     bookkeeping; no off-by-one.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `double_or_zero` via `opt.map_or(0, |x| x * 2)`
//     - Tighter and arguably nicer for this exact case. We used `if let`
//       to drill the syntax.
//
//   `parse_or_zero` via `s.parse::<i32>().unwrap_or(0)`
//     - One-liner, faster to type, and clearer. `let else` shines on
//       larger functions where you want the bail-out at the top and the
//       happy path running unindented for many lines.
//
//   `pop_all` via a `for` loop
//     - You can't `for` over a Vec while popping it without an iterator
//       gymnastics; `while let` is exactly the construct for "consume
//       until None".
//
// SUBTLETY:
//   `let else` requires the else block to DIVERGE — that is, never
//   produce a value for the surrounding scope. Allowed: `return`,
//   `break`, `continue`, `panic!`, calls to `-> !` functions. Not
//   allowed: a regular value or fall-through. The compiler enforces
//   this so the bound variable is guaranteed to be initialised after
//   the statement.
