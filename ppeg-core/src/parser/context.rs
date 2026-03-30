/// Context is used for parsing expressions and storing state during parsing.
/// It holds the current position and the input string being parsed.
/// Throughout parsing the context is updated to reflect the current parser state.
#[derive(Debug, Clone, PartialEq)]
pub struct Context<'a> {
  /// Current position in the input.
  pub pos: usize,
  /// The input string being parsed.
  pub input: &'a str,
  /// Current choice depth of the context, used for tracking nested choices in the grammar.
  pub choice_depth: usize,
}

impl<'a> Context<'a> {
  /// Creates a new Context with the given position and input string.
  pub fn new(pos: usize, input: &'a str, choice_depth: usize) -> Self {
    Context {
      pos,
      input,
      choice_depth,
    }
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
    assert_eq!(context.choice_depth, 0);
  }
}
