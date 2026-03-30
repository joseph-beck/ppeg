/// Context is used for parsing expressions and storing state during parsing.
/// It holds the current position and the input string being parsed.
/// Throughout parsing the context is updated to reflect the current parser state.
#[derive(Debug, Clone, PartialEq)]
pub struct Context<'a> {
  /// Current position in the input.
  pub pos: usize,
  /// The input string being parsed.
  pub input: &'a str,
}

impl<'a> Context<'a> {
  /// Creates a new Context with the given position and input string.
  pub fn new(pos: usize, input: &'a str) -> Self {
    Context { pos, input }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_context_new() {
    let context = Context::new(0, "test");

    assert_eq!(context.pos, 0);
    assert_eq!(context.input, "test");
  }
}
