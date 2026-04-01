//! Prelude uses a series of crates from the core library to quickly and easily
//! export all functionality for creating a parser using the PPEG library.
//!
//! ```rust
//! use ppeg_core::prelude::*;
//! ```

pub use crate::meta::{error::MetaError, parser::Meta};
pub use crate::parser::{
  context::Context,
  cst::{CST, Label},
  error::ParserError,
  expression::Expression,
  grammar::Grammar,
  packrat::Packrat,
  peg::Parser,
  rule::Rule,
};
