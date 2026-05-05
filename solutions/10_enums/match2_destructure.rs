// SOLUTION — match2_destructure

#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn quadrant(p: (i32, i32)) -> &'static str {
    match p {
        (0, 0)                   => "origin",
        (_, 0)                   => "x-axis",
        (0, _)                   => "y-axis",
        (x, y) if x > 0 && y > 0 => "Q1",
        (x, y) if x < 0 && y > 0 => "Q2",
        (x, y) if x < 0 && y < 0 => "Q3",
        _                        => "Q4",
    }
}

fn describe_point(p: &Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x: _, y: 0 } => "on x-axis",
        Point { x: 0, y: _ } => "on y-axis",
        _                    => "elsewhere",
    }
}

// WHY THIS IS OPTIMAL:
//
//   `quadrant` shows a clean priority cascade. The first three arms peel
//   off the "on an axis" cases via literal-position patterns (0 in the
//   relevant slot, `_` in the other) — no arithmetic, no `if`. Only when
//   all axes are excluded do we use guards (`if x > 0 && y > 0`) to
//   carve up the four quadrants. The final `_` catches Q4 since at that
//   point we know `x > 0 && y < 0` is the only remaining possibility.
//
//   `describe_point` mirrors the same shape using struct patterns.
//   `Point { x: 0, y: 0 }` matches a Point whose fields equal those
//   literals; `Point { x: _, y: 0 }` matches any Point with y == 0; and
//   so on.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   if/else chains on `p.0` and `p.1`
//     - Verbose, repeats yourself, and you lose the at-a-glance table
//       layout.
//
//   `Point { x, y }` then if-elsing on `x` and `y`
//     - Works, but you've moved the "what shape are we?" decision out
//       of the pattern and into procedural code. Less Rust-y.
//
// SUBTLETY:
//   In `Point { x: _, y: 0 }`, the `_` MEANS "I don't care about x".
//   Compare with `Point { x, y: 0 }` which BINDS x to a fresh variable
//   — same semantics for matching, different semantics for the arm
//   body. Use `_` when you won't use the value; binding when you will.
//
//   Field shorthand: `Point { x, y }` is sugar for `Point { x: x, y: y }`.
