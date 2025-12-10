#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
  /// Occurs when an expression fails to match the given rule with the given input.
  FailedToMatch {
    position: usize,
    input: &'a str,
    name: &'a str,
  },
  InvalidExpression {
    position: usize,
    name: &'a str,
  },
  /// Occurs when trying to consume when at the end of the input.
  EndOfInput {
    position: usize,
    input: Option<&'a str>,
  },
  /// Occurs when trying to access a rule that does not exist in the grammar.
  RuleNotFound {
    position: usize,
    name: &'a str,
  },
  /// Occurs when something unexpected happens.
  Unexpected {
    position: usize,
  },
  /// Catch all unknown error.
  Unknown,
}

impl ParserError<'_> {
  pub fn to_string(&self) -> String {
    match self {
      ParserError::FailedToMatch { position, input, name } => format!(
        "Error: failed to match rule '{}' at position {}: remaining input '{}'",
        name, position, input
      ),
      ParserError::InvalidExpression { position, name } => {
        format!("Error: invalid expression in rule '{}' at position {}", name, position)
      }
      ParserError::EndOfInput { position, input } => match input {
        Some(remaining) => format!(
          "Error: end of input at position {}: remaining input '{}'",
          position, remaining
        ),
        None => format!("Error: end of input at position {}", position),
      },
      ParserError::RuleNotFound { position, name } => {
        format!("Error: rule '{}' not found in grammar at position {}", name, position)
      }
      ParserError::Unexpected { position } => {
        format!("Error: unexpected at position {}", position)
      }
      ParserError::Unknown => "Error: unknown".to_string(),
    }
  }
}
