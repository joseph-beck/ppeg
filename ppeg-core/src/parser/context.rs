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

  /// Creates a new ContextBuilder for building a Context instance.
  pub fn builder() -> ContextBuilder<'a> {
    ContextBuilder::new()
  }
}

/// ContextBuilder is a builder for creating Context instances.
pub struct ContextBuilder<'a> {
  pos: usize,
  input: &'a str,
  current_choice_depth: usize,
  current_rule_name: &'a str,
}

/// Provides a default implementation for ContextBuilder, which inits all fields.
/// pos: 0, input: "", current_choice_depth: 0, current_rule_name: "".
impl<'a> Default for ContextBuilder<'a> {
  fn default() -> Self {
    ContextBuilder {
      pos: 0,
      input: "",
      current_choice_depth: 0,
      current_rule_name: "",
    }
  }
}

impl<'a> ContextBuilder<'a> {
  /// Creates a new ContextBuilder with default values.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the position for the context.
  pub fn pos(mut self, pos: usize) -> Self {
    self.pos = pos;

    self
  }

  /// Sets the input string for the context.
  pub fn input(mut self, input: &'a str) -> Self {
    self.input = input;

    self
  }

  /// Sets the current choice depth for the context.
  pub fn current_choice_depth(mut self, current_choice_depth: usize) -> Self {
    self.current_choice_depth = current_choice_depth;

    self
  }

  /// Sets the current rule name for the context.
  pub fn current_rule_name(mut self, current_rule_name: &'a str) -> Self {
    self.current_rule_name = current_rule_name;

    self
  }

  /// Builds and returns the final Context.
  /// Before building, it asserts current_rule_name is not empty, as it is required.
  pub fn build(self) -> Context<'a> {
    assert!(!self.current_rule_name.is_empty(), "rule name cannot be empty");

    Context {
      pos: self.pos,
      input: self.input,
      current_choice_depth: self.current_choice_depth,
      current_rule_name: self.current_rule_name,
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

  #[test]
  fn test_context_builder() {
    let context = ContextBuilder::new()
      .pos(1)
      .input("test_input")
      .current_choice_depth(2)
      .current_rule_name("test_rule")
      .build();

    assert_eq!(context.pos, 1);
    assert_eq!(context.input, "test_input");
    assert_eq!(context.current_choice_depth, 2);
    assert_eq!(context.current_rule_name, "test_rule");
  }

  #[test]
  fn test_context_builder_new() {
    let context = ContextBuilder::new()
      .pos(1)
      .input("test_input")
      .current_choice_depth(2)
      .current_rule_name("test_rule")
      .build();

    assert_eq!(context.pos, 1);
    assert_eq!(context.input, "test_input");
    assert_eq!(context.current_choice_depth, 2);
    assert_eq!(context.current_rule_name, "test_rule");
  }

  #[test]
  #[should_panic(expected = "rule name cannot be empty")]
  fn test_context_builder_empty_rule_name() {
    ContextBuilder::new()
      .pos(1)
      .input("test_input")
      .current_choice_depth(2)
      .current_rule_name("")
      .build();
  }
}
