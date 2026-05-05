// SOLUTION — proj2_io

fn read_input(input: &str) -> String {
    let unix = input.replace("\r\n", "\n");
    unix.trim().to_string()
}

// WHY THIS IS OPTIMAL:
//
//   `str::replace` returns a fresh `String` — it allocates exactly once
//   and walks the input once. `trim()` then yields a `&str` slice into
//   that String, but `to_string()` copies it into a new owned String
//   (which we need, because returning a `&str` here would borrow from
//   `unix`, which is a local).
//
//   We could in principle do the whole thing in one allocation:
//
//       let trimmed = input.trim();
//       trimmed.replace("\r\n", "\n")
//
//   That's one fewer copy. For the course we kept the two-step form
//   because it reads top-to-bottom like the spec ("first fix line
//   endings, then trim"). Either is fine.
//
// ALTERNATIVES:
//
//   - Take `String` instead of `&str`. The function would consume its
//     argument and could mutate in place — but every caller would have
//     to either move-or-clone its String. Taking `&str` is the
//     idiomatic, more flexible choice.
//
//   - Return `&str` instead of `String`. Impossible here because we
//     manufactured the buffer locally; the borrow checker would refuse
//     to let it escape (chapter 17).
//
//   - Use `BufReader::new(stdin()).lines()` to stream line-by-line.
//     Better for huge files, but `read_input` would change shape. We
//     keep this layer flat-and-eager since later stages already accept
//     a single `&str`.
//
// KEY TAKEAWAYS:
//
//   - The IO boundary is where IO errors live. By the time text reaches
//     `count_basic` (proj4) it's a clean `&str`, and the rest of the
//     program is total.
//   - `replace`, `trim`, `to_string` are the three str-method workhorses.
//     Look them up. They have variants — `trim_start`, `trim_end_matches`,
//     `replacen` — that pay back many times over.
