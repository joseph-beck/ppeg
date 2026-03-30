use crate::parser::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
  /// As this is a named rule, it needs a name!
  pub name: &'a str,
  /// Expression that applies to this rule.
  /// For example Expression::OneAndOne.
  pub expression: Expression<'a>,
}

impl<'a> Rule<'a> {
  /// Create a new rule with the given name and expression.
  pub fn new(name: &'a str, expression: Expression<'a>) -> Self {
    Rule { name, expression }
  }
}
