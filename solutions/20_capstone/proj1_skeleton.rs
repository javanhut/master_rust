// SOLUTION — proj1_skeleton

fn main() {
    let project_name: &str = "wordstat";
    println!("starting capstone project: {project_name}");
}

// WHY THIS IS OPTIMAL:
//
//   The first exercise is a placeholder — its job is to make you open the
//   file, read the chapter plan, and confirm the toolchain still works.
//   The `&str` annotation is intentional: by the time you finish chapter
//   17 you should be reflexively reaching for borrowed string slices over
//   `String` for short-lived data.
//
// ALTERNATIVES:
//
//   - `let project_name = String::from("wordstat");` — works but allocates
//     on the heap for no reason. The literal lives in the binary's
//     read-only data; a `&'static str` is free.
//
//   - `const PROJECT_NAME: &str = "wordstat";` — equally fine, and a
//     better fit if the value never changes. We use `let` only because
//     subsequent files will pass the binding around.
//
// KEY TAKEAWAYS:
//
//   - String literals are `&'static str`. They're the cheapest string
//     type in Rust.
//   - `println!` is a macro (chapter 19). The `{project_name}` capture
//     syntax has been stable since Rust 1.58.
