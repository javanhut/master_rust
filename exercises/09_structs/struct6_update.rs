// =============================================================================
//  struct6 — struct-update syntax `..other` and field-init shorthand
// =============================================================================
//
// When you want to build a new struct that is "mostly the same as another
// one but with a couple of fields changed", you have two readable ways:
//
//   1. STRUCT-UPDATE SYNTAX `..other`
//
//      Foo { x: 1, ..other }
//
//      Means "fill in `x` from the literal, and copy ALL the rest from
//      `other`". The `..other` MUST come last and is the only thing allowed
//      after it.
//
//   2. FIELD-INIT SHORTHAND
//
//      `Foo { x }` is shorthand for `Foo { x: x }` — you've seen it before.
//      Often combined with `..other`:
//
//          let updated = User { email, ..old_user };
//
// MOVE / COPY CAVEAT (important!)
//
// `..other` MOVES the remaining fields out of `other`. If any of those fields
// is NOT `Copy`, `other` becomes partially-moved and you can no longer use
// it as a whole value afterwards. Each individual field that has NOT been
// moved is still usable on its own. (For pure-`Copy` types like `i32`, this
// caveat doesn't apply — copy is copy.)
//
// =============================================================================
//  YOUR TASK
// =============================================================================
// `Config` has THREE fields: `width`, `height`, `title`.
//   - `default_config()` returns a sensible default.
//   - `with_size(base, w, h)` returns a copy of `base` with new `width` and
//     `height`, and the original `title`. USE struct-update syntax `..base`.
//   - `with_title(base, title)` returns a copy of `base` with a new title.
//     USE struct-update syntax + field-init shorthand.

// I AM NOT DONE

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
    // Build a Config whose width/height come from `w`/`h` and whose title
    // comes from `base`. Use the `..base` struct-update form.
    Config {
        width:  ???,
        height: ???,
        ???
    }
}

fn with_title(base: Config, title: String) -> Config {
    // Use field-init shorthand for `title`, then `..base`.
    Config { ???, ???base }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn default_is_800x600_untitled() {
        let c = default_config();
        assert_eq!(c.width, 800);
        assert_eq!(c.height, 600);
        assert_eq!(c.title, "Untitled");
    }
    #[test] fn with_size_keeps_title() {
        let c = with_size(default_config(), 1920, 1080);
        assert_eq!(c.width, 1920);
        assert_eq!(c.height, 1080);
        assert_eq!(c.title, "Untitled");
    }
    #[test] fn with_title_keeps_size() {
        let c = with_title(default_config(), String::from("My App"));
        assert_eq!(c.width, 800);
        assert_eq!(c.height, 600);
        assert_eq!(c.title, "My App");
    }
    #[test] fn chained_updates() {
        let c = with_title(
            with_size(default_config(), 1024, 768),
            String::from("Hello"),
        );
        assert_eq!(c, Config { width: 1024, height: 768, title: String::from("Hello") });
    }
}

fn main() {}
