//! Macros support the creation of PEGs using macros for a more concise and readable syntax.
//! This helps simplify the definitions of grammars, rule and expressions.
//! There are also shortcuts for creating parsers and parsing input using the defined grammars.
//!
//! ## Example
//! ```rust
//! use ppeg_core::{grammar, rule, c, seq, parse};
//!
//! // Create a rule that matches the sequence "a" followed by "b".
//! let a_and_b = rule!("a_and_b" => seq!(c!("a"), c!("b")));
//!
//! let peg = grammar!(
//!   // Insert a predefined rule.
//!   a_and_b,
//!   // Or define a rule inline.
//!   rule!("c" => c!("c"))
//! );
//!
//! // Parse the result and unwrap the error to get the remaining input and CST.
//! let (remaining, cst) = parse!(peg, &mut "ab", "a_and_b").unwrap();
//! ```

/// Shortcut for creating a `Choice` expression that matches any single digit character `[0-9]`.
/// Can be used to create expressions like `number` by combining it with `one_or_more!()`.
///
/// ## Example
/// ```rust
/// use ppeg_core::{digit, one_or_more};
///
/// let number = one_or_more!(digit!());
/// ```
#[macro_export]
macro_rules! digit {
  () => {
    $crate::parser::Expression::Choice(vec![
      $crate::parser::Expression::Char("0"),
      $crate::parser::Expression::Char("1"),
      $crate::parser::Expression::Char("2"),
      $crate::parser::Expression::Char("3"),
      $crate::parser::Expression::Char("4"),
      $crate::parser::Expression::Char("5"),
      $crate::parser::Expression::Char("6"),
      $crate::parser::Expression::Char("7"),
      $crate::parser::Expression::Char("8"),
      $crate::parser::Expression::Char("9"),
    ])
  };
}

/// Shortcut for creating a `Choice` expression that matches any single letter character `[a-zA-Z]`.
/// Can be used to create expressions like `word` by combining it with `one_or_more!()`.
///
/// ## Example
/// ```rust
/// use ppeg_core::{letter, one_or_more};
///
/// let word = one_or_more!(letter!());
/// ```
#[macro_export]
macro_rules! letter {
  () => {
    $crate::parser::Expression::Choice(vec![
      $crate::parser::Expression::Char("a"),
      $crate::parser::Expression::Char("b"),
      $crate::parser::Expression::Char("c"),
      $crate::parser::Expression::Char("d"),
      $crate::parser::Expression::Char("e"),
      $crate::parser::Expression::Char("f"),
      $crate::parser::Expression::Char("g"),
      $crate::parser::Expression::Char("h"),
      $crate::parser::Expression::Char("i"),
      $crate::parser::Expression::Char("j"),
      $crate::parser::Expression::Char("k"),
      $crate::parser::Expression::Char("l"),
      $crate::parser::Expression::Char("m"),
      $crate::parser::Expression::Char("n"),
      $crate::parser::Expression::Char("o"),
      $crate::parser::Expression::Char("p"),
      $crate::parser::Expression::Char("q"),
      $crate::parser::Expression::Char("r"),
      $crate::parser::Expression::Char("s"),
      $crate::parser::Expression::Char("t"),
      $crate::parser::Expression::Char("u"),
      $crate::parser::Expression::Char("v"),
      $crate::parser::Expression::Char("w"),
      $crate::parser::Expression::Char("x"),
      $crate::parser::Expression::Char("y"),
      $crate::parser::Expression::Char("z"),
      $crate::parser::Expression::Char("A"),
      $crate::parser::Expression::Char("B"),
      $crate::parser::Expression::Char("C"),
      $crate::parser::Expression::Char("D"),
      $crate::parser::Expression::Char("E"),
      $crate::parser::Expression::Char("F"),
      $crate::parser::Expression::Char("G"),
      $crate::parser::Expression::Char("H"),
      $crate::parser::Expression::Char("I"),
      $crate::parser::Expression::Char("J"),
      $crate::parser::Expression::Char("K"),
      $crate::parser::Expression::Char("L"),
      $crate::parser::Expression::Char("M"),
      $crate::parser::Expression::Char("N"),
      $crate::parser::Expression::Char("O"),
      $crate::parser::Expression::Char("P"),
      $crate::parser::Expression::Char("Q"),
      $crate::parser::Expression::Char("R"),
      $crate::parser::Expression::Char("S"),
      $crate::parser::Expression::Char("T"),
      $crate::parser::Expression::Char("U"),
      $crate::parser::Expression::Char("V"),
      $crate::parser::Expression::Char("W"),
      $crate::parser::Expression::Char("X"),
      $crate::parser::Expression::Char("Y"),
      $crate::parser::Expression::Char("Z"),
    ])
  };
}

/// Creates an `Empty` expression, which matches the empty string.
///
/// ## Example
/// ```rust
/// use ppeg_core::empty;
///
/// let expr = empty!();
/// ```
#[macro_export]
macro_rules! empty {
  () => {
    $crate::parser::Expression::Empty
  };
}

/// Creates a `Char` expression, which matches with the single given character.
///
/// ## Example
/// ```rust
/// use ppeg_core::c;
///
/// let expr = c!("a");
/// ```
#[macro_export]
macro_rules! c {
  ($expr:expr) => {
    $crate::parser::Expression::Char($expr)
  };
}

/// Creates a `Sequence` expression from a list of expressions.
///
/// ## Example
/// ```rust
/// use ppeg_core::{seq, c};
///
/// let expr = seq!(c!("a"), c!("b"), c!("c"));
/// ```
#[macro_export]
macro_rules! seq {
  ( $( $expr:expr ),* ) => {
    $crate::parser::Expression::Sequence(vec![ $( $expr ),* ])
  };
}

/// Creates a `Choice` expression from a list of expressions.
///
/// ## Example
/// ```rust
/// use ppeg_core::{or, c};
///
/// let expr = or!(c!("a"), c!("b"), c!("c"));
/// ```
#[macro_export]
macro_rules! or {
  ( $( $expr:expr ),* ) => {
    $crate::parser::Expression::Choice(vec![ $( $expr ),* ])
  };
}

/// Creates a `ZeroOrMore` expression from a given expression.
///
/// ## Example
/// ```rust
/// use ppeg_core::{zero_or_more, c};
///
/// let expr = zero_or_more!(c!("a"));
/// ```
#[macro_export]
macro_rules! zero_or_more {
  ( $expr:expr ) => {
    $crate::parser::Expression::ZeroOrMore(Box::new($expr))
  };
}

/// Creates a `OneOrMore` expression from a given expression.
///
/// ## Example
/// ```rust
/// use ppeg_core::{one_or_more, c};
///
/// let expr = one_or_more!(c!("a"));
/// ```
#[macro_export]
macro_rules! one_or_more {
  ( $expr:expr ) => {
    $crate::parser::Expression::OneOrMore(Box::new($expr))
  };
}

/// Creates a `NamedRule` expression from a given rule name.
///
/// ## Example
/// ```rust
/// use ppeg_core::expr;
///
/// let expr = expr!("rule_x");
/// ```
#[macro_export]
macro_rules! expr {
  ( $expr:expr ) => {
    $crate::parser::Expression::NamedRule($expr)
  };
}

/// Creates a `Rule` from a given name and expression.
///
/// ## Example
/// ```rust
/// use ppeg_core::{rule, c, seq};
///
/// let a_and_b = rule!("my_rule" => seq!(c!("a"), c!("b")));
/// ```
#[macro_export]
macro_rules! rule {
  ( $name:expr => $expr:expr ) => {
    $crate::parser::Rule {
      name: $name,
      expression: $expr,
    }
  };
}

/// Creates a `Grammar` from a list of rules.
///
/// ## Example
/// ```rust
/// use ppeg_core::{grammar, rule, c, seq};
///
/// let peg = grammar!(
///  rule!("a_and_b" => seq!(c!("a"), c!("b"))),
///  rule!("c" => c!("c"))
/// );
/// ```
#[macro_export]
macro_rules! grammar {
  ( $( $rule:expr ),* $(,)? ) => {
    $crate::parser::Grammar::default()
      $( .with($rule) )*
  };
}

/// Creates a `Parser` from a given `Grammar`.
///
/// ## Example
/// ```rust
/// use ppeg_core::{grammar, rule, c, seq, parser};
///
/// let peg = grammar!(
///   rule!("a_and_b" => seq!(c!("a"), c!("b"))),
///   rule!("c" => c!("c"))
/// );
///
/// let parser = parser!(peg);
/// ```
#[macro_export]
macro_rules! parser {
  ( $grammar:expr ) => {
    $crate::parser::Parser::new($grammar)
  };
}

/// Creates a `Parse` result from a given grammar, input and start rule.
///
/// ## Example
/// ```rust
/// use ppeg_core::{grammar, rule, c, seq, parse};
///
/// let peg = grammar!(
///   rule!("a_and_b" => seq!(c!("a"), c!("b"))),
///   rule!("c" => c!("c"))
/// );
///
/// let result = parse!(peg, &mut "ab", "a_and_b");
/// ```
#[macro_export]
macro_rules! parse {
  ( $grammar:expr, $input:expr, $start_rule:expr ) => {{
    let mut parser = $crate::parser::Parser::new($grammar);
    parser.parse($input, $start_rule)
  }};
}

#[cfg(test)]
mod tests {
  use crate::parser::{Expression, Grammar, Parser, Rule};

  #[test]
  fn test_digit_macro() {
    let expr = digit!();

    assert_eq!(
      expr,
      Expression::Choice(vec![
        Expression::Char("0"),
        Expression::Char("1"),
        Expression::Char("2"),
        Expression::Char("3"),
        Expression::Char("4"),
        Expression::Char("5"),
        Expression::Char("6"),
        Expression::Char("7"),
        Expression::Char("8"),
        Expression::Char("9"),
      ])
    );
  }

  #[test]
  fn test_letter_macro() {
    let expr = letter!();

    assert_eq!(
      expr,
      Expression::Choice(vec![
        Expression::Char("a"),
        Expression::Char("b"),
        Expression::Char("c"),
        Expression::Char("d"),
        Expression::Char("e"),
        Expression::Char("f"),
        Expression::Char("g"),
        Expression::Char("h"),
        Expression::Char("i"),
        Expression::Char("j"),
        Expression::Char("k"),
        Expression::Char("l"),
        Expression::Char("m"),
        Expression::Char("n"),
        Expression::Char("o"),
        Expression::Char("p"),
        Expression::Char("q"),
        Expression::Char("r"),
        Expression::Char("s"),
        Expression::Char("t"),
        Expression::Char("u"),
        Expression::Char("v"),
        Expression::Char("w"),
        Expression::Char("x"),
        Expression::Char("y"),
        Expression::Char("z"),
        Expression::Char("A"),
        Expression::Char("B"),
        Expression::Char("C"),
        Expression::Char("D"),
        Expression::Char("E"),
        Expression::Char("F"),
        Expression::Char("G"),
        Expression::Char("H"),
        Expression::Char("I"),
        Expression::Char("J"),
        Expression::Char("K"),
        Expression::Char("L"),
        Expression::Char("M"),
        Expression::Char("N"),
        Expression::Char("O"),
        Expression::Char("P"),
        Expression::Char("Q"),
        Expression::Char("R"),
        Expression::Char("S"),
        Expression::Char("T"),
        Expression::Char("U"),
        Expression::Char("V"),
        Expression::Char("W"),
        Expression::Char("X"),
        Expression::Char("Y"),
        Expression::Char("Z")
      ])
    )
  }

  #[test]
  fn test_empty_macro() {
    let expr = empty!();

    assert_eq!(expr, Expression::Empty);
  }

  #[test]
  fn test_char_macro() {
    let expr = c!("a");

    assert_eq!(expr, Expression::Char("a"));
  }

  #[test]
  fn test_sequence_macro() {
    let expr = seq!(c!("a"), c!("b"), c!("c"));

    assert_eq!(
      expr,
      Expression::Sequence(vec![
        Expression::Char("a"),
        Expression::Char("b"),
        Expression::Char("c")
      ])
    );
  }

  #[test]
  fn test_choice_macro() {
    let expr = or!(c!("a"), c!("b"), c!("c"));

    assert_eq!(
      expr,
      Expression::Choice(vec![
        Expression::Char("a"),
        Expression::Char("b"),
        Expression::Char("c")
      ])
    );
  }

  #[test]
  fn test_zero_or_more_macro() {
    let expr = zero_or_more!(c!("a"));

    assert_eq!(expr, Expression::ZeroOrMore(Box::new(Expression::Char("a"))));
  }

  #[test]
  fn test_one_or_more_macro() {
    let expr = one_or_more!(c!("a"));

    assert_eq!(expr, Expression::OneOrMore(Box::new(Expression::Char("a"))));
  }

  #[test]
  fn test_expr_macro() {
    let expr = expr!("rule_x");

    assert_eq!(expr, Expression::NamedRule("rule_x"));
  }

  #[test]
  fn test_rule_macro() {
    let rule = rule!("a_and_b" => seq!(c!("a"), c!("b")));

    assert_eq!(
      rule,
      Rule {
        name: "a_and_b",
        expression: Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")])
      }
    );
  }

  #[test]
  fn test_grammar_macro() {
    let grammar = grammar!(rule!("a_and_b" => seq!(c!("a"), c!("b"))), rule!("c" => c!("c")));

    let expected_grammar = {
      let mut g = Grammar::default();
      g.insert(rule!("a_and_b" => seq!(c!("a"), c!("b"))));
      g.insert(rule!("c" => c!("c")));
      g
    };

    assert_eq!(grammar, expected_grammar);
  }

  #[test]
  fn test_parser_macro() {
    let grammar = grammar!(rule!("a_and_b" => seq!(c!("a"), c!("b"))), rule!("c" => c!("c")));
    let parser = parser!(grammar.clone());

    let expected_parser = Parser::new(grammar);

    assert_eq!(parser, expected_parser);
  }

  #[test]
  fn test_parse_macro() {
    let grammar = grammar!(rule!("a_and_b" => seq!(c!("a"), c!("b"))), rule!("c" => c!("c")));
    let result = parse!(grammar.clone(), &mut "ab", "a_and_b");

    let expected_result = {
      let mut parser = Parser::new(grammar);
      parser.parse(&mut "ab", "a_and_b")
    };

    assert_eq!(result, expected_result);
  }
}
