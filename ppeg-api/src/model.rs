use ppeg_core::{cst, parser};
use serde::{Deserialize, Serialize};

/// Duplicate of ppeg_core::parser::Expression but with owned Strings for serde.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Expression {
  Empty,
  Char(String),
  Sequence(Vec<Expression>),
  Choice(Vec<Expression>),
  OneOrMore(Box<Expression>),
  ZeroOrMore(Box<Expression>),
  NamedRule(String),
}

impl Expression {
  pub fn to_parser_expression(&self) -> parser::Expression<'static> {
    match self {
      Expression::Empty => parser::Expression::Empty,
      Expression::Char(char) => parser::Expression::Char(Box::leak(char.clone().into_boxed_str())),
      Expression::Sequence(exprs) => {
        parser::Expression::Sequence(exprs.iter().map(|expr| expr.to_parser_expression()).collect())
      }
      Expression::Choice(exprs) => {
        parser::Expression::Choice(exprs.iter().map(|expr| expr.to_parser_expression()).collect())
      }
      Expression::OneOrMore(expr) => parser::Expression::OneOrMore(Box::new(expr.to_parser_expression())),
      Expression::ZeroOrMore(expr) => parser::Expression::ZeroOrMore(Box::new(expr.to_parser_expression())),
      Expression::NamedRule(name) => parser::Expression::NamedRule(Box::leak(name.clone().into_boxed_str())),
    }
  }
}

/// Duplicate of ppeg_core::parser::Rule but with owned Strings for serde.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
  name: String,
  expression: Expression,
}

impl Rule {
  pub fn new(name: String, expression: Expression) -> Self {
    Rule { name, expression }
  }

  pub fn to_parser_rule(&self) -> parser::Rule<'static> {
    parser::Rule {
      name: Box::leak(self.name.clone().into_boxed_str()),
      expression: self.expression.to_parser_expression(),
    }
  }
}

/// Duplicate of ppeg_core::parser::Grammar but with owned Strings for serde.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grammar {
  rules: Vec<Rule>,
}

impl Grammar {
  pub fn new(rules: Option<Vec<Rule>>) -> Self {
    Grammar {
      rules: rules.unwrap_or_default(),
    }
  }

  pub fn to_parser_grammar(&self) -> parser::Grammar<'static> {
    let mut grammar = parser::Grammar::default();

    self.rules.iter().for_each(|rule| grammar.insert(rule.to_parser_rule()));

    grammar
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parse {
  grammar: Grammar,
  input: String,
  rule: String,
}

impl Default for Parse {
  fn default() -> Self {
    Parse {
      grammar: Grammar { rules: vec![] },
      input: String::new(),
      rule: String::new(),
    }
  }
}

impl Parse {
  pub fn new(grammar: Option<Grammar>, input: Option<String>, rule: Option<String>) -> Self {
    Parse {
      grammar: grammar.unwrap_or_else(|| Grammar::new(None)),
      input: input.unwrap_or_default(),
      rule: rule.unwrap_or_default(),
    }
  }

  pub fn grammar(&self) -> Grammar {
    self.grammar.clone()
  }

  pub fn input(&self) -> String {
    self.input.clone()
  }

  pub fn rule(&self) -> String {
    self.rule.clone()
  }
}

/// Duplicate of ppeg_core::cst::CST but with owned Strings for serde.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct CST {
  value: String,
  children: Vec<CST>,
  label: Option<Label>,
}

impl CST {
  pub fn new(value: String, children: Vec<CST>, label: Option<Label>) -> Self {
    CST { value, children, label }
  }

  pub fn from_parser_cst(parser_cst: &cst::CST<'static>) -> CST {
    let children = parser_cst.children().iter().map(CST::from_parser_cst).collect();

    CST {
      value: parser_cst.get().to_string(),
      children,
      label: Label::from_parser_label(parser_cst.label()),
    }
  }
}

/// Duplicate of ppeg_core::cst::Label but with owned Strings for serde.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Label {
  hidden: bool,
  productive: bool,
}

impl Label {
  pub fn new(productive: bool, hidden: bool) -> Self {
    Label { productive, hidden }
  }

  pub fn from_parser_label(parser_label: Option<&cst::Label>) -> Option<Label> {
    parser_label.map(|label| Label {
      hidden: label.is_hidden(),
      productive: label.is_productive(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Output {
  remaining: String,
  cst: Option<CST>,
}

impl Output {
  pub fn new(remaining: String, cst: Option<CST>) -> Self {
    Output { remaining, cst }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_expression_to_parser_expression() {
    let expression = Expression::Char("x".to_string());

    let parser_expression = expression.to_parser_expression();

    assert_eq!(parser_expression, parser::Expression::Char("x"));
  }

  #[test]
  fn test_rule_to_parser_rule() {
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::Char("x".to_string()),
    };

    let parser_rule = rule.to_parser_rule();

    assert_eq!(parser_rule.name, "test_rule");
    assert_eq!(parser_rule.expression, parser::Expression::Char("x"));
  }

  #[test]
  fn test_grammar_to_parser_grammar() {
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::Char("x".to_string()),
    };

    let grammar = Grammar { rules: vec![rule] };

    let parser_grammar = grammar.to_parser_grammar();

    let retrieved_rule = parser_grammar.get("test_rule").unwrap();

    assert_eq!(retrieved_rule.name, "test_rule");
    assert_eq!(retrieved_rule.expression, parser::Expression::Char("x"));
  }

  #[test]
  fn test_cst_from_parser_cst() {
    let parser_cst = cst::CST::new("root", vec![cst::CST::new("child", vec![], None)], None);

    let cst = CST::from_parser_cst(&parser_cst);

    assert_eq!(cst.value, "root");
    assert_eq!(cst.children.len(), 1);
    assert_eq!(cst.children[0].value, "child");
  }
}
