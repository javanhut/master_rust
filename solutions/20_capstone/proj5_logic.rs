// SOLUTION — proj5_logic

use std::collections::HashMap;

pub fn top_words(text: &str, k: usize) -> Vec<(String, u32)> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_string()).or_insert(0) += 1;
    }

    let mut v: Vec<(String, u32)> = counts.into_iter().collect();

    v.sort_by(|(wa, ca), (wb, cb)| {
        cb.cmp(ca).then_with(|| wa.cmp(wb))
    });

    v.into_iter().take(k).collect()
}

// WHY THIS IS OPTIMAL:
//
//   The shape is "build a map, dump to Vec, sort, truncate" — the textbook
//   top-K-by-frequency recipe.
//
//   `entry(key).or_insert(0)` is the canonical "increment a counter"
//   pattern in Rust. It does ONE hash lookup whether the key is present
//   or absent. Compare to the naive form:
//
//       if let Some(v) = counts.get_mut(&w) { *v += 1; }
//       else { counts.insert(w.to_string(), 1); }
//
//   That's two lookups in the miss case and clones the key on every
//   insertion path. Entry is strictly better.
//
//   `into_iter().collect()` on a HashMap consumes it and yields owned
//   `(K, V)` pairs — exactly what `.sort_by` wants. We don't need the
//   map afterwards, so consuming it is free.
//
//   The comparator is two lines and does the right thing:
//
//     cb.cmp(ca)              -> primary: counts descending
//     .then_with(|| wa.cmp(wb))  -> tie:    words ascending
//
//   `then_with` takes a closure (lazy) — only called when the primary
//   compares equal.
//
//   `.take(k)` works for ANY k including zero or larger-than-len. The
//   iterator simply stops early or runs to completion. No bounds check
//   needed.
//
// ALTERNATIVES:
//
//   1. `BinaryHeap` of `(Reverse(count), Reverse(word))` of size k. For
//      huge inputs and small k this is O(n log k) instead of O(n log n)
//      and uses k memory instead of n. The `std::collections::BinaryHeap`
//      is a max-heap; you wrap with `Reverse` for a min-heap.
//
//   2. `sort_unstable_by` — a tiny win on speed, fine here because we
//      handle ties explicitly in the comparator. Stable sort is the
//      default; pick `_unstable` when you don't care about equal-order
//      preservation (which we don't, since we explicitly tiebreak).
//
//   3. Use `&str` keys backed by an arena so we don't clone every word
//      via `to_string()`. For our scale it doesn't matter; for a real
//      indexer this is the next optimisation.
//
// KEY TAKEAWAYS:
//
//   - The entry API is the right hammer for "create-or-update" on a map.
//   - `Ordering::then_with` is how you chain comparators cleanly.
//   - Reverse a sort by reversing operand order in `cmp`, OR by calling
//     `.reverse()` on the resulting Ordering. Both work; the former
//     reads more directly.
