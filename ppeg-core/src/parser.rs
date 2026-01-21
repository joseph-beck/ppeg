use std::{collections::HashMap, vec};

use crate::{
  cst::CST,
  error::ParserError,
  packrat::{Packrat, State},
};

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

impl<'a> Grammar<'a> {
  /// Creates a new instance of grammar with the given rules.
  pub fn new(rules: HashMap<&'a str, Rule<'a>>) -> Self {
    Grammar { rules }
  }
}

impl Default for Grammar<'_> {
  /// Creates a default empty grammar.
  fn default() -> Self {
    Self::new(HashMap::new())
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

  /// Inserts a new rule into the grammar and returns an updated instance of the grammar.
  pub fn with(mut self, rule: Rule<'a>) -> Self {
    self.insert(rule);

    self
  }

  /// Gets the the rule from the grammar lookup.
  /// If the rule does not exist, returns None.
  pub fn get(&self, name: &'a str) -> Option<Rule<'a>> {
    self.rules.get(name).cloned()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Grammars lookup table.
  grammar: Grammar<'a>,
  /// Packrat memoization table.
  packrat: Packrat<'a>,
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given grammar.
  pub fn new(grammar: Grammar<'a>) -> Self {
    Parser {
      grammar,
      packrat: Packrat::new(),
    }
  }

  /// Parses a given input using the named rule as the starting rule.
  /// Returns a result with the remaining input and a CST if parsing is successful.
  /// Otherwise, returns a ParserError.
  pub fn parse(
    &mut self,
    input: &mut &'a str,
    rule_name: &'a str,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let rule = self.grammar.get(rule_name).ok_or(ParserError::RuleNotFound {
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
  fn match_success(
    &mut self,
    input: &mut &'a str,
    expression: &Expression<'a>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    match expression {
      Expression::Empty => Ok((input, None)),
      Expression::Char(char) => self.char(input, char),
      Expression::Sequence(exprs) => self.sequence(input, exprs),
      Expression::Choice(exprs) => self.choice(input, exprs),
      Expression::ZeroOrMore(expr) => self.zero_or_more(input, expr),
      Expression::OneOrMore(expr) => self.one_or_more(input, expr),
      Expression::NamedRule(n) => self.named_rule(input, n),
    }
  }

  /// Parses a character from the input and advances the input when successful.
  /// If the character does not match, returns a `ParserError::Unknown`.
  fn char(&mut self, input: &mut &'a str, char: &'a str) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("char", vec![], None);

    if input.starts_with(char) {
      let remaining = &input[char.len()..];
      cst.add(Some(CST::new(char, vec![], None)));
      *input = remaining;

      Ok((remaining, Some(cst)))
    } else {
      Err(ParserError::Unknown)
    }
  }

  /// Parses a sequence of expressions from the input.
  /// Each expression in the sequence must match in order for the sequence match to be successful.
  /// If any expression fails to match, the entire sequence match fails.
  fn sequence(
    &mut self,
    input: &mut &'a str,
    expressions: &Vec<Expression<'a>>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("sequence", vec![], None);

    for expr in expressions {
      let (remaining, node) = self.match_success(input, expr)?;
      *input = remaining;

      cst.add(node);
    }

    Ok((*input, Some(cst)))
  }

  /// Parses a choice of expressions from the input.
  /// Matches the first expression that succeeds.
  /// If none of the expressions match, returns a `ParserError::Unknown`.
  fn choice(
    &mut self,
    input: &mut &'a str,
    expressions: &Vec<Expression<'a>>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    for expr in expressions {
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

  /// Parses zero or more occurrences of the given expression from the input.
  /// Continues to match the expression until it no longer matches.
  /// This will always succeed, even if no occurrences are found.
  fn zero_or_more(
    &mut self,
    input: &mut &'a str,
    expression: &Expression<'a>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("zero_or_more", vec![], None);
    let mut children: Vec<CST<'a>> = Vec::new();

    loop {
      let start_length = input.len();

      match self.match_success(input, expression) {
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

  /// Parses one or more occurrences of the given expression from the input.
  /// Continues to match the expression until it no longer matches.
  /// If no occurrences are found, returns a `ParserError::FailedToMatch`.
  fn one_or_more(
    &mut self,
    input: &mut &'a str,
    expression: &Expression<'a>,
  ) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let start_length = input.len();
    let (remaining, cst) = self.match_success(input, &Expression::ZeroOrMore(Box::new(expression.clone())))?;

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

  /// Parses a named rule from the grammar.
  /// Looks up the rule by name and applies its expression to the input.
  /// If the rule is not found, returns a `ParserError::RuleNotFound`.
  fn named_rule(&mut self, input: &mut &'a str, name: &'a str) -> Result<(&'a str, Option<CST<'a>>), ParserError<'a>> {
    let key = (name, *input);
    let original_input = *input;

    // Check if the result has already been parsed and what the memo state is.
    // If it is seeding, we have left recursive expression.
    // If None then we continue with the parse.
    if let Some(state) = self.packrat.get(key) {
      match state {
        Ok(State::Seeding) => {
          return Err(ParserError::LeftRecursion { name });
        }
        Ok(State::Parsed(remaining, cst)) => {
          *input = remaining;
          return Ok((remaining, cst.clone()));
        }
        Ok(State::Failed(err)) => {
          return Err(err.clone());
        }
        Err(err) => {
          return Err(err.clone());
        }
      }
    }

    let was_seeding = self.packrat.is_seeding();

    self.packrat.mark(key);

    let rule = self
      .grammar
      .get(name)
      .ok_or(ParserError::RuleNotFound { position: 0, name })?;

    let (mut remaining, cst) = match self.match_success(input, &rule.expression) {
      Ok((r, c)) => (r, c),
      Err(e) => {
        self.packrat.insert(key, Err(e.clone()));
        return Err(e);
      }
    };

    let mut tree = CST::new(rule.name, cst.map_or_else(Vec::new, |c| vec![c]), None);

    self
      .packrat
      .insert(key, Ok(State::Parsed(remaining, Some(tree.clone()))));

    if !was_seeding {
      loop {
        let prev_remaining = remaining;
        let prev_tree = tree.clone();

        self.packrat.set_seeding(true);

        // Whilst trying to parse the result ensure packrat is seeding.
        let mut try_input = original_input;
        let result = self.match_success(&mut try_input, &rule.expression);

        self.packrat.set_seeding(false);

        match result {
          Ok((r, c)) => {
            if r.len() >= prev_remaining.len() {
              break;
            }

            remaining = r;
            if let Some(c) = c {
              tree = CST::new(rule.name, vec![c], None);
            }

            self
              .packrat
              .insert(key, Ok(State::Parsed(remaining, Some(tree.clone()))));
          }
          Err(_e) => {
            tree = prev_tree;
            remaining = prev_remaining;
            break;
          }
        }
      }
    }

    *input = remaining;
    Ok((remaining, Some(tree)))
  }
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty grammar input.
  fn default() -> Self {
    Parser::new(Grammar::default())
  }
}

#[cfg(test)]
mod grammar_tests {
  use super::*;

  #[test]
  fn test_grammar_insert() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("e", Expression::Empty));

    assert_eq!(grammar.rules.get("e"), Some(&Rule::new("e", Expression::Empty)));
    assert_eq!(grammar.rules.get("b"), None);
  }

  #[test]
  fn test_grammar_with() {
    let grammar = Grammar::default().with(Rule::new("e", Expression::Empty));

    assert_eq!(grammar.rules.get("e"), Some(&Rule::new("e", Expression::Empty)));
    assert_eq!(grammar.rules.get("b"), None);
  }

  #[test]
  fn test_grammar_get() {
    let mut grammar = Grammar::default();
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
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Char("a")));

    {
      let mut parser = Parser::new(grammar.clone());
      let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
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
          assert_eq!(node.child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_char_fail() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("a", Expression::Char("a")));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "b", "a");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_sequence_success() {
    {
      let mut grammar = Grammar::default();
      grammar.insert(Rule::new("rule", Expression::Sequence(vec![Expression::Char("a")])));

      let mut parser = Parser::new(grammar);
      let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

      assert!(remaining.is_empty());
      match cst {
        Some(mut node) => {
          node.pretty_print(Some(0));
          assert_eq!(node.get(), "rule");
          assert_eq!(node.child(0).unwrap().get(), "sequence");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }

    {
      let mut grammar = Grammar::default();
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().child(0).unwrap().get(), "b");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_sequence_fail() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Sequence(vec![Expression::Char("a")])));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "b", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_choice_success() {
    let mut grammar = Grammar::default();
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "b");
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "b");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_choice_fail() {
    let mut grammar = Grammar::default();
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
    let mut grammar = Grammar::default();
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().child(0).unwrap().get(), "a");
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_zero_or_more_fail() {
    let mut grammar = Grammar::default();
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
    let mut grammar = Grammar::default();
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
          assert_eq!(node.child(0).unwrap().get(), "one_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().child(0).unwrap().get(), "a");
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
          assert_eq!(node.child(0).unwrap().get(), "one_or_more");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(1).unwrap().child(0).unwrap().get(), "a");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(2).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_one_or_more_fail() {
    let mut grammar = Grammar::default();
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
    let mut grammar = Grammar::default();
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
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
          assert_eq!(node.child(0).unwrap().child(0).unwrap().get(), "char");
          assert_eq!(node.child(0).unwrap().child(0).unwrap().child(0).unwrap().get(), "a");
        }
        None => assert!(false),
      }
    }
  }

  #[test]
  fn test_parser_parse_named_rule_fail() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Char("a")));
    grammar.insert(Rule::new("named", Expression::NamedRule("rule")));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "b", "named");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_left_recursion() {
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );
    let rule_x = Rule::new("rule_x", Expression::NamedRule("rule_expr"));
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_x"),
          Expression::Char("+"),
          Expression::NamedRule("rule_num"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );

    let grammar = Grammar::default().with(rule_num).with(rule_x).with(rule_expr);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    match cst {
      Some(_) => assert!(true),
      None => assert!(false),
    }
  }
}
