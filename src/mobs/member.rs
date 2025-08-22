
#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub skill: String,
}


// When you see #[derive(Debug, Clone)] in Rust code, you should think:

//     The author wants the type to be printable easily for debugging.
//...........what other alternatives of printing are available, and when are they used?
//............

//     The author wants the ability to explicitly duplicate or copy the value using .clone().

// Besides Debug and Clone, Rust offers many other common traits that you can derive automatically, such as:

//     Copy: for types that have "copy semantics" (cheap and implicit copy).

//     PartialEq and Eq: for equality comparisons.

//     PartialOrd and Ord: for ordering comparisons.

//     Hash: for hashing support.

//     Default: to create a default value for the type.
