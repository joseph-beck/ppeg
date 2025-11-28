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
