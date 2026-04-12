//! PEG module holds the main logic for parsing using PEGs.
//! The parser takes in an input, grammar and a starting rule and
//! returns the remaining input and the CST generated from parsing the input.
//!
//! This implementation of PEG uses Packrat parsing to avoid super linear parse times.
//! Packrat stores previous results of parsing named rules in a memoization table.
//! The parser also uses History to store the choices made by the parser,
//! using history the parser can parse left recursive expressions by determining if a choice is productive or not.
//!
//! ## Example
//! ```rust
//! use ppeg_core::prelude::*;
//!
//! // Create a grammar with a single rule "ab" that matches the sequence "a" followed by "b".
//! let mut grammar = Grammar::default();
//! grammar.insert(Rule::new(
//!   "ab",
//!   Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
//! ));
//!
//! // Parse the input "ab" using the rule "ab" as the starting rule.
//! let mut parser = Parser::new(grammar);
//! let (remaining, cst) = parser.parse(&mut "ab", "ab").unwrap();
//! ```

use std::vec;

use crate::parser::{
  context::Context,
  cst::{CST, Label},
  error::ParserError,
  expression::Expression,
  grammar::Grammar,
  history::{Artifact, History},
  packrat::Packrat,
};

/// Parser, a PEG parser, that parses the input using the grammar and its rule.
/// Using the grammar, packrat and history it can parse left recursive expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Grammars lookup table.
  /// Stores all the named rules of the parser.
  grammar: Grammar<'a>,
  /// Packrat memoization table.
  /// Stores parser states so that left recursion can be handled.
  packrat: Packrat<'a>,
  /// Parser history for parsing left recursive expressions.
  /// Stores history of parser states to avoid infinite recursion.
  history: History<'a>,
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty grammar input.
  fn default() -> Self {
    Parser::new(Grammar::default())
  }
}

impl<'a> Parser<'a> {
  /// Creates a new instance of the parser with the given grammar.
  /// Packrat and History are created and begin empty.
  pub fn new(grammar: Grammar<'a>) -> Self {
    Parser {
      grammar,
      packrat: Packrat::new(),
      history: History::new(),
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
    // Creates a context that starts at the position 0 with the input.
    let context = Context::new(0, input, 0, rule_name);

    // Parse must always start with a named rule.
    // Simply call named rule here.
    match self.named_rule(context, rule_name) {
      Ok((ctx, cst)) => Ok((&ctx.input[ctx.pos..], cst)),
      Err(err) => Err(err),
    }
  }

  /// Matches the given input against the provided expression.
  /// Returns the remaining input and CST if successful, otherwise returns a ParserError.
  fn match_success(
    &mut self,
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    match expression {
      Expression::Empty => self.empty(context),
      Expression::Char(char) => self.char(context, char),
      Expression::Sequence(exprs) => self.sequence(context, exprs),
      Expression::Choice(exprs) => self.choice(context, exprs),
      Expression::Not(expr) => self.not(context, expr),
      Expression::Optional(expr) => self.optional(context, expr),
      Expression::ZeroOrMore(expr) => self.zero_or_more(context, expr),
      Expression::OneOrMore(expr) => self.one_or_more(context, expr),
      Expression::NamedRule(n) => self.named_rule(context, n),
    }
  }

  /// Parses an empty expression, which always succeeds without consuming any input.
  /// Returns the same context and None for the CST.
  fn empty(&mut self, context: Context<'a>) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    Ok((context, None))
  }

  /// Parses a character from the input and advances the input when successful.
  /// If the character does not match, returns a `ParserError::Unknown`.
  fn char(&mut self, context: Context<'a>, char: &'a str) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("char", vec![], Some(Label::default().with_hidden(true)));
    let mut ctx = context.clone();

    // When matching a character advanced the input by one character.
    // History can also be wiped here as we have been productive.
    if context.input[context.pos..].starts_with(char) {
      cst.add(Some(CST::new(char, vec![], None)));
      ctx.pos += char.len();

      // Reset choice depth and history as we have been productive.
      self.history.clear();

      Ok((ctx, Some(cst)))
    } else {
      Err(ParserError::Unknown {
        message: format!(
          "failed to match char {:?} at position {}: remaining input {:?}",
          char,
          context.pos,
          &context.input[context.pos..]
        ),
      })
    }
  }

  /// Parses a sequence of expressions from the input.
  /// Each expression in the sequence must match in order for the sequence match to be successful.
  /// If any expression fails to match, the entire sequence match fails.
  fn sequence(
    &mut self,
    context: Context<'a>,
    expressions: &Vec<Expression<'a>>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("sequence", vec![], Some(Label::default().with_hidden(true)));
    let mut ctx = context.clone();

    for expr in expressions {
      let (new_ctx, node) = self.match_success(ctx, expr)?;
      ctx = new_ctx;

      cst.add(node);
    }

    Ok((ctx, Some(cst)))
  }

  /// Parses a choice of expressions from the input.
  /// Matches the first expression that succeeds.
  /// If none of the expressions match, returns a `ParserError::Unknown`.
  fn choice(
    &mut self,
    context: Context<'a>,
    expressions: &Vec<Expression<'a>>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut ctx = context.clone();
    ctx.current_choice_depth += 1;

    let snapshot = self.history.clone();

    for (i, expr) in expressions.iter().enumerate() {
      self.history = snapshot.clone();

      let artifact = Artifact::Ch(i, ctx.current_choice_depth, ctx.current_rule_name);

      if !self.history.prod(artifact.clone()) {
        continue;
      }

      self.history.preserve(artifact.clone());

      match self.match_success(ctx.clone(), expr) {
        Ok((c, cst)) => match cst {
          Some(n) => {
            return Ok((
              c,
              Some(CST::new("choice", vec![n], Some(Label::default().with_hidden(true)))),
            ));
          }
          None => return Ok((c, None)),
        },
        Err(_) => continue,
      }
    }
    Err(ParserError::FailedToMatch {
      position: context.pos,
      input: context.input,
      name: "choice",
    })
  }

  /// Parses a not expression.
  /// In this case the expression must fail to match for the parse to be successful.
  /// If the expression matches and advances the input, returns a `ParserError::FailedToMatch`.
  fn not(
    &mut self,
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    match self.match_success(context.clone(), expression) {
      Ok(_) => Err(ParserError::FailedToMatch {
        position: context.pos,
        input: context.input,
        name: "not",
      }),
      Err(_) => Ok((
        context,
        Some(CST::new("not", vec![], Some(Label::default().with_hidden(true)))),
      )),
    }
  }

  /// Parses an optional expression.
  /// In this case the expression may match or fail to match, but cases are successful.
  fn optional(
    &mut self,
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    match self.match_success(context.clone(), expression) {
      Ok((ctx, cst)) => {
        let mut node = CST::new("optional", vec![], Some(Label::default().with_hidden(true)));

        if let Some(child) = cst {
          node.add(Some(child));
        }

        Ok((ctx, Some(node)))
      }
      Err(_) => Ok((context, None)),
    }
  }

  /// Parses zero or more occurrences of the given expression from the input.
  /// Continues to match the expression until it no longer matches.
  /// This will always succeed, even if no occurrences are found.
  fn zero_or_more(
    &mut self,
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut children: Vec<CST<'a>> = Vec::new();
    let mut ctx = context.clone();

    // Zero or more continues until no progress is made on the input.
    loop {
      let start_pos = ctx.pos;

      match self.match_success(ctx.clone(), expression) {
        Ok((new_ctx, cst)) => {
          if new_ctx.pos == start_pos {
            break;
          }

          ctx = new_ctx;

          if let Some(n) = cst {
            children.push(n);
          } else {
            break;
          }
        }
        Err(_) => break,
      }
    }

    let mut cst = CST::new("zero_or_more", vec![], Some(Label::default().with_hidden(true)));

    for child in children {
      cst.add(Some(child));
    }

    match cst.is_leaf() {
      true => Ok((ctx.clone(), None)),
      false => Ok((ctx.clone(), Some(cst))),
    }
  }

  /// Parses one or more occurrences of the given expression from the input.
  /// Continues to match the expression until it no longer matches.
  /// If no occurrences are found, returns a `ParserError::FailedToMatch`.
  fn one_or_more(
    &mut self,
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let start_pos = context.pos;
    let (ctx, cst) = self.match_success(context, &Expression::ZeroOrMore(Box::new(expression.clone())))?;

    // When no progress is made one or more has failed to match.
    if ctx.pos == start_pos {
      return Err(ParserError::FailedToMatch {
        position: ctx.pos,
        input: ctx.input,
        name: "one_or_more",
      });
    }

    let mut cst = cst.unwrap();
    cst.set("one_or_more");

    Ok((ctx, Some(cst)))
  }

  /// Parses a named rule from the grammar.
  /// Looks up the rule by name and applies its expression to the input.
  /// First uses the Packrat memo to check if the rule has already been parsed,
  /// if it has that is returned.
  /// Otherwise the rule is parsed, with its result being stored in the Packrat memo table.
  /// If the rule is not found, returns a `ParserError::RuleNotFound`.
  fn named_rule(
    &mut self,
    context: Context<'a>,
    name: &'a str,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let memo_result = self.packrat.get((name, context.pos));
    if let Some(memo) = memo_result {
      return memo.clone();
    }

    let mut ctx = context.clone();

    match self.grammar.get(name) {
      Some(rule) => {
        // Every time we enter a named rule we reset the choice depth and update the current rule.
        ctx.current_choice_depth = 0;
        ctx.current_rule_name = name;

        match self.match_success(ctx.clone(), rule.expression()) {
          Ok((c, cst)) => {
            let mut node = CST::new(name, vec![], Some(Label::default().with_hidden(false)));
            if let Some(n) = cst {
              node.add(Some(n));
            }

            let result = Ok((c, Some(node)));
            self.packrat.insert((name, context.pos), result.clone());

            result
          }
          Err(err) => {
            let result = Err(err);
            self.packrat.insert((name, context.pos), result.clone());

            result
          }
        }
      }
      None => Err(ParserError::RuleNotFound {
        name,
        position: context.pos,
      }),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::rule::Rule;

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
  fn test_parser_parse_empty() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Empty));

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "", "rule").unwrap();

    assert!(remaining.is_empty());
    assert_eq!(
      cst,
      Some(CST::new("rule", vec![], Some(Label::default().with_hidden(false))))
    );
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
  fn test_parser_parse_not_success() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Not(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "b", "rule").unwrap();

    assert_eq!(remaining, "b");
    match cst {
      Some(mut node) => {
        assert_eq!(node.get(), "rule");
        assert_eq!(node.child(0).unwrap().get(), "not");
      }
      None => assert!(false),
    }
  }

  #[test]
  fn test_parser_parse_not_fail() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Not(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "a", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_optional_success() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Optional(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "a", "rule").unwrap();

    assert!(remaining.is_empty());
    match cst {
      Some(mut node) => {
        assert_eq!(node.get(), "rule");
        assert_eq!(node.child(0).unwrap().get(), "optional");
      }
      None => assert!(false),
    }
  }

  #[test]
  fn test_parser_parse_optional_fail() {
    // Optional should never fail, but this is the case it matches nothing and returns None as the CST.
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Optional(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "b", "rule").unwrap();

    assert_eq!(remaining, "b");
    match cst {
      Some(node) => {
        assert_eq!(node.get(), "rule");
      }
      None => assert!(false),
    }
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
  fn test_parser_parse_indirect_left_recursion() {
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
          Expression::NamedRule("rule_x"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );

    let grammar = Grammar::default().with(rule_num).with(rule_x).with(rule_expr);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2+3", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    assert!(cst.is_some());
  }

  #[test]
  fn test_parser_parse_direct_left_recursion() {
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
          Expression::NamedRule("rule_expr"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );

    let grammar = Grammar::default().with(rule_expr).with(rule_num);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2+3", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    assert!(cst.is_some());
  }

  #[test]
  fn test_parser_parse_direct_left_recursion_nested() {
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
          Expression::NamedRule("rule_term"),
        ]),
        Expression::NamedRule("rule_term"),
      ]),
    );
    let rule_term = Rule::new(
      "rule_term",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_term"),
          Expression::Char("*"),
          Expression::NamedRule("rule_num"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );

    let grammar = Grammar::default().with(rule_expr).with(rule_term).with(rule_num);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1*2+3*1", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    assert!(cst.is_some());
  }

  #[test]
  fn test_parser_parse_direct_left_recursion_more_choices() {
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("*"),
          Expression::NamedRule("rule_expr"),
        ]),
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
          Expression::NamedRule("rule_expr"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );

    let grammar = Grammar::default().with(rule_expr).with(rule_num);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2+3", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    assert!(cst.is_some());
  }

  #[test]
  fn test_parser_parse_direct_left_recursion_base_case() {
    // This is more a of a sanity check, ensuring it defaults to the base case correctly
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
          Expression::NamedRule("rule_expr"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );

    let grammar = Grammar::default().with(rule_expr).with(rule_num);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+4", "rule_expr").unwrap();

    assert_eq!(remaining, "+4");
    assert!(cst.is_some());
  }

  #[test]
  fn test_parser_parse_direct_left_recursion_with_fail() {
    // Another sanity check, ensuring it completely fails on invalid input.
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
          Expression::NamedRule("rule_expr"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );
    let rule_num = Rule::new(
      "rule_num",
      Expression::OneOrMore(Box::new(Expression::Choice(vec![
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
      ]))),
    );

    let grammar = Grammar::default().with(rule_expr).with(rule_num);

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "4+1", "rule_expr");

    assert!(result.is_err());
  }
}
