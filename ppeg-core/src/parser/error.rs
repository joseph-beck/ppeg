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
  /// Occurs when a feature is not implemented.
  NotImplemented { position: usize, name: &'a str },
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
      ParserError::NotImplemented { position, name } => {
        write!(f, "Error: feature '{}' not implemented at position {}", name, position)
      }
      ParserError::Unknown => write!(f, "Error: unknown"),
    }
  }
}
