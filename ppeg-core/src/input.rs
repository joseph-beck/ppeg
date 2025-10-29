pub trait ParserInput<'a> {
  type Item;

  fn current(&self) -> Option<Self::Item>;

  fn length(&self) -> usize;

  fn remaining(&self) -> usize;
}

pub struct Input<'a> {
  /// Data stored in the input.
  data: Vec<&'a str>,
  /// Current position in the input data.
  position: usize,
}

impl<'a> Input<'a> {
  /// Creates a new Input instance with the given data vector.
  /// position is defaulted to 0.
  pub fn new(data: Vec<&'a str>) -> Self {
    Input { data, position: 0 }
  }
}

impl<'a> Iterator for Input<'a> {
  type Item = &'a str;

  /// Returns the item at the current position and advances the position by one.
  fn next(&mut self) -> Option<Self::Item> {
    match self.position < self.data.len() {
      true => {
        let item = self.data[self.position];
        self.position += 1;
        Some(item)
      }
      false => None,
    }
  }
}

impl<'a> ParserInput<'a> for Input<'a> {
  type Item = &'a str;

  /// Returns the current item without advancing the position.
  fn current(&self) -> Option<Self::Item> {
    match self.position < self.data.len() {
      true => Some(self.data[self.position]),
      false => None,
    }
  }

  /// Returns the total size of the input vec, not the remaining items.
  fn length(&self) -> usize {
    self.data.len()
  }

  /// Returns the remaining items in the input vec.
  fn remaining(&self) -> usize {
    self.data.len() - self.position
  }
}

impl<'a> Default for Input<'a> {
  /// Creates a default Input with an empty data vec.
  fn default() -> Self {
    Input::new(vec![])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_input_new() {
    let input = Input::new(vec!["test"]);
    assert_eq!(input.length(), 1);
    assert_eq!(input.current(), Some("test"));
  }

  #[test]
  fn test_input_new_empty_vec() {
    let input = Input::new(vec![]);
    assert_eq!(input.length(), 0);
    assert_eq!(input.current(), None);
  }

  #[test]
  fn test_input_next() {
    let mut input = Input::new(vec!["a", "b", "c"]);

    assert_eq!(input.next(), Some("a"));
    assert_eq!(input.next(), Some("b"));
    assert_eq!(input.next(), Some("c"));

    assert_eq!(input.next(), None);
  }

  #[test]
  fn test_input_current() {
    let mut input = Input::new(vec!["a", "b"]);
    assert_eq!(input.current(), Some("a"));

    input.next();
    assert_eq!(input.current(), Some("b"));

    input.next();
    assert_eq!(input.current(), None);
  }

  #[test]
  fn test_input_length() {
    let mut input = Input::new(vec!["a", "b", "c"]);
    assert_eq!(input.length(), 3);

    input.next();
    assert_eq!(input.length(), 3);
  }

  #[test]
  fn test_input_remaining() {
    let mut input = Input::new(vec!["a", "b", "c"]);
    assert_eq!(input.remaining(), 3);

    input.next();
    assert_eq!(input.remaining(), 2);
  }
}
