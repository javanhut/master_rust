// SOLUTION — vec5_slicing

fn middle(v: &Vec<i32>) -> &[i32] {
    &v[1..v.len() - 1]
}

fn head(v: &Vec<i32>) -> Option<i32> {
    v.first().copied()
}

fn tail(v: &Vec<i32>) -> Option<i32> {
    v.last().copied()
}

fn chunk_sums(v: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for c in v.chunks(n) {
        let mut s = 0;
        for &x in c {
            s += x;
        }
        out.push(s);
    }
    out
}

// WHY THESE ARE OPTIMAL:
//
//   `&v[1..v.len() - 1]` returns `&[i32]` — a borrowed view, zero allocation,
//   zero copying. The borrow checker tracks that the slice can't outlive the
//   Vec it points into.
//
//   `v.first()` / `v.last()` are clearer than `v.get(0)` / `v.get(v.len()-1)`
//   and they handle the empty case automatically (returning `None`). Pairing
//   with `.copied()` turns `Option<&T>` into `Option<T>` for `T: Copy`.
//
//   `v.chunks(n)` yields sub-slices of length n (the last may be shorter).
//   No allocation per chunk — each sub-slice is (pointer, length) into the
//   original buffer.
//
// SIGNATURE NOTE — `&Vec<T>` vs `&[T]`:
//
//   We took `&Vec<i32>` here to match the rest of the chapter's exercises,
//   but production code prefers `&[i32]`. A `&Vec<T>` only accepts Vecs;
//   `&[T]` accepts Vecs, arrays, and other slices. Clippy will tell you so.
//   Internally `&Vec<T>` deref-coerces to `&[T]` automatically when calling
//   slice methods, which is why `v.first()`, `v.last()`, and `v.chunks(n)`
//   all work directly on a Vec reference.
//
// ITERATOR PREVIEW (chapter 7):
//
//     fn chunk_sums(v: &[i32], n: usize) -> Vec<i32> {
//         v.chunks(n).map(|c| c.iter().sum()).collect()
//     }
//   One line, same machine code. We're going long-form here on purpose.
//
// COUSIN METHODS WORTH KNOWING:
//
//     v.split_at(i)    -> (&[T], &[T])     // two halves
//     v.windows(n)     -> &[T] views, OVERLAPPING (chunks doesn't overlap)
//     v.chunks_exact(n)-> only full-size chunks; remainder via .remainder()
