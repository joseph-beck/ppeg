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
  pub fn new(pos: usize, input: &'a str) -> Self {
    Context { pos, input }
  }
}
