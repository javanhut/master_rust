// SOLUTION — str2_create

fn make_from() -> String {
    String::from("hi")
}

fn make_to_string() -> String {
    "hi".to_string()
}

fn make_to_owned() -> String {
    "hi".to_owned()
}

fn make_empty_with_capacity() -> String {
    String::with_capacity(64)
}

// WHY EACH SPELLING EXISTS AND WHEN TO PREFER IT:
//
//   String::from("hi")
//       The `From` trait is the fundamental conversion mechanism in Rust.
//       Reads as "make a String *from* this thing". Crystal clear at a glance.
//
//   "hi".to_string()
//       Comes from the `ToString` trait, which is BLANKET-implemented for
//       every type that implements `Display`. Means it works on numbers,
//       booleans, your own types — anything printable.
//           42.to_string()           // "42"
//           true.to_string()         // "true"
//
//   "hi".to_owned()
//       Comes from the `ToOwned` trait. Generic over "things with a borrowed
//       view". Use this in code that's generic over `T: ?Sized + ToOwned`,
//       or when working with slices / Path / OsStr / etc.
//
//   String::with_capacity(64)
//       Allocates the buffer up front but length stays 0. Use this when you
//       are about to `push_str` a lot and you have a size estimate — it
//       avoids the reallocation-and-copy you'd otherwise pay each time the
//       buffer doubles. Pure performance optimisation; identical OBSERVABLE
//       behaviour to `String::new()` until you start writing.
//
// ALTERNATIVES:
//   "hi".into()         — works because the target type `String` is fixed by
//                         the function return signature; the compiler picks
//                         `Into<String> for &str` automatically.
//   format!("hi")       — fine but heavier; the formatter machinery is
//                         overkill for a static literal. Use format! when
//                         you're ACTUALLY interpolating (str3).
