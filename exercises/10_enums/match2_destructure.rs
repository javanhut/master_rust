// =============================================================================
//  match2 — destructuring (structs, tuples, nested patterns)
// =============================================================================
//
// Patterns aren't only for enums. ANY composite value can be taken apart by
// pattern, both in `match` arms and in plain `let` bindings.
//
// TUPLES
//
//     let pair = (3, 7);
//     let (a, b) = pair;             // a = 3, b = 7
//
//     match pair {
//         (0, 0)        => "origin",
//         (x, 0) | (0, x) if x != 0 => "on an axis",
//         (x, y)        => "general",
//     }
//
// STRUCTS  (chapter 9 territory — recapping briefly)
//
//     struct Point { x: i32, y: i32 }
//
//     let p = Point { x: 3, y: 7 };
//     let Point { x, y } = p;        // shorthand: binds both
//     let Point { x: px, y: py } = p; // rename while binding
//
//     match p {
//         Point { x: 0, y: 0 } => "origin",
//         Point { x, y: 0 }    => "on the x-axis",     // y must be 0
//         Point { x, y }       => "elsewhere",
//     }
//
// USEFUL SHORTHAND: `..`
//   In a struct pattern, `..` means "I don't care about the rest of the
//   fields". `Point3 { x, .. }` ignores y and z.
//
// NESTED PATTERNS
//   Patterns compose. You can match an enum variant whose payload is a
//   struct whose field is a tuple — all in one pattern:
//
//     match shape {
//         Shape::Circle(Point { x, y }, r) => ...,
//         Shape::Pair((a, b))              => ...,
//         _                                => ...,
//     }
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// Two functions:
//
//   1. `quadrant((x, y)) -> &'static str`
//        Tuple destructure. Return:
//          "origin"   if both are 0
//          "x-axis"   if y == 0 (x != 0)
//          "y-axis"   if x == 0 (y != 0)
//          "Q1" Q2 Q3 Q4 by sign as usual (Q1: x>0,y>0; Q2: x<0,y>0; etc.)
//
//   2. `describe_point(p: &Point) -> &'static str`
//        Struct destructure. Return "origin" for (0,0), "on x-axis" if
//        y == 0, "on y-axis" if x == 0, otherwise "elsewhere".
//
// Replace every `???`. Do NOT touch the tests.

// I AM NOT DONE

#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn quadrant(p: (i32, i32)) -> &'static str {
    match p {
        (0, 0)               => "origin",
        (_, 0)               => "x-axis",
        (???, ???)           => "y-axis",
        (x, y) if x > 0 && y > 0 => "Q1",
        (x, y) if x < 0 && y > 0 => ???,
        (x, y) if x < 0 && y < 0 => ???,
        _                    => "Q4",
    }
}

fn describe_point(p: &Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x: _, y: 0 } => ???,
        ??? { x: 0, y: _ }   => "on y-axis",
        _                    => "elsewhere",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn quad() {
        assert_eq!(quadrant(( 0,  0)), "origin");
        assert_eq!(quadrant(( 3,  0)), "x-axis");
        assert_eq!(quadrant(( 0,  4)), "y-axis");
        assert_eq!(quadrant(( 1,  1)), "Q1");
        assert_eq!(quadrant((-1,  1)), "Q2");
        assert_eq!(quadrant((-1, -1)), "Q3");
        assert_eq!(quadrant(( 1, -1)), "Q4");
    }
    #[test] fn pt() {
        assert_eq!(describe_point(&Point { x: 0, y: 0 }), "origin");
        assert_eq!(describe_point(&Point { x: 5, y: 0 }), "on x-axis");
        assert_eq!(describe_point(&Point { x: 0, y: 9 }), "on y-axis");
        assert_eq!(describe_point(&Point { x: 1, y: 1 }), "elsewhere");
    }
}

fn main() {}
