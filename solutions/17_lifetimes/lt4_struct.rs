// SOLUTION — lt4_struct

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn first_sentence(s: &str) -> Excerpt<'_> {
        let end = s.find('.').unwrap_or(s.len());
        Excerpt { part: &s[..end] }
    }

    fn part(&self) -> &str {
        self.part
    }

    fn len(&self) -> usize {
        self.part.len()
    }
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   The struct definition is the heart of the chapter:
//
//       struct Excerpt<'a> {
//           part: &'a str,
//       }
//
//   That `<'a>` is mandatory. A reference field can't have an "inferred"
//   lifetime at the type level — every `Excerpt` value carries with it
//   the region of code its `part` is valid for. The compiler uses that
//   region to bound how long any given `Excerpt` instance can live.
//
//   The constructor `first_sentence(s: &str) -> Excerpt<'_>` uses Rule 2
//   of elision: one input reference, output borrows from it. The `'_`
//   in `Excerpt<'_>` means "infer the lifetime parameter" — same idea
//   as `&'_ str`. It's signposting "this thing has a lifetime, please
//   work it out from context."
//
//   `part` and `len` are methods on `&self`. Inside the impl block we
//   wrote `impl<'a> Excerpt<'a>` — `'a` is declared once after `impl`
//   and used once on the type. Rule 3 then elides the lifetimes inside
//   the methods themselves, so we don't have to write them again.
//
// WHAT THE COMPILER PREVENTS:
//
//   The whole reason to write `<'a>` on a struct is to make this kind of
//   bug impossible at compile time:
//
//       fn make_dangling() -> Excerpt<'?> {
//           let s = String::from("Hello.");
//           Excerpt::first_sentence(&s)   // borrows from `s`...
//       }                                  // ...but `s` dies here.
//
//   The compiler refuses: the returned `Excerpt` would outlive `s`.
//   There's no lifetime you can write that makes the function valid.
//
// ALTERNATIVES:
//
//   1. Owned variant — drop the lifetime entirely:
//
//          struct OwnedExcerpt { part: String }
//          impl OwnedExcerpt {
//              fn first_sentence(s: &str) -> OwnedExcerpt {
//                  let end = s.find('.').unwrap_or(s.len());
//                  OwnedExcerpt { part: s[..end].to_string() }
//              }
//          }
//
//      Trades one allocation for total freedom from lifetime constraints.
//      Many real-world types start out borrowed and switch to owned the
//      moment the borrowing pattern becomes a hassle.
//
//   2. Range form — store byte offsets instead of a slice:
//
//          struct ExcerptRange { start: usize, end: usize }
//
//      The struct is now `'static`-able — no lifetime — but you must pass
//      the original string back in every time you want to read the part.
//      Useful in column-oriented or arena-allocated code.
//
//   3. `Cow<'a, str>` for "either borrowed or owned, decided at runtime."
//      Out of scope here.
