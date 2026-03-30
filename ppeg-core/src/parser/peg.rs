//! PPEG Parser module holds the logic for parsing expressions defined in PEG grammars.
//! It includes the definitions for expressions, rules, and grammars.

use std::vec;

use crate::parser::{
  context::Context,
  cst::{CST, Label},
  error::ParserError,
  expression::Expression,
  grammar::Grammar,
  packrat::{Packrat, State},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
  /// Grammars lookup table.
  /// Stores all the named rules of the parser.
  grammar: Grammar<'a>,
  /// Packrat memoization table.
  /// Stores parser states so that left recursion can be handled.
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
    let context = Context::new(0, input);
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
      Expression::Not(_expr) => Err(ParserError::NotImplemented {
        position: context.pos,
        name: "not",
      }),
      Expression::Optional(_expr) => Err(ParserError::NotImplemented {
        position: context.pos,
        name: "optional",
      }),
      Expression::ZeroOrMore(expr) => self.zero_or_more(context, expr),
      Expression::OneOrMore(expr) => self.one_or_more(context, expr),
      Expression::NamedRule(n) => self.named_rule(context, n),
    }
  }

  fn empty(&mut self, context: Context<'a>) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    Ok((context, None))
  }

  /// Parses a character from the input and advances the input when successful.
  /// If the character does not match, returns a `ParserError::Unknown`.
  fn char(&mut self, context: Context<'a>, char: &'a str) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut cst = CST::new("char", vec![], Some(Label::default().with_hidden(true)));
    let mut ctx = context.clone();

    // When matching a character advanced the input by one character.
    if context.input[context.pos..].starts_with(char) {
      cst.add(Some(CST::new(char, vec![], None)));
      ctx.pos += char.len();

      Ok((ctx, Some(cst)))
    } else {
      Err(ParserError::Unknown)
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
    for expr in expressions {
      match self.match_success(context.clone(), expr) {
        Ok((ctx, cst)) => match cst {
          Some(n) => {
            return Ok((
              ctx,
              Some(CST::new("choice", vec![n], Some(Label::default().with_hidden(true)))),
            ));
          }
          None => return Ok((ctx, None)),
        },
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
    context: Context<'a>,
    expression: &Expression<'a>,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let mut children: Vec<CST<'a>> = Vec::new();
    let mut ctx = context.clone();

    // Zero or more continues until no progress is made on the input.
    loop {
      let start_pos = context.pos;

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
  /// If the rule is not found, returns a `ParserError::RuleNotFound`.
  fn named_rule(
    &mut self,
    context: Context<'a>,
    name: &'a str,
  ) -> Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>> {
    let key = (name, context.pos);
    let original_ctx = context.clone();

    // Check if the result has already been parsed and what the memo state is.
    // If it is seeding, we have left recursive expression, it returns and error and continues resolving.
    // If None then we continue with the parse.
    if let Some(state) = self.packrat.get(key) {
      match state {
        Ok(State::Seeding) => {
          return Err(ParserError::LeftRecursion { name });
        }
        Ok(State::Parsed(ctx, cst)) => {
          return Ok((ctx.clone(), cst.clone()));
        }
        Ok(State::Failed(err)) => {
          return Err(err.clone());
        }
        Err(err) => {
          return Err(err.clone());
        }
      }
    }

    // Check if packrat was seeding and mark the current rule for seeding.
    let was_seeding = self.packrat.is_seeding();
    self.packrat.mark(key);

    let rule = self
      .grammar
      .get(name)
      .ok_or(ParserError::RuleNotFound { position: 0, name })?;

    let ctx = context.clone();

    let (mut new_ctx, cst) = match self.match_success(ctx.clone(), &rule.expression()) {
      Ok((r, c)) => (r, c),
      Err(err) => {
        self.packrat.insert(key, Err(err.clone()));
        return Err(err);
      }
    };

    let mut tree = CST::new(
      rule.name(),
      cst.map_or_else(Vec::new, |c| vec![c]),
      Some(Label::default().with_hidden(true)),
    );

    self
      .packrat
      .insert(key, Ok(State::Parsed(new_ctx.clone(), Some(tree.clone()))));

    if !was_seeding {
      loop {
        let prev_ctx = new_ctx.clone();
        let prev_tree = tree.clone();

        // Clear all memoed entries except for the current rule being processed.
        // Supports indirect left recursion.
        self.packrat.clear_except((name, original_ctx.clone().pos));
        // Whilst trying to parse the result ensure packrat is seeding.
        self.packrat.set_seeding(true);

        let try_ctx = original_ctx.clone();
        let result = self.match_success(try_ctx, &rule.expression());

        // Stop seeding after trying to parse and check the result.
        self.packrat.set_seeding(false);

        match result {
          Ok((r, c)) => {
            if r.pos >= prev_ctx.pos {
              break;
            }

            new_ctx = r;
            if let Some(c) = c {
              tree = CST::new(rule.name(), vec![c], Some(Label::default().with_hidden(true)));
            }

            self
              .packrat
              .insert(key, Ok(State::Parsed(new_ctx.clone(), Some(tree.clone()))));
          }
          Err(_) => {
            // Failed to parse the left recursive expression here.
            // Revert back to previous state.
            tree = prev_tree;
            new_ctx = prev_ctx;

            break;
          }
        }
      }
    }

    Ok((new_ctx, Some(tree)))
  }
}

impl<'a> Default for Parser<'a> {
  /// Creates a default parser with an empty grammar input.
  fn default() -> Self {
    Parser::new(Grammar::default())
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
      Some(CST::new("rule", vec![], Some(Label::default().with_hidden(true))))
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
  fn test_parser_parse_not() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Not(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "a", "rule");

    assert!(result.is_err());
  }

  #[test]
  fn test_parser_parse_optional() {
    let mut grammar = Grammar::default();
    grammar.insert(Rule::new("rule", Expression::Optional(Box::new(Expression::Char("a")))));

    let mut parser = Parser::new(grammar);
    let result = parser.parse(&mut "a", "rule");

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
          Expression::NamedRule("rule_num"),
        ]),
        Expression::NamedRule("rule_num"),
      ]),
    );

    let grammar = Grammar::default().with(rule_num).with(rule_x).with(rule_expr);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2+3", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    match cst {
      Some(_) => assert!(true),
      None => assert!(false),
    }
  }

  #[test]
  fn test_parser_parse_direct_left_recursion() {
    let rule_expr = Rule::new(
      "rule_expr",
      Expression::Choice(vec![
        Expression::Sequence(vec![
          Expression::NamedRule("rule_expr"),
          Expression::Char("+"),
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

    let grammar = Grammar::default().with(rule_expr).with(rule_num);

    let mut parser = Parser::new(grammar);
    let (remaining, cst) = parser.parse(&mut "1+2+3", "rule_expr").unwrap();

    assert!(remaining.is_empty());
    match cst {
      Some(_) => assert!(true),
      None => assert!(false),
    }
  }
}
