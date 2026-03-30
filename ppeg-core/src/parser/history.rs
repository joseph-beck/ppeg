#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'a> {
  Ch(usize, usize),
  Nm(&'a str),
  Seq(usize),
  Ct(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct History<'a> {
  nodes: Vec<Node<'a>>,
}

impl<'a> History<'a> {
  pub fn new() -> Self {
    History { nodes: Vec::new() }
  }

  pub fn add(&mut self, node: Node<'a>) {
    self.nodes.push(node);
  }

  pub fn get(&self) -> &Vec<Node<'a>> {
    &self.nodes
  }

  pub fn prod(&self, node: Node<'a>) -> bool {
    !self.nodes.contains(&node)
  }

  pub fn clear(&mut self) {
    self.nodes.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_history_new() {
    let history = History::new();
    assert!(history.nodes.is_empty());
  }

  #[test]
  fn test_history_add() {
    let mut history = History::new();
    history.add(Node::Nm("test"));

    assert_eq!(history.nodes, vec![Node::Nm("test")]);
  }

  #[test]
  fn test_history_get() {
    let mut history = History::new();
    history.add(Node::Nm("test"));

    assert_eq!(history.get(), &vec![Node::Nm("test")]);
  }

  #[test]
  fn test_history_prod() {
    let mut history = History::new();
    let mut node = Node::Nm("test");
    assert!(history.prod(node.clone()));

    history.add(node.clone());
    assert!(!history.prod(node));

    history.clear();

    node = Node::Ch(0, 0);
    assert!(history.prod(node.clone()));

    history.add(node.clone());
    assert!(!history.prod(node));
  }

  #[test]
  fn test_history_clear() {
    let mut history = History::new();
    history.add(Node::Nm("test"));

    history.clear();
    assert!(history.nodes.is_empty());
  }
}
