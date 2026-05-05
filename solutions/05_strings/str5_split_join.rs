// SOLUTION — str5_split_join

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

fn clean(s: &str) -> String {
    s.trim().to_string()
}

fn csv_upper(s: &str) -> String {
    s.split(',')
        .map(|piece| piece.to_uppercase())
        .collect::<Vec<_>>()
        .join(",")
}

fn nth_line(s: &str, n: usize) -> Option<&str> {
    s.lines().nth(n)
}

// WHY THIS IS OPTIMAL:
//
//   word_count -> split_whitespace
//       `split_whitespace` is exactly the function for this question:
//          - splits on any Unicode whitespace,
//          - collapses consecutive whitespace,
//          - never yields empty pieces at the ends.
//       `s.split(' ').count()` would WRONGLY count "  a  b" as 5 because
//       it would yield two empties for the leading double space and one
//       between a and b. Almost always wrong for natural text.
//
//   clean -> trim().to_string()
//       `trim` returns a `&str` view (no allocation). `to_string` then
//       owns it. This is the standard "give me a clean owned copy" pattern.
//
//   csv_upper -> split + map + collect::<Vec<_>> + join
//       Canonical functional pipeline. The turbofish `::<Vec<_>>` is needed
//       because `collect` is generic over the target collection type and
//       `join` needs a concrete `Vec<String>` (or `Vec<&str>`) to work on.
//       Trying to call `.join` directly on an iterator does not compile
//       in std (it's an `itertools` extension).
//
//   nth_line -> lines().nth(n)
//       `lines()` cleanly handles both \n and \r\n endings, so the CRLF
//       test passes without any extra logic. Always prefer it over
//       `split('\n')` for line iteration.
//
// ALTERNATIVES:
//
//   csv_upper, allocation-light:
//       use std::fmt::Write;
//       let mut out = String::with_capacity(s.len());
//       for (i, p) in s.split(',').enumerate() {
//           if i > 0 { out.push(','); }
//           for c in p.chars() { for u in c.to_uppercase() { out.push(u); } }
//       }
//       Faster, uglier. Reach for it only after profiling proves it matters.
