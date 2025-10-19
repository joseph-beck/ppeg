use crate::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  /// Matches one and another occurrences of an expression.
  /// Expression for `EE'`. ?
  OneAndOne,
  /// Matches one and another occurrences of an expression.
  /// Expression for `E|E'`. ?
  OneOrOne,
  /// Matches zero or more occurrences of the expression.
  /// Expression for `E*`.
  ZeroOrMore,
}

pub struct Rule<'a> {
  /// Name of the rule.
  pub name: String,
  /// Expression matching used to parse this rule.
  pub expression: Expression,
  /// String value used to match against this rule.
  pub value: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Input string to be parsed.
  input: &'a str,
  /// Current position in the input string.
  position: usize,
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given input string.
  pub fn new(input: &'a str) -> Self {
    Parser { input, position: 0 }
  }

  /// Advances the current position by `n` characters.
  pub fn advance(&mut self, n: usize) {
    self.position += n;
  }

  /// Returns the substring of the input from the current position to the end.
  pub fn current(&self) -> &'a str {
    &self.input[self.position..]
  }

  pub fn one_and_one(&self, _rule_one: Rule, _rule_two: Rule) -> Result<(), ParserError<'a>> {
    Ok(())
  }

  pub fn one_or_one(&self, rule_one: Rule, rule_two: Rule) -> Result<(), ParserError<'a>> {
    match self.current() == rule_one.value {
      true => Ok(()),
      false => match self.current() == rule_two.value {
        true => Ok(()),
        false => Err(ParserError::FailedToMatch {
          position: self.position,
          input: self.current(),
          expression: rule_two.expression,
        }),
      },
    }
  }

  pub fn zero_or_more(&self, rule: Rule) -> Result<(), ParserError<'a>> {
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

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty input string.
  fn default() -> Self {
    Parser::new("")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parser_new() {
    let parser = Parser::new("");
    assert_eq!(
      parser,
      Parser {
        input: "",
        position: 0
      }
    );
  }

  #[test]
  fn test_parser_default() {
    let parser = Parser::default();
    assert_eq!(parser, Parser::new(""));
  }

  #[test]
  fn test_parser_advance() {
    let mut parser = Parser::new("abc");
    parser.advance(2);
    assert_eq!(parser.position, 2);
  }

  #[test]
  fn test_parser_current() {
    let parser = Parser::new("abc");
    assert_eq!(parser.current(), "abc");
  }

  #[test]
  fn test_parser_one_or_one_match_rule_one() {
    let parser = Parser::new("a");
    let rule_one = Rule {
      name: "rule_one".to_string(),
      expression: Expression::OneOrOne,
      value: "a",
    };
    let rule_two = Rule {
      name: "rule_two".to_string(),
      expression: Expression::OneOrOne,
      value: "b",
    };

    let result = parser.one_or_one(rule_one, rule_two);
    assert_eq!(result, Ok(()));
  }

  #[test]
  fn test_parser_one_or_one_match_rule_two() {
    let parser = Parser::new("b");
    let rule_one = Rule {
      name: "rule_one".to_string(),
      expression: Expression::OneOrOne,
      value: "a",
    };
    let rule_two = Rule {
      name: "rule_two".to_string(),
      expression: Expression::OneOrOne,
      value: "b",
    };

    let result = parser.one_or_one(rule_one, rule_two);
    assert_eq!(result, Ok(()));
  }

  #[test]
  fn test_parser_one_or_one_no_match() {
    let parser = Parser::new("c");
    let rule_one = Rule {
      name: "rule_one".to_string(),
      expression: Expression::OneOrOne,
      value: "a",
    };
    let rule_two = Rule {
      name: "rule_two".to_string(),
      expression: Expression::OneOrOne,
      value: "b",
    };

    let result = parser.one_or_one(rule_one, rule_two);
    assert_eq!(
      result,
      Err(ParserError::FailedToMatch {
        position: 0,
        input: "c",
        expression: Expression::OneOrOne,
      })
    );
  }

  #[test]
  fn test_parser_zero_or_more_match() {
    let parser = Parser::new("a");
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::ZeroOrMore,
      value: "a",
    };

    let result = parser.zero_or_more(rule);
    assert_eq!(result, Ok(()));
  }

  #[test]
  fn test_parser_zero_or_more_no_match() {
    let parser = Parser::new("a");
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::ZeroOrMore,
      value: "b",
    };

    let result = parser.zero_or_more(rule);
    assert_eq!(
      result,
      Err(ParserError::FailedToMatch {
        position: 0,
        input: "a",
        expression: Expression::ZeroOrMore,
      })
    );
  }
}
