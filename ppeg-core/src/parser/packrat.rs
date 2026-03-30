//! PPEG Packrat parsing module holds the logic and data structures used for Packrat parsing.
//! Packrat parsing uses a memoization table to store previous parsing results,
//! This prevents super linear parse times when parsing expressions.
//!
//! ## Example
//! ```rust
//! use ppeg_core::prelude::*;
//! ```

use std::collections::HashMap;

use crate::parser::{context::Context, cst::CST, error::ParserError};

/// Packrat memo table for storing previous parser state.
#[derive(Clone, Debug, PartialEq)]
pub struct Packrat<'a> {
  /// Memo table mapping (grammar rule, input position) to parser output.
  memo_table: HashMap<(&'a str, usize), Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>>>,
}

impl<'a> Packrat<'a> {
  /// Create a new empty Packrat memo table.
  /// HashMap is defaulted to a new empty map.
  pub fn new() -> Self {
    Packrat {
      memo_table: HashMap::new(),
    }
  }

  /// Get a memoed result for a given (rule, input) pair.
  pub fn get(&self, key: (&'a str, usize)) -> Option<&Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>>> {
    self.memo_table.get(&key)
  }

  /// Insert a memoed result for a given (rule, input) pair.
  pub fn insert(&mut self, key: (&'a str, usize), output: Result<(Context<'a>, Option<CST<'a>>), ParserError<'a>>) {
    self.memo_table.insert(key, output);
  }

  /// Remove a memoed result for a given (rule, input) pair.
  pub fn remove(&mut self, key: (&'a str, usize)) {
    self.memo_table.remove(&key);
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
    packrat
      .memo_table
      .insert(("a", 0), Ok((Context::new(0, "input"), None)));

    let result = packrat.get(("a", 0));
    assert_eq!(result, Some(&Ok((Context::new(0, "input"), None))));
  }

  #[test]
  fn test_packrat_insert() {
    let mut packrat = Packrat::new();
    packrat.insert(("a", 0), Ok((Context::new(0, "input"), None)));

    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, Some(&Ok((Context::new(0, "input"), None))));
  }

  #[test]
  fn test_packrat_remove() {
    let mut packrat = Packrat::new();
    packrat
      .memo_table
      .insert(("a", 0), Ok((Context::new(0, "input"), None)));

    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, Some(&Ok((Context::new(0, "input"), None))));

    packrat.remove(("a", 0));
    let result = packrat.memo_table.get(&("a", 0));
    assert_eq!(result, None);
  }
}
