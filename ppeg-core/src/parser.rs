use std::{collections::HashMap, vec};

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
  /// Matches a named rule.
  /// Expression for `N`.
  NamedRule(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
  /// As this is a named rule, it needs a name!
  pub name: &'a str,
  /// Expression that applies to this rule.
  /// For example Expression::OneAndOne.
  pub expression: Expression<'a>,
}

impl<'a> Rule<'a> {
  pub fn new(name: &'a str, expression: Expression<'a>) -> Self {
    Rule { name, expression }
  }
}

#[derive(Clone, PartialEq)]
pub struct Grammar<'a> {
  /// Stores all of the rules of a grammar.
  /// Mapping of rule name to the rule data.
  rules: HashMap<&'a str, Rule<'a>>,
}

impl Grammar<'_> {
  pub fn new() -> Self {
    Grammar {
      rules: HashMap::new(),
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
    write!(f, "Grammars: {:?}", self.rules)
  }
}

impl<'a> Grammar<'a> {
  /// Inserts a new grammar into the grammars lookup table.
  /// Breaks down the grammar into its value and rule components.
  pub fn insert(&mut self, rule: Rule<'a>) {
    self.rules.insert(rule.name, rule);
  }

  /// Gets the the rule from the grammar lookup.
  pub fn get(&self, name: &'a str) -> Option<Rule<'a>> {
    self.rules.get(name).cloned()
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
  /// Grammars lookup table.
  grammar: Grammar<'a>,
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given input string.
  pub fn new(grammar: Grammar<'a>) -> Self {
    Parser { grammar }
  }

  /// Parses a given input using the named rule as the starting rule.
  /// Returns a result with the remaining input and a CST if parsing is successful.
  /// Otherwise, returns a ParserError.
  pub fn parse(
    &mut self,
    input: &mut &'a str,
    rule_name: &'a str,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let rule = self
      .grammar
      .get(rule_name)
      .ok_or(ParserError::RuleNotFound {
        position: 0,
        name: rule_name,
      })?;

    let (remaining, cst) = self.match_success(input, &rule.expression)?;
    if let Some(node) = cst {
      let mut root = CST::new(rule.name, vec![], None);
      root.add(Some(node));
      Ok((remaining, Some(root)))
    } else {
      Ok((remaining, None))
    }
  }

  /// Matches the given input against the provided expression.
  /// Returns the remaining input and CST if successful, otherwise returns a ParserError.
  pub fn match_success(
    &mut self,
    input: &mut &'a str,
    expression: &Expression<'a>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    match expression {
      Expression::Empty => Ok((input, None)),
      Expression::Char(char) => {
        if input.starts_with(char) {
          let remaining = &input[char.len()..];
          let node = CST::new(char, vec![], None);
          *input = remaining;
          Ok((remaining, Some(node)))
        } else {
          Err(ParserError::Unknown)
        }
      }
      Expression::Sequence(exprs) => {
        let mut cst = CST::new("sequence", vec![], None);

        for expr in exprs {
          let (remaining, node) = self.match_success(input, expr)?;
          *input = remaining;

          cst.add(node);
        }

        Ok((*input, Some(cst)))
      }
      Expression::Choice(exprs) => {
        for expr in exprs {
          match self.match_success(input, expr) {
            Ok((remaining, cst)) => {
              *input = remaining;

              match cst {
                Some(n) => return Ok((remaining, Some(CST::new("choice", vec![n], None)))),
                None => return Ok((remaining, None)),
              }
            }
            Err(_) => continue,
          }
        }
        Err(ParserError::Unknown)
      }
      Expression::ZeroOrMore(expr) => {
        let mut cst = CST::new("zero_or_more", vec![], None);
        let mut children: Vec<CST<'a>> = Vec::new();

        loop {
          let start_length = input.len();

          match self.match_success(input, expr) {
            Ok((remaining, cst)) => {
              if remaining.len() == start_length {
                break;
              }

              *input = remaining;

              if let Some(n) = cst {
                children.push(n);
              } else {
                break;
              }
            }
            Err(_) => break,
          }
        }

        for child in children {
          cst.add(Some(child));
        }

        match cst.is_leaf() {
          true => Ok((*input, None)),
          false => Ok((*input, Some(cst))),
        }
      }
      Expression::OneOrMore(expr) => {
        let start_length = input.len();
        let (remaining, cst) = self.match_success(input, &Expression::ZeroOrMore(expr.clone()))?;

        if remaining.len() == start_length {
          return Err(ParserError::FailedToMatch {
            position: 0,
            input,
            name: "one_or_more",
          });
        }

        let mut cst = cst.unwrap();
        cst.set("one_or_more");

        Ok((remaining, Some(cst)))
      }
      Expression::NamedRule(n) => {
        let rule = self.grammar.get(n);

        match rule {
          Some(r) => {
            let (remaining, cst_option) = self.match_success(input, &r.expression)?;
            *input = remaining;

            match cst_option {
              Some(child) => Ok((remaining, Some(CST::new(n, vec![child], None)))),
              None => Ok((remaining, Some(CST::new(n, vec![], None)))),
            }
          }
          None => Err(ParserError::RuleNotFound {
            position: 0,
            name: n,
          }),
        }
      }
    }
  }

  pub fn match_failure(&mut self) -> Result<CST<'a>, ParserError<'a>> {
    Err(ParserError::Unknown)
  }
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty input string.
  fn default() -> Self {
    Parser::new(Grammar::new())
  }
}

#[cfg(test)]
mod grammar_tests {
  use super::*;

  #[test]
  fn test_grammars_insert() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("e", Expression::Empty));

    assert_eq!(
      grammar.rules.get("e"),
      Some(&Rule::new("e", Expression::Empty))
    );
    assert_eq!(grammar.rules.get("b"), None);
  }

  #[test]
  fn test_grammars_get() {
    let mut grammar = Grammar::new();
    grammar.rules.insert("e", Rule::new("e", Expression::Empty));

    assert_eq!(grammar.get("e"), Some(Rule::new("e", Expression::Empty)));
    assert_eq!(grammar.get("b"), None);
  }
}

#[cfg(test)]
mod parser_tests {
  use super::*;

  #[test]
  fn test_parser_parse_char_success() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("rule", Expression::Char("a")));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "ab", "rule").unwrap();

      assert_eq!(remaining, "b");
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_char_fail() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", Expression::Char("a")));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "b", "a");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_sequence_success() {
    {
      let mut grammar = Grammar::new();
      grammar.insert(Rule::new(
        "rule",
        Expression::Sequence(vec![Expression::Char("a")]),
      ));

      let mut parser = Parser::new(grammar);
      let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          node.pretty_print(Some(0));
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "sequence");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut grammar = Grammar::new();
      grammar.insert(Rule::new(
        "rule",
        Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
      ));

      let mut parser = Parser::new(grammar);
      let (remaining, cst) = parser.parse(&mut "ab", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          node.pretty_print(Some(0));
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "sequence");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "b");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_sequence_fail() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::Sequence(vec![Expression::Char("a")]),
    ));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "b", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_choice_success() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::Choice(vec![Expression::Char("a"), Expression::Char("b")]),
    ));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "choice");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "b", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "choice");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "b");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "ba", "rule").unwrap();

      assert_eq!(remaining, "a");
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "choice");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "b");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_choice_fail() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::Choice(vec![Expression::Char("a"), Expression::Char("b")]),
    ));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "c", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_zero_or_more_success() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::ZeroOrMore(Box::new(Expression::Char("a"))),
    ));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "aaa", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "zero_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "aaab", "rule").unwrap();

      assert_eq!(remaining, "b");
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "zero_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_zero_or_more_fail() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::ZeroOrMore(Box::new(Expression::Char("a"))),
    ));

    let mut parser = Parser::new(grammar.clone());
    let result = parser.parse(&mut "b", "rule");

    // this doesn't fail as there is no requirement to match
    assert!(result.is_ok());
  }

  #[test]
  fn test_parser_parse_one_or_more_success() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::OneOrMore(Box::new(Expression::Char("a"))),
    ));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "aaa", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          // reuses zero_or_more logic for now
          assert_eq!(node.child(0).unwrap().get(), "one_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "aaab", "rule").unwrap();

      assert_eq!(remaining, "b");
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          // reuses zero_or_more logic for now
          assert_eq!(node.child(0).unwrap().get(), "one_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_one_or_more_fail() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new(
      "rule",
      Expression::OneOrMore(Box::new(Expression::Char("a"))),
    ));

    let mut parser = Parser::new(grammar.clone());
    let result = parser.parse(&mut "b", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_named_rule_success() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("rule", Expression::Char("a")));
    grammar.insert(Rule::new("named", Expression::NamedRule("rule")));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "a", "named").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "named");
          assert_eq!(node.child(0).unwrap().get(), "rule");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "aa", "named").unwrap();

      assert_eq!(remaining, "a");
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "named");
          assert_eq!(node.child(0).unwrap().get(), "rule");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }
}
