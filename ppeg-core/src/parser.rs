use crate::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  /// Matches zero or more occurrences of the expression.
  /// Expression for `E*`.
  ZeroOrMore,
}

pub struct Rule {
  /// Name of the rule.
  pub name: String,
  /// Expression matching used to parse this rule.
  pub expression: Expression,
  /// String value used to match against this rule.
  pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
  /// Input string to be parsed.
  input: String,
  /// Current position in the input string.
  position: usize,
}

impl Parser {
  /// Creates a new instance of the parser with the given input string.
  pub fn new(input: String) -> Self {
    Parser { input, position: 0 }
  }

  /// Advances the current position by `n` characters.
  pub fn advance(&mut self, n: usize) {
    self.position += n;
  }

  /// Returns the substring of the input from the current position to the end.
  pub fn current(&self) -> String {
    self.input[self.position..].to_string()
  }

  pub fn zero_or_more(&self, rule: Rule) -> Result<(), ParserError> {
    match self.current() == rule.value {
      true => Ok(()),
      false => Err(ParserError::FailedToMatch {
        position: self.position,
        input: self.current(),
        expression: rule.expression,
      }),
    }
  }
}

impl Default for Parser {
  /// Creates a default parser with an empty input string.
  fn default() -> Self {
    Parser::new(String::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parser_new() {
    let parser = Parser::new(String::new());
    assert_eq!(
      parser,
      Parser {
        input: String::new(),
        position: 0
      }
    );
  }

  #[test]
  fn test_parser_default() {
    let parser = Parser::default();
    assert_eq!(parser, Parser::new(String::new()));
  }

  #[test]
  fn test_parser_advance() {
    let mut parser = Parser::new("abc".to_string());
    parser.advance(2);
    assert_eq!(parser.position, 2);
  }

  #[test]
  fn test_parser_current() {
    let parser = Parser::new("abc".to_string());
    assert_eq!(parser.current(), "abc".to_string());
  }

  #[test]
  fn test_parser_zero_or_more_match() {
    let parser = Parser::new("a".to_string());
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::ZeroOrMore,
      value: "a".to_string(),
    };
    let result = parser.zero_or_more(rule);
    assert_eq!(result, Ok(()));
  }

  #[test]
  fn test_parser_zero_or_more_no_match() {
    let parser = Parser::new("a".to_string());
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::ZeroOrMore,
      value: "b".to_string(),
    };
    let result = parser.zero_or_more(rule);
    assert_eq!(
      result,
      Err(ParserError::FailedToMatch {
        position: 0,
        input: "a".to_string(),
        expression: Expression::ZeroOrMore,
      })
    );
  }
}
