//! PPEG meta defines the meta grammar that can be used to write grammars.
//! The meta grammar is translated into a grammar that the parser uses to parse an input.
//!
//! Character
//! - A single character that must be matched for the rule to progress the input.
//! - For example the rule `char_c := { 'c' }` matches the character 'c'.
//!
//! Sequence
//! - A series of rule that must be in order for the rule to match.
//! - For example the rule `sequence := { char_c, char_c }` matches character 'c' followed by character 'c'.
//!
//! Choice
//! - A series of rules that any one of them can match for the rule to match.
//! - For example the rule `choice := { char_c | 'b' }` matches either character 'c' or character 'b'.
//!
//! Zero or more
//! - A rule that can match zero or more times.
//! - For example the rule `zero_or_more := { char_c* }` matches zero or more characters 'c'.
//!
//! One or more
//! - A rule that must match one or more times.
//! - For example the rule `one_or_more := { char_c+ }` matches one or more characters 'c'.
//!
//! Named rule
//! - A rule that matches the result of another rule.
//! - For example the rule `named_rule := { char_c }` matches the result of the rule `char_c`.
//! - This is more powerful when used in other rules, for example `sequence := { char_c, named_rule }`.
//! - Which matches with `char_c` twice.
//!
//! An example using this meta grammar to define some rules of a grammar.
//! ```txt
//! // A character rule that matches the character 'c'.
//! char_c := { 'c' }
//!
//! // The sequence of characters that must be matched in order for the rule to match.
//! // In this case must match with character 'c' followed by character 'c'.
//! sequence := { char_c, char_c }
//!
//! // The pipe symbol indicates that the given rule can match either the left or right side of the pipe.
//! // Trying to match with the left side first, then right side if the left side fails.
//! choice := { char_c | sequence }
//!
//! // The asterisk indicates that the given rule can match zero or more times.
//! zero_or_more := { char_c* }
//!
//! // The plus sign indicates that the given rule must match one or more times.
//! one_or_more := { char_c+ }
//!
//! // Takes the name of the rule to match
//! named_rule := { char_c }
//!
//! // A more complex rule that is matches with 'a' or a sequence of 'b' followed by 'c'.
//! complex := { 'a' | { 'b', char_c } }
//! ```
//!
//! ## Usage
//! ```rust
//! use ppeg_core::meta::Meta;
//! use ppeg_core::parse;
//!
//! let grammar = Meta::new().generate("
//!   char_c := { 'c' }
//!   rule := { char_c, char_c }
//! ").unwrap();
//!
//! let (remaining, cst) = parse(&grammar, "rule", "cc").unwrap();
//! ```

use crate::{
  error::MetaError,
  parser::{Expression, Grammar, Rule},
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Meta {}

impl<'a> Meta {
  /// Create a new Meta instance.
  /// Used for generating a grammar from a string or file input.
  pub fn new() -> Self {
    Self {}
  }

  /// Generates a grammar from the given input string.
  /// The input should follow the syntax defined in the module documentation.
  /// Returns a `Grammar` if the input is valid, otherwise returns a `MetaError
  pub fn generate(self, input: &'a str) -> Result<Grammar<'a>, MetaError> {
    Parser::new(input).parse_grammar()
  }

  /// Generates a grammar from the given file path.
  /// Reads the file content and then generates a grammar from it.
  /// Returns a `Grammar` if the file content is valid, otherwise returns a `MetaError`.
  pub fn file(self, path: &str) -> Result<Grammar<'static>, MetaError> {
    let input = std::fs::read_to_string(path).map_err(|e| MetaError::InvalidInput {
      position: 0,
      input: format!("failed to read file '{}': {}", path, e),
    })?;

    // back to this dirty trick...
    let input_leaked: &'static str = Box::leak(input.into_boxed_str());

    Meta::new().generate(input_leaked)
  }
}

struct Parser<'a> {
  /// The input being parsed into a grammar.
  input: &'a str,
  /// Current position within the input.
  /// Ensure this is advanced by the size of a utf8 character (c.len_utf8()).
  position: usize,
}

impl<'a> Parser<'a> {
  /// Creates a new MetaParser for the given input.
  /// Starts at position 0.
  fn new(input: &'a str) -> Self {
    Parser { input, position: 0 }
  }

  /// Returns the character at the current position,
  /// or `None` if at end of input.
  fn current(&self) -> Option<char> {
    self.input[self.position..].chars().next()
  }

  /// Advances the position by the utf8 length of the current character.
  fn advance(&mut self) {
    if let Some(c) = self.current() {
      self.position += c.len_utf8();
    }
  }

  /// Consumes any whitespace and comments.
  /// Comments start with `//` and continue until the end of the line.
  /// Comments can be at the start of a line or inline at the end of a line.
  fn consume_comments_and_whitespace(&mut self) {
    loop {
      while let Some(c) = self.current() {
        if !c.is_whitespace() {
          break;
        }

        self.advance();
      }

      if !self.input[self.position..].starts_with("//") {
        break;
      }

      while let Some(c) = self.current() {
        self.advance();
        if c == '\n' {
          break;
        }
      }
    }
  }

  /// Consumes a character if it matches the expected character,
  /// otherwise returns an error.
  fn consume(&mut self, expected: char) -> Result<(), MetaError> {
    match self.current() {
      Some(c) if c == expected => {
        self.advance();
        Ok(())
      }
      _ => Err(MetaError::InvalidExpression {
        position: self.position,
        expression: format!("expected '{}'", expected),
      }),
    }
  }

  /// Parses the input into a Grammar by parsing all defined rules within the input.
  /// Returns an error if any rule definition is invalid or syntax errors.
  fn parse_grammar(&mut self) -> Result<Grammar<'a>, MetaError> {
    let mut grammar = Grammar::default();

    self.consume_comments_and_whitespace();

    while self.position < self.input.len() {
      let rule = self.parse_rule()?;
      grammar.insert(rule);

      self.consume_comments_and_whitespace();
    }

    Ok(grammar)
  }

  /// Parses a PEG rule from an input that follows the syntax `rule_name := { expression }`.
  /// First parses the identifier, then `:=`, open curly, the expression and finally closed curly.
  fn parse_rule(&mut self) -> Result<Rule<'a>, MetaError> {
    let name = self.parse_identifier()?;

    self.consume_comments_and_whitespace();
    self.consume(':')?;
    self.consume('=')?;
    self.consume_comments_and_whitespace();
    self.consume('{')?;
    self.consume_comments_and_whitespace();

    // parse_choice kicks off expression parsing.
    let expression = self.parse_choice()?;

    self.consume_comments_and_whitespace();
    self.consume('}')?;

    Ok(Rule::new(name, expression))
  }

  // Parses the identifier, either parses the name of a rule or the value of a named rule.
  fn parse_identifier(&mut self) -> Result<&'a str, MetaError> {
    let start = self.position;

    match self.current() {
      Some(c) => {
        if c.is_alphabetic() || c == '_' {
          self.advance()
        }
      }
      _ => {
        return Err(MetaError::InvalidExpression {
          position: self.position,
          expression: "".to_string(),
        });
      }
    }

    while let Some(c) = self.current() {
      if !c.is_alphanumeric() && c != '_' {
        break;
      }

      self.advance();
    }

    Ok(&self.input[start..self.position])
  }

  /// Parses an expression, which can be a character,
  /// a sequence, a choice, a named rule or an expression followed by `*` or `+`.
  /// If followed by `*` or `+`, the expression is parsed
  /// as a zero or more or one or more expression respectively.
  fn parse_expression(&mut self) -> Result<Expression<'a>, MetaError> {
    let expr = match self.current() {
      Some('\'') => self.parse_char()?,
      Some('{') => {
        self.advance();
        self.consume_comments_and_whitespace();

        let expr = self.parse_choice()?;

        self.consume_comments_and_whitespace();
        self.consume('}')?;

        expr
      }
      Some(c) if c.is_alphabetic() || c == '_' => self.parse_named_rule()?,
      Some(c) => {
        return Err(MetaError::InvalidExpression {
          position: self.position,
          expression: format!("unexpected character '{}'", c),
        });
      }
      None => {
        return Err(MetaError::InvalidExpression {
          position: self.position,
          expression: "unexpected end of input".to_string(),
        });
      }
    };

    match self.current() {
      Some('*') => self.parse_zero_or_more(expr),
      Some('+') => self.parse_one_or_more(expr),
      _ => Ok(expr),
    }
  }

  /// Parses a character expression, which is a single character wrapped in single quotes.
  /// For example `'c'` matches the character 'c'.
  fn parse_char(&mut self) -> Result<Expression<'a>, MetaError> {
    self.consume('\'')?;
    let start = self.position;

    while let Some(c) = self.current() {
      if c == '\'' {
        break;
      }

      self.advance();
    }

    if self.position == start {
      return Err(MetaError::InvalidExpression {
        position: self.position,
        expression: "".to_string(),
      });
    }

    let chr = &self.input[start..self.position];
    self.consume('\'')?;

    Ok(Expression::Char(chr))
  }

  /// Parses a sequence of expressions separated by commas.
  /// For example `{ char_c, 'b' }` matches a sequence of `char_c` followed by character 'b'.
  fn parse_sequence(&mut self) -> Result<Expression<'a>, MetaError> {
    let mut sequence = vec![self.parse_expression()?];

    loop {
      self.consume_comments_and_whitespace();
      if self.current() != Some(',') {
        break;
      }

      self.advance();
      self.consume_comments_and_whitespace();

      sequence.push(self.parse_expression()?);
    }

    if sequence.len() == 1 {
      return Ok(sequence.remove(0));
    }

    Ok(Expression::Sequence(sequence))
  }

  /// Parses a choice of expressions separated by pipe symbols.
  /// For example `{ char_c | 'b' }` matches either `char_c` or character 'b'.
  fn parse_choice(&mut self) -> Result<Expression<'a>, MetaError> {
    let mut choices = vec![self.parse_sequence()?];

    loop {
      self.consume_comments_and_whitespace();

      if self.current() != Some('|') {
        break;
      }

      self.advance();
      self.consume_comments_and_whitespace();

      choices.push(self.parse_sequence()?);
    }

    if choices.len() == 1 {
      return Ok(choices.remove(0));
    }

    Ok(Expression::Choice(choices))
  }

  /// Parses an expression that is followed by a `+` operator, indicating it must match one or more times.
  /// For example `char_c+` matches one or more characters 'c'.
  fn parse_one_or_more(&mut self, expr: Expression<'a>) -> Result<Expression<'a>, MetaError> {
    self.consume('+')?;

    Ok(Expression::OneOrMore(Box::new(expr)))
  }

  /// Parses an expression that is followed by a `*` operator, indicating it can match zero or more times.
  /// For example `char_c*` matches zero or more characters 'c'.
  fn parse_zero_or_more(&mut self, expr: Expression<'a>) -> Result<Expression<'a>, MetaError> {
    self.consume('*')?;

    Ok(Expression::ZeroOrMore(Box::new(expr)))
  }

  /// Parses a named rule, which is an identifier that matches the result of another rule.
  /// For example `named_rule := { char_c }` matches the result of the rule
  fn parse_named_rule(&mut self) -> Result<Expression<'a>, MetaError> {
    let name = self.parse_identifier()?;
    Ok(Expression::NamedRule(name))
  }
}

#[cfg(test)]
mod meta_tests {
  use super::*;

  #[test]
  fn test_meta_new() {
    let meta = Meta::new();
    assert_eq!(meta, Meta {});
  }

  #[test]
  fn test_meta_generate() {
    let meta = Meta::new();
    let grammar = meta.generate("char_c := { 'c' }").unwrap();

    let expected_grammar = Grammar::default().with(Rule::new("char_c", Expression::Char("c")));

    assert_eq!(grammar, expected_grammar);
  }
}

#[cfg(test)]
mod parser_tests {
  use super::*;

  #[test]
  fn test_parser_new() {
    let parser = Parser::new("input");
    assert_eq!(parser.input, "input");
    assert_eq!(parser.position, 0);
  }

  #[test]
  fn test_parser_current() {
    let parser = Parser::new("abc");
    assert_eq!(parser.current(), Some('a'));
  }

  #[test]
  fn test_parser_consume() {
    let mut parser = Parser::new("abc");
    assert_eq!(parser.consume('a'), Ok(()));
    assert_eq!(parser.current(), Some('b'));
  }

  #[test]
  fn test_parser_current_and_advance() {
    let mut parser = Parser::new("abc");
    assert_eq!(parser.current(), Some('a'));
    parser.advance();
    assert_eq!(parser.current(), Some('b'));
    parser.advance();
    assert_eq!(parser.current(), Some('c'));
    parser.advance();
    assert_eq!(parser.current(), None);
  }
}
