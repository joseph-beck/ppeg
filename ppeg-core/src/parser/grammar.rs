use std::collections::HashMap;

use crate::parser::rule::Rule;

#[derive(Clone, PartialEq)]
pub struct Grammar<'a> {
  /// Stores all of the rules of a grammar.
  /// Mapping of rule name to the rule data.
  /// Ruled data consists of the name and the expression.
  rules: HashMap<&'a str, Rule<'a>>,
}

impl<'a> Grammar<'a> {
  /// Creates a new instance of grammar with the given rules.
  pub fn new(rules: HashMap<&'a str, Rule<'a>>) -> Self {
    Grammar { rules }
  }

  /// Gets all rules stored in the grammar as a vector.
  pub fn rules(&self) -> Vec<&Rule<'a>> {
    self.rules.values().collect()
  }
}

impl Default for Grammar<'_> {
  /// Creates a default empty grammar.
  fn default() -> Self {
    Self::new(HashMap::new())
  }
}

impl std::fmt::Debug for Grammar<'_> {
  /// Debugger formatting.
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

#[cfg(test)]
mod tests {
  use crate::parser::{expression::Expression, rule::Rule};

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
