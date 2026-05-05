// SOLUTION — intro2_print
//
// Three placeholders, three different patterns:
//   - precision flag for floats:        {:.2}
//   - capture-from-scope named args:    {name}, {age}
//   - Debug formatting for arrays:      {:?}

fn main() {
    let pi = 3.14159f64;
    let name = "Ada";
    let age = 36;
    let bytes = [1, 2, 3];

    println!("pi ≈ {:.2}", pi);
    println!("{name} is {age}");
    println!("bytes: {:?}", bytes);
}

// WHY THIS IS OPTIMAL:
//
//   {:.2}    is the standard "two decimal places" precision specifier; it
//            is checked at compile time so a typo will not blow up at
//            runtime.
//
//   {name}   captures the local variable directly. Available since Rust
//            1.58 and now the dominant style — clearer than positional
//            arguments and safer than re-listing them after the format
//            string. Older code you'll see uses `"{}", name` but you
//            should write the captured form for new code.
//
//   {:?}     is the Debug formatter. Arrays do NOT implement Display, so
//            {:?} is required here. For deeply nested data prefer {:#?}.
//
// COMMON SLIPS:
//
//   - Forgetting the `:` in `{:.2}` — `{.2}` is a parse error.
//   - Trying `{}` on the array — only Debug is implemented for [T; N].
//   - Mixing positional and capture: `println!("{0} {age}", name)` works,
//     but pick one style per call.
