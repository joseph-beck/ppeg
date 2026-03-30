//! Rule is used for named rules that are stored in a PEG grammar.
//! It uses both a name and an expression that applies when the rule is referenced in
//! other expressions, such as Expression::NamedRule.
//!
//! ## Example
//! ```rust
//! use ppeg_core::{rule::Rule, expression::Expression};
//!
//! // Create the rule "ab" that matches the sequence "a" followed by "b".
//! let rule = Rule::new(
//!   "ab",
//!  Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
//! )
//!
//! // Use a named expression to reference the rule in another rule.
//! let named_expr = Expression::NamedRule("ab");
//!
//! // Add the rule to a grammar and parse an input string.
//! let mut grammar = Grammar::default();
//! grammar.insert(rule);
//!
//! // Parse using the rule "ab" as the starting rule.
//! let mut parser = Parser::new(grammar);
//! let (remaining, cst) = parser.parse(&mut "ab", "ab").unwrap();
//! ```

use crate::parser::expression::Expression;

/// Rule is used for named rules that are stored in a grammar.
/// It contains both a name and an expression that applies to the rule.
/// The name is used to reference the rule in other expressions, such as Expression::NamedRule.
/// It can also be used to start parsing, as PEG requires a starting rule to begin parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
  /// As this is a named rule, it needs a name!
  name: &'a str,
  /// Expression that applies to this rule.
  /// For example Expression::Choice.
  expression: Expression<'a>,
}

impl<'a> Rule<'a> {
  /// Create a new rule with the given name and expression.
  pub fn new(name: &'a str, expression: Expression<'a>) -> Self {
    Rule { name, expression }
  }

  /// Get the expression of the rule.
  pub fn expression(&self) -> &Expression<'a> {
    &self.expression
  }

  /// Get the name of the rule.
  pub fn name(&self) -> &'a str {
    self.name
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::expression::Expression;

  #[test]
  fn test_rule_new() {
    let rule = Rule::new(
      "a_and_b",
      Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
    );

    assert_eq!(rule.name, "a_and_b");
    assert_eq!(
      rule.expression,
      Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")])
    );
  }

  #[test]
  fn test_rule_name() {
    let rule = Rule::new(
      "a_and_b",
      Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
    );

    assert_eq!(rule.name(), "a_and_b");
  }

  #[test]
  fn test_rule_expression() {
    let rule = Rule::new(
      "a_and_b",
      Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")]),
    );

    assert_eq!(
      rule.expression(),
      &Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")])
    );
  }
}
