#[derive(Debug, Clone)]
pub struct Boss {
    pub name: String,
    pub power: u32,
}




// Here is a bullet list PRINTING ALTERNATIVES  available in Rust:

// - `print!()`: Prints text to stdout without a newline at the end.
// - `println!()`: Prints text to stdout with a newline appended.
// - `eprint!()`: Prints text to stderr without a newline.
// - `eprintln!()`: Prints text to stderr with a newline.
// - `format!()`: Writes formatted text into a `String` instead of printing.
// - `{:?}` with `Debug` trait: Developer-focused debug printing.
// - `{:#?}` with `Debug` trait: Pretty-printed, multi-line debug output for readability.
// - `{}` with `Display` trait: User-friendly output, requires manual implementation.
// - `dbg!()`: Debug macro that prints an expression with source location info, useful during development.
// - Custom formatters for hexadecimal, binary, octal, etc. using traits like `LowerHex`, `UpperHex`, `Binary`, `Octal`.

// These cover a range of use cases from simple text output, error messages, formatted strings, debugging, and user-facing display.[1][2][4]

// [1](https://www.w3schools.com/rust/rust_output.php)
// [2](https://doc.rust-lang.org/rust-by-example/hello/print.html)
// [3](https://www.programiz.com/rust/print-output)
// [4](https://www.nayab.xyz/programming/rust-formatterd-print.html)
// [5](https://doc.rust-lang.org/book/ch03-02-data-types.html)
// [6](https://en.wikipedia.org/wiki/Rust_(programming_language))
// [7](https://www.geeksforgeeks.org/rust/introduction-to-rust-programming-language/)
// [8](https://doc.rust-lang.org/std/macro.println.html)
// [9](https://labex.io/tutorials/rust-printable-types-in-rust-s-standard-library-99187)