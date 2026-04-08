//! Parser contexts are used to store and transfer state during parsing.
//! A Context holds the current position and input being parsed.
//! For productive PEG parsing it also stores the current choice depth and rule being parsed.

/// Context is used for parsing expressions and storing state during parsing.
/// It holds the current position and the input string being parsed.
/// Throughout parsing the context is updated to reflect the current parser state.
#[derive(Debug, Clone, PartialEq)]
pub struct Context<'a> {
  /// Current position in the input.
  pub pos: usize,
  /// The input string being parsed.
  pub input: &'a str,
  /// Stores the current choice depth of the context.
  pub current_choice_depth: usize,
  /// Stores the current rule being parsed.
  pub current_rule_name: &'a str,
}

impl<'a> Context<'a> {
  /// Creates a new Context with the given position and input string.
  pub fn new(pos: usize, input: &'a str, current_choice_depth: usize, current_rule_name: &'a str) -> Self {
    Context {
      pos,
      input,
      current_choice_depth,
      current_rule_name,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_context_new() {
    let context = Context::new(0, "test", 0, "test_rule");

    assert_eq!(context.pos, 0);
    assert_eq!(context.input, "test");
    assert_eq!(context.current_choice_depth, 0);
    assert_eq!(context.current_rule_name, "test_rule");
  }
}
