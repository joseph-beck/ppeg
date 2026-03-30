use crate::prelude::Expression;

/// Context is used for parsing expressions and storing state during parsing.
/// It holds the current position and the input string being parsed.
/// Throughout parsing the context is updated to reflect the current parser state.
#[derive(Debug, Clone, PartialEq)]
pub struct Context<'a> {
  /// Current position in the input.
  pub pos: usize,
  /// The input string being parsed.
  pub input: &'a str,

  pub choices: Vec<Expression<'a>>,
}

impl<'a> Context<'a> {
  /// Creates a new Context with the given position and input string.
  pub fn new(pos: usize, input: &'a str, _choice_depth: usize) -> Self {
    Context {
      pos,
      input,

      choices: Vec::new(),
    }
  }

  pub fn choice_depth(&mut self, choice: Expression<'a>) -> usize {
    if self.choices.contains(&choice) {
      self.choices.iter().position(|c| c == &choice).unwrap_or(0)
    } else {
      self.choices.push(choice);
      self.choices.len()
    }
  }

  pub fn reset_choice_depth(&mut self) {
    self.choices.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_context_new() {
    let context = Context::new(0, "test", 0);

    assert_eq!(context.pos, 0);
    assert_eq!(context.input, "test");
    assert_eq!(context.choices, Vec::new());
  }
}
