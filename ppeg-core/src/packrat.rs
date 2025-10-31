use std::collections::HashMap;

use crate::parser::Output;

/// Packrat memoization table for storing previous parser state.
pub struct Packrat<'a> {
  /// Memoization table mapping (grammar rule, input position) to parser output.
  memo_table: HashMap<(&'a str, usize), Output<'a>>,
}

impl<'a> Packrat<'a> {
  pub fn new() -> Self {
    Packrat {
      memo_table: HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: (&'a str, usize), output: Output<'a>) {
    self.memo_table.insert(key, output);
  }

  pub fn get(&self, key: (&'a str, usize)) -> Option<&Output<'a>> {
    self.memo_table.get(&key)
  }
}

impl<'a> Default for Packrat<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::parser::Expression;

  #[test]
  fn test_packrat_new() {
    let packrat: Packrat = Packrat::new();
    assert!(packrat.memo_table.is_empty());
  }

  #[test]
  fn test_packrat_default() {
    let packrat: Packrat = Packrat::default();
    assert!(packrat.memo_table.is_empty());
  }

  #[test]
  fn test_packrat_insert() {
    let mut packrat: Packrat = Packrat::new();
    let key = ("rule1", 0);
    let output = Output::new(Ok(vec!["a", "b"]), Expression::Empty);

    packrat.insert(key, output);

    assert!(packrat.memo_table.get(&key).is_some());
    assert!(packrat.memo_table.get(&("rule1", 1)).is_none());
  }

  #[test]
  fn test_packrat_get() {
    let mut packrat: Packrat = Packrat::new();
    let key = ("rule1", 0);
    let output = Output::new(Ok(vec!["a", "b"]), Expression::Empty);
    packrat.memo_table.insert(key, output);

    assert!(packrat.get(key).is_some());
    assert!(packrat.get(("rule1", 1)).is_none());
  }
}
