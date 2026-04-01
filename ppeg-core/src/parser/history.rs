#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Artifact<'a> {
  Ch(usize, usize, &'a str),
  Nm(&'a str),
  Seq(usize),
  Ct(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct History<'a> {
  artifacts: Vec<Artifact<'a>>,
}

impl<'a> History<'a> {
  pub fn new() -> Self {
    History { artifacts: Vec::new() }
  }

  pub fn push(&mut self, node: Artifact<'a>) {
    self.artifacts.push(node);
  }

  pub fn artifacts(&self) -> &Vec<Artifact<'a>> {
    &self.artifacts
  }

  pub fn prod(&self, node: Artifact<'a>) -> bool {
    !self.artifacts.contains(&node)
  }

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
  fn test_history_add() {
    let mut history = History::new();
    history.push(Artifact::Nm("test"));

    assert_eq!(history.artifacts, vec![Artifact::Nm("test")]);
  }

  #[test]
  fn test_history_artifacts() {
    let mut history = History::new();
    history.push(Artifact::Nm("test"));

    assert_eq!(history.artifacts(), &vec![Artifact::Nm("test")]);
  }

  #[test]
  fn test_history_prod() {
    let mut history = History::new();
    let mut node = Artifact::Nm("test");
    assert!(history.prod(node.clone()));

    history.push(node.clone());
    assert!(!history.prod(node));

    history.clear();

    node = Artifact::Ch(0, 0, "a");
    assert!(history.prod(node.clone()));

    history.push(node.clone());
    assert!(!history.prod(node));

    let other_node = Artifact::Ch(0, 0, "b");
    assert!(history.prod(other_node));
  }

  #[test]
  fn test_history_clear() {
    let mut history = History::new();
    history.push(Artifact::Nm("test"));

    history.clear();
    assert!(history.artifacts.is_empty());
  }
}
