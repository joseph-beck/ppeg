/// Concrete Syntax Tree (CST).
#[derive(Debug, Clone, PartialEq)]
pub struct CST<'a> {
  /// Value stored at this CST node.
  value: &'a str,
  /// Child nodes of this CST node.
  children: Vec<CST<'a>>,
  /// Label for this CST node.
  /// Marks whether this has been productive.
  label: Option<Label>,
}

impl<'a> CST<'a> {
  pub fn new(value: &'a str, children: Vec<CST<'a>>, label: Option<Label>) -> Self {
    CST { value, children, label }
  }

  /// Gets the value of this CST node.
  pub fn get(&self) -> &'a str {
    self.value
  }

  /// Sets the value of this CST node.
  pub fn set(&mut self, value: &'a str) {
    self.value = value;
  }

  /// Add a child CST node to the current CST.
  pub fn add(&mut self, child: Option<CST<'a>>) {
    match child {
      None => (),
      Some(c) => self.children.push(c),
    }
  }

  /// Get a mutable child at the given index.
  pub fn child(&mut self, index: usize) -> Option<&mut CST<'a>> {
    self.children.get_mut(index)
  }

  /// Get all children of this CST.
  pub fn children(&self) -> &Vec<CST<'a>> {
    &self.children
  }

  /// Get the label of this CST.
  pub fn label(&self) -> Option<&Label> {
    self.label.as_ref()
  }

  /// Update the label of this CST node.
  pub fn update_label(&mut self, label: Label) {
    self.label = Some(label);
  }

  /// Has this CST node been labelled as productive?
  pub fn is_productive(&self) -> bool {
    match &self.label {
      Some(label) => label.productive,
      None => false,
    }
  }

  /// Has this CST node been labelled as hidden?
  pub fn is_hidden(&self) -> bool {
    match &self.label {
      Some(label) => label.hidden,
      None => false,
    }
  }

  /// Is this CST node a leaf?
  /// Is the children array empty?
  pub fn is_leaf(&self) -> bool {
    self.children.is_empty()
  }

  /// Pretty print the CST.
  /// Using this node as the root, outputs the tree structure.
  /// For example:
  /// `
  /// root
  /// -child1
  /// --child11
  /// -child2
  /// `
  pub fn pretty_print(&self, indent: Option<usize>) {
    let indent = indent.unwrap_or(0);

    for _ in 0..indent {
      print!("-");
    }

    println!("{}", self.value);

    if self.children.is_empty() {
      return;
    }

    for child in &self.children {
      child.pretty_print(Some(indent + 1));
    }
  }
}

impl<'a> Default for CST<'a> {
  fn default() -> Self {
    Self::new("root", Vec::new(), None)
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Label {
  /// Has this been labelled as productive?
  productive: bool,
  /// Should this item be hidden in the tree?
  hidden: bool,
}

impl Label {
  pub fn new(productive: bool, hidden: bool) -> Self {
    Label { productive, hidden }
  }

  /// Is this label marked as productive?
  pub fn productive(&self) -> bool {
    self.productive
  }

  /// Is this label marked as hidden?
  pub fn hidden(&self) -> bool {
    self.hidden
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cst_new() {
    let cst = CST::new("root", vec![CST::new("child", vec![], None)], None);

    assert_eq!(cst.value, "root");
    assert_eq!(cst.children[0].value, "child");
  }

  #[test]
  fn test_cst_default() {
    let cst: CST = Default::default();

    assert_eq!(cst.value, "root");
    assert!(cst.children.is_empty());
  }

  #[test]
  fn test_cst_get() {
    let cst = CST::new("root", vec![CST::new("child", vec![], None)], None);
    assert_eq!(cst.get(), "root");
  }

  #[test]
  fn test_cst_add() {
    let mut cst = CST::new("root", vec![], None);

    cst.add(Some(CST::new("child1", vec![], None)));
    cst.add(Some(CST::new("child2", vec![], None)));

    assert_eq!(cst.children.len(), 2);
    assert_eq!(cst.children[0].value, "child1");
    assert_eq!(cst.children[1].value, "child2");
  }

  #[test]
  fn test_cst_child() {
    let mut cst = CST::new("root", vec![], None);

    cst.add(Some(CST::new("child1", vec![], None)));
    let child1 = cst.child(0).unwrap();

    assert_eq!(child1.value, "child1");
  }

  #[test]
  fn test_cst_label_productive() {
    let mut cst = CST::new("root", vec![], None);
    assert!(cst.label.is_none());

    cst.update_label(Label::new(true, false));
    assert!(cst.label.is_some());
  }

  #[test]
  fn test_cst_is_productive() {
    let mut cst = CST::new("root", vec![], None);
    cst.update_label(Label::new(true, false));

    assert!(cst.is_productive());
  }

  #[test]
  fn test_cst_is_leaf() {
    let mut cst = CST::new("root", vec![], None);
    assert!(cst.is_leaf());

    cst.add(Some(CST::new("child", vec![], None)));
    assert!(!cst.is_leaf());
  }

  #[test]
  fn test_label_default() {
    let label: Label = Default::default();

    assert!(!label.productive);
  }
}
