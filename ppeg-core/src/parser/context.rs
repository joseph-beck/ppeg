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
  /// Stores the choices made during parsing.
  /// Helps track the current choice depth of the context.
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
    match self.choices.contains(&choice) {
      true => self.choices.iter().position(|c| c == &choice).unwrap_or(0),
      false => {
        self.choices.push(choice);
        self.choices.len() - 1
      }
    }
  }

  pub fn reset_choices(&mut self) {
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

  #[test]
  fn test_context_choice_depth() {
    let mut context = Context::new(0, "test", 0);

    let choice = Expression::Choice(vec![Expression::Char("a"), Expression::Char("b")]);

    assert_eq!(context.choice_depth(choice.clone()), 0);
  }

  #[test]
  fn test_context_reset_choices() {
    let mut context = Context::new(0, "test", 0);

    let choice = Expression::Choice(vec![Expression::Char("a"), Expression::Char("b")]);

    context.choice_depth(choice.clone());
    assert_eq!(context.choices.len(), 1);

    context.reset_choices();
    assert_eq!(context.choices.len(), 0);
  }
}
