pub mod cst;
pub mod error;
pub mod input;
pub mod parser;

pub trait ParserInput<'a> {
  type Item;

  fn current(&self) -> Option<Self::Item>;

  fn length(&self) -> usize;

  fn remaining(&self) -> usize;
}
