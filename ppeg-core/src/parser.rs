use std::collections::HashMap;

use crate::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
  /// Matches what is an empty expression.
  /// Expression for `ε` symbol.
  Empty,
  /// Matches one and another occurrences of an expression.
  /// Expression for `EE'`. ?
  OneAndOne,
  /// Matches one and another occurrences of an expression.
  /// Expression for `E|E'`. ?
  OneOrOne,
  /// Matches zero or more occurrences of the expression.
  /// Expression for `E*`.
  ZeroOrMore,
  /// Matches when an error has occurred.
  Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grammar<'a> {
  /// How the grammar is matched and represented.
  /// For example "A".
  value: &'a str,
  /// Rule that applies to this grammar.
  /// For example Rule::OneAndOne.
  rule: Rule,
}

impl<'a> Grammar<'a> {
  pub fn new(value: &'a str, rule: Rule) -> Self {
    Grammar { value, rule }
  }
}

#[derive(Clone, PartialEq)]
pub struct Grammars<'a> {
  /// Mapping of grammar values to their corresponding rules.
  grammars: HashMap<&'a str, Rule>,
}

impl Grammars<'_> {
  pub fn new() -> Self {
    Grammars {
      grammars: HashMap::new(),
    }
  }
}

impl Default for Grammars<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl std::fmt::Debug for Grammars<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Grammars: {:?}", self.grammars)
  }
}

impl<'a> Grammars<'a> {
  /// Inserts a new grammar into the grammars lookup table.
  /// Breaks down the grammar into its value and rule components.
  pub fn insert(&mut self, grammar: Grammar<'a>) {
    self.grammars.insert(grammar.value, grammar.rule);
  }

  /// Gets the rule associated with the given grammar value.
  pub fn get(&self, value: &str) -> Option<Rule> {
    self.grammars.get(value).cloned()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Output<'a> {
  /// Output result of parsing.
  /// Ok if successful, Err with ParserError if failed.
  pub result: Result<Vec<&'a str>, ParserError<'a>>,
  /// Rule that was applied to produce this output.
  pub rule: Rule,
}

impl<'a> Output<'a> {
  pub fn new(result: Result<Vec<&'a str>, ParserError<'a>>, rule: Rule) -> Self {
    Output { result, rule }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Input string to be parsed.
  input: &'a str,
  /// Current position in the input string.
  position: usize,
  /// Grammars lookup table.
  grammars: Grammars<'a>,
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given input string.
  pub fn new(input: &'a str, position: usize, grammars: Grammars<'a>) -> Self {
    Parser {
      input,
      position,
      grammars,
    }
  }

  /// Advances the current position by `n` characters.
  pub fn advance(&mut self, n: usize) {
    self.position += n;
  }

  /// Returns the substring of the input from the current position to the end.
  pub fn current(&self) -> &'a str {
    &self.input[self.position..self.position + 1]
  }

  /// Returns the total length of the input string.
  pub fn length(&self) -> usize {
    self.input.len()
  }

  /// Returns whether the input string is empty.
  pub fn is_empty(&self) -> bool {
    self.input.is_empty() || self.position >= self.length()
  }

  /// Judges, evaluates, the input string against the defined grammars and produces a vec of outputs.
  pub fn judge(&mut self) -> Result<Vec<Output<'a>>, ParserError<'a>> {
    if self.is_empty() {
      return Err(ParserError::EndOfInput {
        position: self.position,
        input: None,
      });
    }

    let mut outputs = Vec::new();

    while !self.is_empty() {
      let rule = self.grammars.get(self.current());

      match rule {
        Some(Rule::Empty) => {
          outputs.push(Output::new(Ok(vec![self.current()]), Rule::Empty));
          self.advance(1);
        }
        Some(Rule::OneAndOne) => {
          outputs.push(Output::new(self.one_and_one(), Rule::OneAndOne));
        }
        Some(Rule::OneOrOne) => {
          outputs.push(Output::new(self.one_or_one(), Rule::OneOrOne));
        }
        Some(Rule::ZeroOrMore) => {
          outputs.push(Output::new(self.zero_or_more(), Rule::ZeroOrMore));
        }
        _ => {
          outputs.push(Output::new(
            Err(ParserError::FailedToMatch {
              position: self.position,
              input: self.current(),
              rule: Rule::Error,
            }),
            Rule::Error,
          ));
          self.advance(1);
        }
      }
    }

    Ok(outputs)
  }

  pub fn one_and_one(&mut self) -> Result<Vec<&'a str>, ParserError<'a>> {
    // here we can do some logic later.
    let current = self.current();
    self.advance(1);
    Ok(vec![current])
  }

  pub fn one_or_one(&mut self) -> Result<Vec<&'a str>, ParserError<'a>> {
    // here we can do some logic later.
    let current = self.current();
    self.advance(1);
    Ok(vec![current])
  }

  pub fn zero_or_more(&mut self) -> Result<Vec<&'a str>, ParserError<'a>> {
    // here we can do some logic later.
    let current = self.current();
    self.advance(1);
    Ok(vec![current])
  }
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty input string.
  fn default() -> Self {
    Parser::new("", 0, Grammars::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_grammars_insert() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("e", Rule::Empty));
    assert_eq!(grammars.grammars.get("e"), Some(&Rule::Empty));
    assert_eq!(grammars.grammars.get("b"), None);
  }

  #[test]
  fn test_grammars_get() {
    let mut grammars = Grammars::new();
    grammars.grammars.insert("e", Rule::Empty);
    assert_eq!(grammars.get("e"), Some(Rule::Empty));
    assert_eq!(grammars.get("b"), None);
  }

  #[test]
  fn test_parser_new() {
    let parser = Parser::new("", 0, Grammars::new());
    assert_eq!(
      parser,
      Parser {
        input: "",
        position: 0,
        grammars: Grammars::new(),
      }
    );
  }

  #[test]
  fn test_parser_default() {
    let parser = Parser::default();
    assert_eq!(parser, Parser::new("", 0, Grammars::new()));
  }

  #[test]
  fn test_parser_advance() {
    let mut parser = Parser::new("abc", 0, Grammars::new());
    parser.advance(2);
    assert_eq!(parser.position, 2);
  }

  #[test]
  fn test_parser_current() {
    let parser = Parser::new("abc", 0, Grammars::new());
    assert_eq!(parser.current(), "a");
  }

  #[test]
  fn test_parser_length() {
    let parser = Parser::new("abc", 0, Grammars::new());
    assert_eq!(parser.length(), 3);
  }

  #[test]
  fn test_parser_empty_not_empty() {
    let parser = Parser::new("ab", 0, Grammars::new());
    assert!(!parser.is_empty());
  }

  #[test]
  fn test_parser_empty_empty_no_input() {
    let parser = Parser::new("", 0, Grammars::new());
    assert!(parser.is_empty());
  }

  #[test]
  fn test_parser_empty_empty_input() {
    let mut parser = Parser::new("a", 0, Grammars::new());
    assert!(!parser.is_empty());
    parser.advance(1);
    assert!(parser.is_empty());
  }

  #[test]
  fn test_parser_judge_empty_input() {
    let mut parser = Parser::new("", 0, Grammars::new());
    let result = parser.judge();
    assert_eq!(
      result,
      Err(ParserError::EndOfInput {
        position: 0,
        input: None
      })
    );
  }

  #[test]
  fn test_parser_judge_one_and_one() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("a", Rule::OneAndOne));

    let mut parser = Parser::new("aa", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a"]), Rule::OneAndOne),
        Output::new(Ok(vec!["a"]), Rule::OneAndOne)
      ])
    );
  }

  #[test]
  fn test_parser_judge_one_or_one() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("a", Rule::OneOrOne));

    let mut parser = Parser::new("a", 0, grammars);

    let result = parser.judge();
    assert_eq!(result, Ok(vec![Output::new(Ok(vec!["a"]), Rule::OneOrOne)]));
  }

  #[test]
  fn test_parser_judge_zero_or_more() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("a", Rule::ZeroOrMore));

    let mut parser = Parser::new("aaa", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a"]), Rule::ZeroOrMore),
        Output::new(Ok(vec!["a"]), Rule::ZeroOrMore),
        Output::new(Ok(vec!["a"]), Rule::ZeroOrMore)
      ])
    );
  }

  #[test]
  fn test_parser_judge_mixed() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("a", Rule::ZeroOrMore));
    grammars.insert(Grammar::new("b", Rule::OneAndOne));

    let mut parser = Parser::new("aab", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a"]), Rule::ZeroOrMore),
        Output::new(Ok(vec!["a"]), Rule::ZeroOrMore),
        Output::new(Ok(vec!["b"]), Rule::OneAndOne)
      ])
    );
  }

  #[test]
  fn test_parser_judge_failed_to_match() {
    let mut grammars = Grammars::new();
    grammars.insert(Grammar::new("a", Rule::OneAndOne));

    let mut parser = Parser::new("b", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![Output::new(
        Err(ParserError::FailedToMatch {
          position: 0,
          input: "b",
          rule: Rule::Error,
        }),
        Rule::Error
      )])
    );
  }
}
