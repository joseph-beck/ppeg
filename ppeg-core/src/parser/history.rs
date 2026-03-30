#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
  Ch(u32, u32),
  Nm(&'a str),
  Seq(u32),
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
}
