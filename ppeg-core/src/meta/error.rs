use std::fmt;

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

impl fmt::Display for MetaError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
