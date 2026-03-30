//! PPEG Core holds the logic for defining and parsing Parsing Expression Grammars (PEGs).
//! It includes the definitions for grammars, rules, expressions, and the parser itself.
//! It also includes support for Packrat parsing techniques that allow for the parsing of left recursive expressions.
//! To simplify the writing of PEGs, macros are provided for defining grammars, rules, and expressions in a more concise manner.
//! The core library is designed to be flexible and extensible, allowing for the creation of custom parsers and grammars.
//!
//! ## Example
//! ```rust
//! use ppeg_core::{grammar, rule, c, seq, parse};
//!
//! // Create a rule that matches the sequence "a" followed by "b".
//! let a_and_b = rule!("a_and_b" => seq!(c!("a"), c!("b")));
//! let peg = grammar!(
//!   // Insert a predefined rule.
//!   a_and_b,
//!   // Or define a rule inline.
//!   rule!("c" => c!("c"))
//! );
//!
//! // Parse the result and unwrap the error to get the remaining input and CST.
//! let (remaining, cst) = parse!(peg, &mut "ab", "a_and_b").unwrap();
//! ```

pub mod meta;
#[macro_use]
pub mod macros;
pub mod parser;
pub mod prelude;
