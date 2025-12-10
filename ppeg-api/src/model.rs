use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
  pub fn to_parser_expression(&self) -> ppeg_core::parser::Expression<'static> {
    match self {
      Expression::Empty => ppeg_core::parser::Expression::Empty,
      Expression::Char(c) => {
        ppeg_core::parser::Expression::Char(Box::leak(c.clone().into_boxed_str()))
      }
      Expression::Sequence(exprs) => ppeg_core::parser::Expression::Sequence(
        exprs.iter().map(|e| e.to_parser_expression()).collect(),
      ),
      Expression::Choice(exprs) => ppeg_core::parser::Expression::Choice(
        exprs.iter().map(|e| e.to_parser_expression()).collect(),
      ),
      Expression::OneOrMore(e) => {
        ppeg_core::parser::Expression::OneOrMore(Box::new(e.to_parser_expression()))
      }
      Expression::ZeroOrMore(e) => {
        ppeg_core::parser::Expression::ZeroOrMore(Box::new(e.to_parser_expression()))
      }
      Expression::NamedRule(n) => {
        ppeg_core::parser::Expression::NamedRule(Box::leak(n.clone().into_boxed_str()))
      }
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
  name: String,
  expression: Expression,
}

impl Rule {
  pub fn new(name: String, expression: Expression) -> Self {
    Rule { name, expression }
  }

  pub fn to_parser_rule(&self) -> ppeg_core::parser::Rule<'static> {
    ppeg_core::parser::Rule {
      name: Box::leak(self.name.clone().into_boxed_str()),
      expression: self.expression.to_parser_expression(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grammar {
  rules: Vec<Rule>,
}

impl Grammar {
  pub fn new(rules: Option<Vec<Rule>>) -> Self {
    Grammar {
      rules: rules.unwrap_or_else(Vec::new),
    }
  }

  pub fn to_parser_grammar(&self) -> ppeg_core::parser::Grammar<'static> {
    let mut grammar = ppeg_core::parser::Grammar::new();

    self
      .rules
      .iter()
      .for_each(|rule| grammar.insert(rule.to_parser_rule()));

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
      input: input.unwrap_or_else(String::new),
      rule: rule.unwrap_or_else(String::new),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_expression_to_parser_expression() {
    let expression = Expression::Char("x".to_string());

    let parser_expression = expression.to_parser_expression();

    assert_eq!(parser_expression, ppeg_core::parser::Expression::Char("x"));
  }

  #[test]
  fn test_rule_to_parser_rule() {
    let rule = Rule {
      name: "test_rule".to_string(),
      expression: Expression::Char("x".to_string()),
    };

    let parser_rule = rule.to_parser_rule();

    assert_eq!(parser_rule.name, "test_rule");
    assert_eq!(
      parser_rule.expression,
      ppeg_core::parser::Expression::Char("x")
    );
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
    assert_eq!(
      retrieved_rule.expression,
      ppeg_core::parser::Expression::Char("x")
    );
  }
}
