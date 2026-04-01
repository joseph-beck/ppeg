//! History enables the parser to parse left recursive expressions.
//! This uses the history of artifacts to determine if using a certain choice, via prod,
//! is productive or not.
//! When a choice is not productive, the parser can choose to fail and explore a different branch of the expression.
//!
//! Artifacts are bits of information that are stored in the history, this is an enum.
//! The Ch artifact is used to store information about choices,
//! using the choice index, choice depth and the rule name that invoked the choice.

/// Artifacts enum, these are stored in the History.
/// Ch is a Choice artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact<'a> {
  /// Ch is a Choice artifact, it stores the
  /// choice index, choice depth and the rule name that invoked the choice.
  /// The rule that invoked the choice may not have directly invoked the choice,
  /// but it is the closest rule that invoked the choice.
  /// (choice_index, choice_depth, rule_name)
  Ch(usize, usize, &'a str),
}

/// History is used for storing artifacts during parsing.
/// These artifacts are used to see if the parser is parsing productively.
/// When something is not productive, prod will return false, and a choice
/// can be made to fail and explore a different branch of the expression.
#[derive(Debug, Clone, PartialEq)]
pub struct History<'a> {
  /// Vector of artifacts that have been preserved in the history.
  artifacts: Vec<Artifact<'a>>,
}

impl<'a> History<'a> {
  /// Create a new History.
  /// Starts with an empty vector of artifacts.
  pub fn new() -> Self {
    History { artifacts: Vec::new() }
  }

  /// Preserve an artifact in the history.
  /// This pushes the artifact into the vector of artifacts.
  pub fn preserve(&mut self, node: Artifact<'a>) {
    self.artifacts.push(node);
  }

  /// Gets the artifacts in the history.
  /// Returns a clone of the vector of artifacts.
  pub fn artifacts(&self) -> Vec<Artifact<'a>> {
    self.artifacts.clone()
  }

  /// Prod the history with an artifact.
  /// Checks if the artifact already exists in the vector of artifacts.
  /// If it does, returns false. Otherwise, returns true.
  pub fn prod(&self, node: Artifact<'a>) -> bool {
    !self.artifacts.contains(&node)
  }

  /// Clear the history, removes all artifacts from the vector of artifacts.
  pub fn clear(&mut self) {
    self.artifacts.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_history_new() {
    let history = History::new();
    assert!(history.artifacts.is_empty());
  }

  #[test]
  fn test_history_preserve() {
    let mut history = History::new();
    history.preserve(Artifact::Ch(0, 0, "test"));

    assert_eq!(history.artifacts, vec![Artifact::Ch(0, 0, "test")]);
  }

  #[test]
  fn test_history_artifacts() {
    let mut history = History::new();
    history.preserve(Artifact::Ch(0, 0, "test"));

    assert_eq!(history.artifacts(), vec![Artifact::Ch(0, 0, "test")]);
  }

  #[test]
  fn test_history_prod() {
    let mut history = History::new();
    let mut node = Artifact::Ch(0, 0, "test");
    assert!(history.prod(node.clone()));

    history.preserve(node.clone());
    assert!(!history.prod(node));

    history.clear();

    node = Artifact::Ch(0, 0, "a");
    assert!(history.prod(node.clone()));

    history.preserve(node.clone());
    assert!(!history.prod(node));

    let other_node = Artifact::Ch(0, 0, "b");
    assert!(history.prod(other_node));
  }

  #[test]
  fn test_history_clear() {
    let mut history = History::new();
    history.preserve(Artifact::Ch(0, 0, "test"));

    history.clear();
    assert!(history.artifacts.is_empty());
  }
}
