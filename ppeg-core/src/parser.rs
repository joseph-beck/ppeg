use std::collections::HashMap;

use crate::{cst::CST, error::ParserError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
  /// Matches what is an "empty" expression.
  /// This means that it matches with what it is given,
  /// consumes the output but gives no output.
  /// Expression for `ε` symbol.
  Empty,
  /// Matches a single character.
  /// For example, `A` matches the character A.
  /// Expression for `C`.
  Char(&'a str),
  /// Matches a sequence of the given expressions.
  /// For example, `AB` matches A followed by B.
  /// Expression for `EE'`.
  Sequence(Vec<Expression<'a>>),
  /// Matches a or b occurrences of an expression.
  /// For example, `A|B` matches either A or B.
  /// Expression for `E|E'`. ?
  Choice(Vec<Expression<'a>>),
  /// Matches one or more occurrences of the expression.
  /// For example, `A` matches one or more occurrences of A.
  /// Expression for `EE*`.
  OneOrMore(Box<Expression<'a>>),
  /// Matches zero or more occurrences of the expression.
  /// For example, `A` matches zero or more occurrences of A.
  /// Expression for `E*`.
  ZeroOrMore(Box<Expression<'a>>),
  /// Matches when an error has occurred.
  Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
  /// As this is a named rule, it needs a name!
  pub name: &'a str,
  /// How the grammar is matched and represented.
  /// For example "A".
  pub values: Vec<&'a str>,
  /// Expression that applies to this rule.
  /// For example Expression::OneAndOne.
  pub expression: Expression<'a>,
}

impl<'a> Rule<'a> {
  pub fn new(name: &'a str, values: Vec<&'a str>, expression: Expression<'a>) -> Self {
    Rule {
      name,
      values,
      expression,
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct Grammar<'a> {
  /// Mapping of grammar values to their corresponding rules.
  grammars: HashMap<&'a str, Rule<'a>>,
}

impl Grammar<'_> {
  pub fn new() -> Self {
    Grammar {
      grammars: HashMap::new(),
    }
  }
}

impl Default for Grammar<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl std::fmt::Debug for Grammar<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Grammars: {:?}", self.grammars)
  }
}

impl<'a> Grammar<'a> {
  /// Inserts a new grammar into the grammars lookup table.
  /// Breaks down the grammar into its value and rule components.
  pub fn insert(&mut self, rule: Rule<'a>) {
    self.grammars.insert(rule.name, rule);
  }

  /// Gets the the rule from the grammar lookup.
  pub fn get(&self, value: &'a str) -> Option<Rule<'a>> {
    self
      .grammars
      .values()
      .find(|rule| rule.values.contains(&value))
      .cloned()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Output<'a> {
  /// Output result of parsing.
  /// Ok if successful, Err with ParserError if failed.
  pub result: Result<Vec<&'a str>, ParserError<'a>>,
  /// Rule that was applied to produce this output.
  pub expression: Expression<'a>,
}

impl<'a> Output<'a> {
  pub fn new(result: Result<Vec<&'a str>, ParserError<'a>>, expression: Expression<'a>) -> Self {
    Output { result, expression }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Input string to be parsed.
  input: &'a str,
  /// Current position in the input string.
  position: usize,
  /// Grammars lookup table.
  grammar: Grammar<'a>,
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given input string.
  pub fn new(input: &'a str, position: usize, grammar: Grammar<'a>) -> Self {
    Parser {
      input,
      position,
      grammar,
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

  pub fn parse(&mut self, input: &'a str, rule_name: &'a str) -> Result<CST<'a>, ParserError<'a>> {
    let rule = self
      .grammar
      .get(rule_name)
      .ok_or(ParserError::RuleNotFound {
        position: 0,
        name: rule_name,
      })?;

    self.input = input;
    self.position = 0;

    match self.match_success(&rule.expression) {
      Ok(cst) => Ok(cst),
      Err(_) => self.match_failure(),
    }
  }

  pub fn match_success(&mut self, expression: &Expression) -> Result<CST<'a>, ParserError<'a>> {
    match expression {
      Expression::Empty => Ok(CST::default()),
      Expression::Sequence(e) => {
        let mut cst = CST::default();

        for expr in e {
          let res = self.match_success(expr)?;
          cst.add(res);
        }

        Ok(cst)
      }
      Expression::Choice(_e) => Err(ParserError::Unknown),
      _ => Err(ParserError::Unknown),
    }
  }

  pub fn match_failure(&mut self) -> Result<CST<'a>, ParserError<'a>> {
    Err(ParserError::Unknown)
  }
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty input string.
  fn default() -> Self {
    Parser::new("", 0, Grammar::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_grammars_insert() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("e", vec!["e"], Expression::Empty));

    assert_eq!(
      grammar.grammars.get("e"),
      Some(&Rule::new("e", vec!["e"], Expression::Empty))
    );
    assert_eq!(grammar.grammars.get("b"), None);
  }

  #[test]
  fn test_grammars_get() {
    let mut grammar = Grammar::new();
    grammar
      .grammars
      .insert("e", Rule::new("e", vec!["e"], Expression::Empty));

    assert_eq!(
      grammar.get("e"),
      Some(Rule::new("e", vec!["e"], Expression::Empty))
    );
    assert_eq!(grammar.get("b"), None);
  }
}
