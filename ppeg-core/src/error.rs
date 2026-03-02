use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
  /// Occurs when an expression fails to match the given rule with the given input.
  FailedToMatch {
    position: usize,
    input: &'a str,
    name: &'a str,
  },
  /// Occurs when the given expression is invalid.
  InvalidExpression { position: usize, name: &'a str },
  /// Occurs when trying to consume when at the end of the input.
  EndOfInput { position: usize, input: Option<&'a str> },
  /// Occurs when trying to access a rule that does not exist in the grammar.
  RuleNotFound { position: usize, name: &'a str },
  /// Occurs when something unexpected happens.
  Unexpected { position: usize },
  /// Left recursion has been detected.
  /// This may not result in a parsing error.
  /// It is flagged for seeding the expression.
  LeftRecursion { name: &'a str },
  /// Catch all unknown error.
  Unknown,
}

impl fmt::Display for ParserError<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParserError::FailedToMatch { position, input, name } => write!(
        f,
        "Error: failed to match rule '{}' at position {}: remaining input '{}'",
        name, position, input
      ),
      ParserError::InvalidExpression { position, name } => {
        write!(
          f,
          "Error: invalid expression in rule '{}' at position {}",
          name, position
        )
      }
      ParserError::EndOfInput { position, input } => match input {
        Some(remaining) => write!(
          f,
          "Error: end of input at position {}: remaining input '{}'",
          position, remaining
        ),
        None => write!(f, "Error: end of input at position {}", position),
      },
      ParserError::RuleNotFound { position, name } => {
        write!(
          f,
          "Error: rule '{}' not found in grammar at position {}",
          name, position
        )
      }
      ParserError::Unexpected { position } => {
        write!(f, "Error: unexpected at position {}", position)
      }
      ParserError::LeftRecursion { name } => {
        write!(f, "Error: left recursion detected in rule '{}'", name)
      }
      ParserError::Unknown => write!(f, "Error: unknown"),
    }
  }
}

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
