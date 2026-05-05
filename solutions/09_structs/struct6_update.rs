// SOLUTION — struct6_update

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    width:  u32,
    height: u32,
    title:  String,
}

fn default_config() -> Config {
    Config {
        width:  800,
        height: 600,
        title:  String::from("Untitled"),
    }
}

fn with_size(base: Config, w: u32, h: u32) -> Config {
    Config {
        width:  w,
        height: h,
        ..base
    }
}

fn with_title(base: Config, title: String) -> Config {
    Config { title, ..base }
}

// WHY THIS IS OPTIMAL:
//
//   Struct-update syntax `..base` is the right tool whenever you want
//   "everything else, unchanged". It scales: add a fourth field to `Config`
//   and `with_size` / `with_title` keep working without edits.
//
//   `with_size` overrides `width` and `height` and pulls `title` out of
//   `base` — no `String::clone`, no allocation, just a move of the existing
//   `String` buffer.
//
//   `with_title` combines field-init shorthand (`title` instead of
//   `title: title`) with `..base` to express the intent in three tokens.
//
//   Both functions consume `base` by value because they need to MOVE the
//   `String` out of it. Trying to do this with `&base` would force a
//   clone — measurable, avoidable, and the wrong default.
//
// EQUIVALENT BUT NOISIER:
//
//   fn with_size(base: Config, w: u32, h: u32) -> Config {
//       Config { width: w, height: h, title: base.title }
//   }
//   Works for three fields. Add a fourth and you must remember to add it
//   here too. `..base` removes that maintenance burden.
//
//   fn with_title(base: &Config, title: String) -> Config {
//       Config { title, width: base.width, height: base.height,
//                /* but title can't be `..base` because we'd need
//                   base.title, which would force a clone */ }
//   }
//   This is the form you reach for when callers genuinely cannot give up
//   ownership — but it does cost more (every non-Copy field must be cloned).
