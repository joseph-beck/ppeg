//! PPEG Packrat parsing module holds the logic and data structures used for Packrat parsing.
//! Packrat parsing uses a memoization table to store previous parsing results,
//! allowing for parses to be seeded and left recursive expressions to be handled without infinite loops.

use std::collections::HashMap;

use crate::{
  cst::CST,
  parser::{error::ParserError, context::Context},
};

/// When parsing a rule using Packrat parsing state must be tracked.
/// This enum represents the possible states of parsing a rule.
#[derive(Debug, Clone, PartialEq)]
pub enum State<'a> {
  /// The parser is currently seeding this rule.
  Seeding,
  /// The rule has been successfully parsed.
  Parsed(Context<'a>, Option<CST<'a>>),
  /// The rule failed to parse.
  Failed(ParserError<'a>),
}

/// Packrat memo table for storing previous parser state.
#[derive(Clone, Debug, PartialEq)]
pub struct Packrat<'a> {
  /// Memo table mapping (grammar rule, input position) to parser output.
  memo_table: HashMap<(&'a str, usize), Result<State<'a>, ParserError<'a>>>,
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
  pub fn get(&self, key: (&'a str, usize)) -> Option<&Result<State<'a>, ParserError<'a>>> {
    self.memo_table.get(&key)
  }

  /// Insert a memoed result for a given (rule, input) pair.
  pub fn insert(&mut self, key: (&'a str, usize), output: Result<State<'a>, ParserError<'a>>) {
    self.memo_table.insert(key, output);
  }

  /// Remove a memoed result for a given (rule, input) pair.
  pub fn remove(&mut self, key: (&'a str, usize)) {
    self.memo_table.remove(&key);
  }

  // Removes all memoed entries from the memo table.
  pub fn clear(&mut self) {
    self.memo_table.clear();
  }

  /// Removes all memoed entries except for the given key, rule with a given input,
  /// and the rule that must be kept.
  pub fn clear_except(&mut self, key: (&'a str, usize)) {
    self.memo_table.retain(|&k, _| key == k);
  }

  /// Mark a rule at a given input position as being in the seeding state.
  pub fn mark(&mut self, key: (&'a str, usize)) {
    self.insert(key, Ok(State::Seeding));
  }

  /// Update the memo table only if the current state is Seeding.
  pub fn update_when_seeding(&mut self, key: (&'a str, usize), result: Result<State<'a>, ParserError<'a>>) {
    let data = self.get(key);

    if let Some(Ok(State::Seeding)) = data {
      self.insert(key, result);
    }
  }

  /// Set the state of the seeding flag.
  pub fn set_seeding(&mut self, seeding: bool) {
    self.seeding = seeding;
  }

  /// Check if currently seeding.
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

  #[test]
  fn test_packrat_get() {
    let mut packrat = Packrat::new();
    packrat.memo_table.insert(("a", 0), Ok(State::Seeding));

    let result = packrat.get(("a", 0));
    assert_eq!(result, Some(&Ok(State::Seeding)));
  }

  #[test]
  fn test_packrat_insert() {
    let mut packrat = Packrat::new();
    packrat.insert(("a", 0), Ok(State::Seeding));

    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, Some(&Ok(State::Seeding)));
  }

  #[test]
  fn test_packrat_remove() {
    let mut packrat = Packrat::new();
    packrat.memo_table.insert(("a", 0), Ok(State::Seeding));

    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, Some(&Ok(State::Seeding)));

    packrat.remove(("a", 0));
    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, None);
  }

  #[test]
  fn test_packrat_clear() {
    let mut packrat = Packrat::new();
    packrat.memo_table.insert(("a", 0), Ok(State::Seeding));
    packrat
      .memo_table
      .insert(("b", 1), Ok(State::Parsed(Context::new(0, "input2"), None)));
    packrat
      .memo_table
      .insert(("a", 2), Ok(State::Failed(ParserError::Unknown)));

    packrat.clear();

    assert_eq!(packrat.memo_table.len(), 0);
  }

  #[test]
  fn test_packrat_clear_except() {
    let mut packrat = Packrat::new();
    packrat.memo_table.insert(("a", 0), Ok(State::Seeding));
    packrat
      .memo_table
      .insert(("b", 1), Ok(State::Parsed(Context::new(0, "input2"), None)));
    packrat
      .memo_table
      .insert(("a", 2), Ok(State::Failed(ParserError::Unknown)));

    packrat.clear_except(("a", 0));

    assert_eq!(packrat.memo_table.len(), 1);
    assert_eq!(packrat.memo_table.get(&("a", 0)), Some(&Ok(State::Seeding)));
  }

  #[test]
  fn test_packrat_mark() {
    let mut packrat = Packrat::new();
    packrat.mark(("a", 0));

    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, Some(&Ok(State::Seeding)));
  }

  #[test]
  fn test_packrat_update_when_seeding() {
    let mut packrat = Packrat::new();

    {
      packrat.memo_table.insert(("a", 0), Ok(State::Seeding));

      packrat.update_when_seeding(("a", 0), Ok(State::Parsed(Context::new(0, "a"), None)));

      let result = packrat.memo_table.get(&("a", 0));
      assert_eq!(result, Some(&Ok(State::Parsed(Context::new(0, "a"), None))));
    }

    {
      packrat
        .memo_table
        .insert(("c", 1), Ok(State::Parsed(Context::new(0, "c"), None)));

      packrat.update_when_seeding(("c", 1), Ok(State::Failed(ParserError::Unknown)));

      let result = packrat.memo_table.get(&("c", 1));
      assert_eq!(result, Some(&Ok(State::Parsed(Context::new(0, "c"), None))));
    }
  }

  #[test]
  fn test_packrat_set_seeding() {
    let mut packrat = Packrat::new();
    assert_eq!(packrat.is_seeding(), false);

    packrat.set_seeding(true);
    assert_eq!(packrat.is_seeding(), true);

    packrat.set_seeding(false);
    assert_eq!(packrat.is_seeding(), false);
  }

  #[test]
  fn test_packrat_is_seeding() {
    let mut packrat = Packrat::new();
    assert_eq!(packrat.is_seeding(), false);

    packrat.seeding = true;
    assert_eq!(packrat.is_seeding(), true);

    packrat.seeding = false;
    assert_eq!(packrat.is_seeding(), false);
  }
}
