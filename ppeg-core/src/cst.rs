/// Concrete Syntax Tree (CST).
pub struct CST<'a> {
  /// Value stored at this CST node.
  value: &'a str,
  /// Child nodes of this CST node.
  children: Vec<CST<'a>>,
}

impl<'a> CST<'a> {
  pub fn new(value: &'a str, children: Vec<CST<'a>>) -> Self {
    CST { value, children }
  }

  /// Gets the value of this CST node.
  pub fn get(&self) -> &'a str {
    self.value
  }

  /// Add a child CST node to the current CST.
  pub fn add(&mut self, child: CST<'a>) {
    self.children.push(child);
  }
}

impl<'a> Default for CST<'a> {
  fn default() -> Self {
    Self::new("", Vec::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cst_new() {
    let cst = CST::new("root", vec![CST::new("child", vec![])]);

    assert_eq!(cst.value, "root");
    assert_eq!(cst.children[0].value, "child");
  }

  #[test]
  fn test_cst_default() {
    let cst: CST = Default::default();

    assert_eq!(cst.value, "");
    assert!(cst.children.is_empty());
  }

  #[test]
  fn test_cst_get() {
    let cst = CST::new("root", vec![CST::new("child", vec![])]);
    assert_eq!(cst.get(), "root");
  }

  #[test]
  fn test_cst_add() {
    let mut cst = CST::new("root", vec![]);

    cst.add(CST::new("child1", vec![]));
    cst.add(CST::new("child2", vec![]));

    assert_eq!(cst.children.len(), 2);
    assert_eq!(cst.children[0].value, "child1");
    assert_eq!(cst.children[1].value, "child2");
  }
}
