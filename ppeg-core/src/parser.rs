use std::{collections::HashMap, vec};

use crate::{cst::CST, error::ParserError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  /// Matches what is an "empty" expression.
  /// This means that it matches with what it is given,
  /// consumes the output but gives no output.
  /// Expression for `ε` symbol.
  Empty,
  /// Matches a sequence of the given expressions.
  /// For example, `AB` matches A followed by B.
  /// Expression for `EE'`. ?
  Sequence,
  /// Matches a or b occurrences of an expression.
  /// For example, `A|B` matches either A or B.
  /// Expression for `E|E'`. ?
  Choice,
  /// Matches one or more occurrences of the expression.
  /// For example, `A` matches one or more occurrences of A.
  /// Expression for `EE*`.
  OneOrMore,
  /// Matches zero or more occurrences of the expression.
  /// For example, `A` matches zero or more occurrences of A.
  /// Expression for `E*`.
  ZeroOrMore,
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
  pub expression: Expression,
}

impl<'a> Rule<'a> {
  pub fn new(name: &'a str, values: Vec<&'a str>, expression: Expression) -> Self {
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
  pub expression: Expression,
}

impl<'a> Output<'a> {
  pub fn new(result: Result<Vec<&'a str>, ParserError<'a>>, expression: Expression) -> Self {
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

  pub fn parse(&mut self) -> Result<CST<'a>, ParserError<'a>> {
    let outputs = self.judge()?;
    println!("Outputs: {:?}", outputs);

    if outputs.is_empty() {
      return Err(ParserError::EndOfInput {
        position: self.position,
        input: None,
      });
    }

    let mut cst = CST::default();

    match self.create_cst(&mut cst, outputs) {
      Ok(_) => Ok(cst),
      Err(e) => Err(e),
    }
  }

  pub fn create_cst(
    &mut self,
    cst: &mut CST<'a>,
    rest: Vec<Output<'a>>,
  ) -> Result<(), ParserError<'a>> {
    if rest.is_empty() {
      return Ok(());
    }

    let current = rest.first().unwrap();

    match current.expression {
      // If we have an empty expression, lets just skip it and keep creating CST nodes.
      Expression::Empty => self.create_cst(cst, rest[1..].to_vec()),
      // This is a choice.
      Expression::Choice | Expression::Sequence => match current.result.clone() {
        Ok(values) => {
          if values.len() != 1 {
            return Err(ParserError::InvalidExpression {
              position: self.position,
              rule: Expression::Choice,
            });
          }

          let child = CST::new(values.first().unwrap(), vec![], None);
          cst.add(child);
          self
            .create_cst(cst.child(0).unwrap(), rest[1..].to_vec())
            .unwrap();
          Ok(())
        }
        Err(e) => Err(e),
      },
      // This is a value, like 1.
      Expression::ZeroOrMore | Expression::OneOrMore => match current.result.clone() {
        Ok(values) => {
          for value in values {
            let child = CST::new(value, Vec::new(), None);
            cst.add(child);
          }

          self
            .create_cst(cst.child(0).unwrap(), rest[1..].to_vec())
            .unwrap();
          Ok(())
        }
        Err(e) => Err(e),
      },
      _ => Err(ParserError::Unknown),
    }
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
      let grammar = self.grammar.get(self.current());

      match grammar {
        Some(g) => match g.expression {
          Expression::Empty => {
            outputs.push(Output::new(Ok(vec![self.current()]), Expression::Empty));
            // When matching an empty CST node should be created with a label with hidden as true.
            self.advance(1);
          }
          Expression::Sequence => {
            outputs.push(Output::new(self.sequence(g), Expression::Sequence));
          }
          Expression::Choice => {
            outputs.push(Output::new(self.choice(g), Expression::Choice));
          }
          Expression::OneOrMore => {
            outputs.push(Output::new(self.one_or_more(g), Expression::OneOrMore));
          }
          Expression::ZeroOrMore => {
            outputs.push(Output::new(self.zero_or_more(g), Expression::ZeroOrMore));
          }
          _ => {
            // advance by 1 for now should handle this case.
            // to be implemented later.
            outputs.push(Output::new(
              Err(ParserError::InvalidExpression {
                position: self.position,
                rule: Expression::Error,
              }),
              Expression::Error,
            ));
            self.advance(1);
          }
        },
        None => {
          outputs.push(Output::new(
            Err(ParserError::FailedToMatch {
              position: self.position,
              input: self.current(),
              rule: Expression::Error,
            }),
            Expression::Error,
          ));
          self.advance(1);
        }
      }
    }

    Ok(outputs)
  }

  pub fn sequence(&mut self, rule: Rule<'a>) -> Result<Vec<&'a str>, ParserError<'a>> {
    let mut results = Vec::new();

    for rule_value in &rule.values {
      if self.current() != *rule_value {
        return Err(ParserError::FailedToMatch {
          position: self.position,
          input: self.current(),
          rule: rule.expression,
        });
      }

      results.push(self.current());
      self.advance(1);
    }

    Ok(results)
  }

  pub fn choice(&mut self, rule: Rule<'a>) -> Result<Vec<&'a str>, ParserError<'a>> {
    let mut results = Vec::new();

    for rule_value in &rule.values {
      if self.current() == *rule_value {
        results.push(self.current());
        self.advance(1);

        return Ok(results);
      }
    }

    Err(ParserError::FailedToMatch {
      position: self.position,
      input: self.current(),
      rule: rule.expression,
    })
  }

  pub fn one_or_more(&mut self, rule: Rule<'a>) -> Result<Vec<&'a str>, ParserError<'a>> {
    match self.zero_or_more(rule) {
      Ok(results) => match results.is_empty() {
        true => Err(ParserError::FailedToMatch {
          position: self.position,
          input: self.current(),
          rule: Expression::OneOrMore,
        }),
        false => Ok(results),
      },
      Err(e) => Err(e),
    }
  }

  pub fn zero_or_more(&mut self, rule: Rule<'a>) -> Result<Vec<&'a str>, ParserError<'a>> {
    let mut results = Vec::new();

    for rule_value in &rule.values {
      while self.current() == *rule_value {
        results.push(self.current());
        self.advance(1);

        if self.is_empty() {
          break;
        }
      }
    }

    Ok(results)
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

  #[test]
  fn test_parser_new() {
    let parser = Parser::new("", 0, Grammar::new());

    assert_eq!(
      parser,
      Parser {
        input: "",
        position: 0,
        grammar: Grammar::new(),
      }
    );
  }

  #[test]
  fn test_parser_default() {
    let parser = Parser::default();

    assert_eq!(parser, Parser::new("", 0, Grammar::new()));
  }

  #[test]
  fn test_parser_advance() {
    let mut parser = Parser::new("abc", 0, Grammar::new());
    parser.advance(2);

    assert_eq!(parser.position, 2);
  }

  #[test]
  fn test_parser_current() {
    let parser = Parser::new("abc", 0, Grammar::new());

    assert_eq!(parser.current(), "a");
  }

  #[test]
  fn test_parser_length() {
    let parser = Parser::new("abc", 0, Grammar::new());

    assert_eq!(parser.length(), 3);
  }

  #[test]
  fn test_parser_empty_not_empty() {
    let parser = Parser::new("ab", 0, Grammar::new());

    assert!(!parser.is_empty());
  }

  #[test]
  fn test_parser_empty_empty_no_input() {
    let parser = Parser::new("", 0, Grammar::new());

    assert!(parser.is_empty());
  }

  #[test]
  fn test_parser_empty_empty_input() {
    let mut parser = Parser::new("a", 0, Grammar::new());

    assert!(!parser.is_empty());

    parser.advance(1);

    assert!(parser.is_empty());
  }

  #[test]
  fn test_parser_judge_empty_input() {
    let mut parser = Parser::new("", 0, Grammar::new());
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
  fn test_parser_judge_sequence_a_ok() {
    let mut grammars = Grammar::new();
    grammars.insert(Rule::new("a", vec!["a"], Expression::Sequence));

    let mut parser = Parser::new("a", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![Output::new(Ok(vec!["a"]), Expression::Sequence),])
    );
  }

  #[test]
  fn test_parser_judge_sequence_a_err() {
    let mut grammars = Grammar::new();
    grammars.insert(Rule::new("a", vec!["a"], Expression::Sequence));

    let mut parser = Parser::new("b", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![Output::new(
        Err(ParserError::FailedToMatch {
          position: 0,
          input: "b",
          rule: Expression::Error
        }),
        Expression::Error
      ),])
    );
  }

  #[test]
  fn test_parser_judge_sequence_ab_ok() {
    let mut grammars = Grammar::new();
    grammars.insert(Rule::new("a", vec!["a", "b"], Expression::Sequence));

    let mut parser = Parser::new("ab", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![Output::new(Ok(vec!["a", "b"]), Expression::Sequence),])
    );
  }

  #[test]
  fn test_parser_judge_sequence_ab_err() {
    let mut grammars = Grammar::new();
    grammars.insert(Rule::new("a", vec!["a", "b"], Expression::Sequence));

    let mut parser = Parser::new("ac", 0, grammars);

    let result = parser.judge();
    assert_eq!(
      result,
      Ok(vec![
        Output::new(
          // different failure as we failing in the sequence parsing segment.
          Err(ParserError::FailedToMatch {
            position: 1,
            input: "c",
            rule: Expression::Sequence
          }),
          Expression::Sequence
        ),
        // for now two errors are correct as it attempts to reparse the unconsumed remainder of the input.
        Output::new(
          Err(ParserError::FailedToMatch {
            position: 1,
            input: "c",
            rule: Expression::Error
          }),
          Expression::Error
        )
      ])
    );
  }

  #[test]
  fn test_parser_judge_choice_match_one() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::Choice));

    let mut parser = Parser::new("a", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(Ok(vec!["a"]), Expression::Choice)])
    );
  }

  #[test]
  fn test_parser_judge_choice_match_two() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a or b", vec!["a", "b"], Expression::Choice));

    let mut parser = Parser::new("b", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(Ok(vec!["b"]), Expression::Choice)])
    );
  }

  #[test]
  fn test_parser_judge_choice_no_match_one() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::Choice));

    let mut parser = Parser::new("b", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(
        Err(ParserError::FailedToMatch {
          position: 0,
          input: "b",
          rule: Expression::Error
        }),
        Expression::Error
      )])
    );
  }

  #[test]
  fn test_parser_judge_choice_no_match_two() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a or b", vec!["a", "b"], Expression::Choice));

    let mut parser = Parser::new("c", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(
        Err(ParserError::FailedToMatch {
          position: 0,
          input: "c",
          rule: Expression::Error
        }),
        Expression::Error
      )])
    );
  }

  #[test]
  fn test_parser_judge_zero_or_more_one() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::ZeroOrMore));

    let mut parser = Parser::new("aaa", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(
        Ok(vec!["a", "a", "a"]),
        Expression::ZeroOrMore
      ),])
    );
  }

  #[test]
  fn test_parser_judge_zero_or_more_two() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::ZeroOrMore));
    grammar.insert(Rule::new("b", vec!["b"], Expression::Empty));

    let mut parser = Parser::new("aab", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a", "a",]), Expression::ZeroOrMore),
        Output::new(Ok(vec!["b"]), Expression::Empty)
      ])
    );
  }

  #[test]
  fn test_parser_judge_one_or_more() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::OneOrMore));
    grammar.insert(Rule::new("b", vec!["b"], Expression::Empty));

    let mut parser = Parser::new("aab", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a", "a",]), Expression::OneOrMore),
        Output::new(Ok(vec!["b"]), Expression::Empty)
      ])
    );
  }

  #[test]
  fn test_parser_judge_one_or_more_many() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a and b", vec!["a", "b"], Expression::OneOrMore));

    let mut parser = Parser::new("aab", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(
        Ok(vec!["a", "a", "b"]),
        Expression::OneOrMore
      ),])
    );
  }

  #[test]
  fn test_parser_judge_mixed() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::ZeroOrMore));
    grammar.insert(Rule::new("b", vec!["b"], Expression::Sequence));

    let mut parser = Parser::new("aab", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![
        Output::new(Ok(vec!["a", "a"]), Expression::ZeroOrMore),
        Output::new(Ok(vec!["b"]), Expression::Sequence)
      ])
    );
  }

  #[test]
  fn test_parser_judge_failed_to_match() {
    let mut grammar = Grammar::new();
    grammar.insert(Rule::new("a", vec!["a"], Expression::Sequence));

    let mut parser = Parser::new("b", 0, grammar);
    let result = parser.judge();

    assert_eq!(
      result,
      Ok(vec![Output::new(
        Err(ParserError::FailedToMatch {
          position: 0,
          input: "b",
          rule: Expression::Error,
        }),
        Expression::Error
      )])
    );
  }

  #[test]
  fn test_parser_one_or_more_ok() {
    let grammar = Grammar::new();
    let one_or_more_rule = Rule::new("a", vec!["a"], Expression::OneOrMore);

    let mut parser = Parser::new("a", 0, grammar);
    let result = parser.one_or_more(one_or_more_rule);

    assert_eq!(result, Ok(vec!["a"]));
  }

  #[test]
  fn test_parser_one_or_more_err() {
    let grammar = Grammar::new();
    let one_or_more_rule = Rule::new("a", vec!["a"], Expression::OneOrMore);

    let mut parser = Parser::new(" ", 0, grammar);
    let result = parser.one_or_more(one_or_more_rule);

    assert_eq!(
      result,
      Err(ParserError::FailedToMatch {
        position: 0,
        input: " ",
        rule: Expression::OneOrMore,
      })
    );
  }

  #[test]
  fn test_parser_zero_or_more_ok() {
    let grammar = Grammar::new();
    let zero_or_more_rule = Rule::new("a", vec!["a"], Expression::ZeroOrMore);
    let mut parser = Parser::new("a", 0, grammar.clone());

    {
      // one match
      let result = parser.zero_or_more(zero_or_more_rule.clone());

      assert_eq!(result, Ok(vec!["a"]));
    }

    {
      // two match
      parser = Parser::new("aa", 0, grammar.clone());
      let result = parser.zero_or_more(zero_or_more_rule.clone());

      assert_eq!(result, Ok(vec!["a", "a"]));
    }

    {
      // no matches
      parser = Parser::new(" ", 0, grammar);
      let result = parser.zero_or_more(zero_or_more_rule);

      assert_eq!(result, Ok(vec![]));
    }
  }
}
