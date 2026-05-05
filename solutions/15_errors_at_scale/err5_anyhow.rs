// SOLUTION — err5_anyhow (manually written, simulating the crate)

use std::fmt;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, DynError>;

#[derive(Debug)]
struct Errored {
    msg: String,
    source: DynError,
}

impl fmt::Display for Errored {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Errored {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

trait Context<T> {
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: Into<DynError>,
{
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        match self {
            Ok(v)  => Ok(v),
            Err(e) => Err(Box::new(Errored {
                msg: f(),
                source: e.into(),
            })),
        }
    }
}

fn parse_pair(s: &str) -> Result<(u32, u32)> {
    let (a, b) = s
        .split_once(',')
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "no comma",
        ))?;

    let a: u32 = a.parse::<u32>()
        .with_context(|| format!("parsing left half {:?}", a))?;
    let b: u32 = b.parse::<u32>()
        .with_context(|| format!("parsing right half {:?}", b))?;

    Ok((a, b))
}

// WHY THIS IS OPTIMAL FOR THIS LESSON:
//
//   `Box<dyn Error + Send + Sync + 'static>` IS what `anyhow::Error`
//   wraps. The blanket impl `impl<E: Error + Send + Sync + 'static>
//   From<E> for Box<dyn Error + Send + Sync + 'static>` is in std,
//   which is why `?` can shove ANY concrete error through this type
//   alias without a hand-written `From` impl. That's the whole trick.
//
//   The `Context` trait + the blanket `impl<T, E: Into<DynError>>`
//   is, again, almost line-for-line what `anyhow::Context` provides:
//   `.with_context(|| msg)` on any `Result` whose error converts
//   into our boxed type. The wrapper struct's `source()` exposes the
//   underlying error, so a pretty-printer walking the chain shows:
//
//       parsing left half "oops"
//         caused by: invalid digit found in string
//
//   The `Errored` Display prints ONLY its own message, deliberately.
//   The chain is recovered via `source()`, which lets logging layers
//   format it however they want (Debug `{:#?}`, single-line, etc.).
//   `anyhow::Error`'s `{:#}` formatter does exactly this walk.
//
// LESS IDIOMATIC ALTERNATIVES:
//
//   `Box<dyn Error>` (no Send/Sync/'static):
//   Compiles for single-threaded code, but won't cross thread
//   boundaries and can't be returned from `#[tokio::main]` functions.
//   `anyhow::Error` requires Send + Sync — copy that decision.
//
//   Hand-written `From` impls for every concrete error:
//   This is what err3 was about, and is the right move when you have
//   a NAMED enum. Boxing throws away variant identity but eliminates
//   the boilerplate — pick the trade based on whether your callers
//   need to match.
//
//   Stringifying errors with `e.to_string()` and tossing the source:
//   You lose the chain. The whole point of context is to ADD a layer
//   on top, not replace what's underneath.
//
//   Defining your own `bail!` / `ensure!` macros:
//   `anyhow::bail!("oh no")` and `anyhow::ensure!(x > 0, "...")` are
//   nice. Building them by hand here would distract — they're just
//   `return Err(anyhow!("..."))` and `if !cond { bail!(msg) }`.
//
// SUBTLETY:
//   `e.downcast_ref::<io::Error>()` works because the boxed dyn Error
//   carries the original concrete TypeId. That's how the test recovers
//   the exact `io::Error` we put in. `anyhow::Error` exposes the same
//   API. So even with type-erased errors, callers who DO want to
//   inspect a specific variant can reach in.
//
//   `Send + Sync + 'static` is the right bound set for almost every
//   application. The only reason to drop them is if you intentionally
//   want errors that hold borrowed data — exceedingly rare.
//
//   To migrate to the real crate:
//       cargo add anyhow
//       use anyhow::{Context, Result};
//   Drop the type aliases and the `Errored`/`Context` definitions.
//   The function bodies — `?`, `with_context`, error construction
//   via `.into()` — stay identical.
