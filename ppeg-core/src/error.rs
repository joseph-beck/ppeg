use crate::parser::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
  /// Occurs when an expression fails to match the given rule with the given input.
  FailedToMatch {
    position: usize,
    input: String,
    expression: Expression,
  },
  /// Occurs when trying to consume when at the end of the input.
  EndOfInput { position: usize, input: String },
}
