use crate::parser::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
  /// Occurs when an expression fails to match the given rule with the given input.
  FailedToMatch {
    position: usize,
    input: &'a str,
    expression: Expression,
  },
  /// Occurs when trying to consume when at the end of the input.
  EndOfInput { position: usize, input: &'a str },
}
