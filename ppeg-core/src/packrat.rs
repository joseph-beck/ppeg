use std::collections::HashMap;

use crate::{cst::CST, error::ParserError};

/// When parsing a rule using Packrat parsing state must be tracked.
/// This enum represents the possible states of parsing a rule.
#[derive(Debug, Clone, PartialEq)]
pub enum State<'a> {
  /// The parser is currently seeding this rule.
  Seeding,
  /// The rule has been successfully parsed.
  Parsed(&'a str, Option<CST<'a>>),
  /// The rule failed to parse.
  Failed(ParserError<'a>),
}

/// Packrat memo table for storing previous parser state.
#[derive(Clone, Debug, PartialEq)]
pub struct Packrat<'a> {
  /// Memo table mapping (grammar rule, input position) to parser output.
  memo_table: HashMap<(&'a str, &'a str), Result<State<'a>, ParserError<'a>>>,
  /// Seeding flag for left recursion handling.
  seeding: bool,
}

impl<'a> Packrat<'a> {
  /// Create a new empty Packrat memo table.
  /// HashMap is defaulted to a new empty map.
  /// Seeding flag is defaulted to false.
  pub fn new() -> Self {
    Packrat {
      memo_table: HashMap::new(),
      seeding: false,
    }
  }

  /// Get a memoed result for a given (rule, input) pair.
  pub fn get(&self, key: (&'a str, &'a str)) -> Option<&Result<State<'a>, ParserError<'a>>> {
    self.memo_table.get(&key)
  }

  /// Insert a memoed result for a given (rule, input) pair.
  pub fn insert(&mut self, key: (&'a str, &'a str), output: Result<State<'a>, ParserError<'a>>) {
    self.memo_table.insert(key, output);
  }

  /// Remove a memoed result for a given (rule, input) pair.
  pub fn remove(&mut self, key: (&'a str, &'a str)) {
    self.memo_table.remove(&key);
  }

  /// Mark a rule at a given input position as being in the seeding state.
  pub fn mark(&mut self, key: (&'a str, &'a str)) {
    self.memo_table.insert(key, Ok(State::Seeding));
  }

  /// Update the memo table only if the current state is Seeding.
  pub fn update_when_seeding(&mut self, key: (&'a str, &'a str), result: Result<State<'a>, ParserError<'a>>) {
    let data = self.get(key);

    if let Some(Ok(State::Seeding)) = data {
      self.memo_table.insert(key, result);
    }
  }

  /// Clears all memoed entries except for the given rule with a given input
  /// and the rule that must be kept.
  pub fn clear_except(&mut self, input: &'a str, keep_rule: &'a str) {
    self
      .memo_table
      .retain(|&(rule, pos), _| pos != input || rule == keep_rule);
  }

  /// Set the seeding flag.
  pub fn set_seeding(&mut self, seeding: bool) {
    self.seeding = seeding;
  }

  /// Check if currently seeding
  pub fn is_seeding(&self) -> bool {
    self.seeding
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
}
