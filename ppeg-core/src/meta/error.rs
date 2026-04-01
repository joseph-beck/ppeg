//! ## Example
//! ```rust
//! use ppeg_core::prelude::*;
//! ```

use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
};

// MetaError
#[derive(Debug, Clone, PartialEq)]
pub enum MetaError {
  // Occurs when the input given to meta is invalid.
  InvalidInput { position: usize, input: String },
  // Occurs when a rule is invalid.
  InvalidRule { position: usize, rule: String },
  // Occurs when an expression is invalid.
  InvalidExpression { position: usize, expression: String },
  // Catch all unknown error.
  Unknown,
}

impl Display for MetaError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      MetaError::InvalidInput { position, input } => {
        write!(f, "Error: invalid input at position {}: '{}'", position, input)
      }
      MetaError::InvalidRule { position, rule } => {
        write!(f, "Error: invalid rule at position {}: '{}'", position, rule)
      }
      MetaError::InvalidExpression { position, expression } => {
        write!(
          f,
          "Error: invalid expression at position {}: '{}'",
          position, expression
        )
      }
      MetaError::Unknown => write!(f, "Error: unknown"),
    }
  }
}

impl Error for MetaError {}
