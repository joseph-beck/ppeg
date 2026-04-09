//! Macros support the creation of PEGs using macros for a more concise and readable syntax.
//! This helps simplify the definitions of grammars, rule and expressions.
//! There are also shortcuts for creating parsers and parsing input using the defined grammars.
//!
//! ## Example
//! ```rust
//! use ppeg_core::{grammar, rule, c, seq, meta, parse};
//!
//! // Create a rule that matches the sequence "a" followed by "b".
//! let a_and_b = rule!("a_and_b" => seq!(c!("a"), c!("b")));
//!
//! // Define a grammar with the rules we just created and another rule that matches "c".
//! let peg = grammar!(
//!   // Insert a predefined rule.
//!   a_and_b,
//!   // Or define a rule inline.
//!   rule!("c" => c!("c"))
//! );
//!
//! // Or define with a meta grammar string.
//! let peg = meta!(r#"
//!   a_and_b := { 'a', 'b' }
//!   c := { 'c' }
//! "#);
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
    $crate::parser::expression::Expression::Choice(vec![
      $crate::parser::expression::Expression::Char("0"),
      $crate::parser::expression::Expression::Char("1"),
      $crate::parser::expression::Expression::Char("2"),
      $crate::parser::expression::Expression::Char("3"),
      $crate::parser::expression::Expression::Char("4"),
      $crate::parser::expression::Expression::Char("5"),
      $crate::parser::expression::Expression::Char("6"),
      $crate::parser::expression::Expression::Char("7"),
      $crate::parser::expression::Expression::Char("8"),
      $crate::parser::expression::Expression::Char("9"),
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
    $crate::parser::expression::Expression::Choice(vec![
      $crate::parser::expression::Expression::Char("a"),
      $crate::parser::expression::Expression::Char("b"),
      $crate::parser::expression::Expression::Char("c"),
      $crate::parser::expression::Expression::Char("d"),
      $crate::parser::expression::Expression::Char("e"),
      $crate::parser::expression::Expression::Char("f"),
      $crate::parser::expression::Expression::Char("g"),
      $crate::parser::expression::Expression::Char("h"),
      $crate::parser::expression::Expression::Char("i"),
      $crate::parser::expression::Expression::Char("j"),
      $crate::parser::expression::Expression::Char("k"),
      $crate::parser::expression::Expression::Char("l"),
      $crate::parser::expression::Expression::Char("m"),
      $crate::parser::expression::Expression::Char("n"),
      $crate::parser::expression::Expression::Char("o"),
      $crate::parser::expression::Expression::Char("p"),
      $crate::parser::expression::Expression::Char("q"),
      $crate::parser::expression::Expression::Char("r"),
      $crate::parser::expression::Expression::Char("s"),
      $crate::parser::expression::Expression::Char("t"),
      $crate::parser::expression::Expression::Char("u"),
      $crate::parser::expression::Expression::Char("v"),
      $crate::parser::expression::Expression::Char("w"),
      $crate::parser::expression::Expression::Char("x"),
      $crate::parser::expression::Expression::Char("y"),
      $crate::parser::expression::Expression::Char("z"),
      $crate::parser::expression::Expression::Char("A"),
      $crate::parser::expression::Expression::Char("B"),
      $crate::parser::expression::Expression::Char("C"),
      $crate::parser::expression::Expression::Char("D"),
      $crate::parser::expression::Expression::Char("E"),
      $crate::parser::expression::Expression::Char("F"),
      $crate::parser::expression::Expression::Char("G"),
      $crate::parser::expression::Expression::Char("H"),
      $crate::parser::expression::Expression::Char("I"),
      $crate::parser::expression::Expression::Char("J"),
      $crate::parser::expression::Expression::Char("K"),
      $crate::parser::expression::Expression::Char("L"),
      $crate::parser::expression::Expression::Char("M"),
      $crate::parser::expression::Expression::Char("N"),
      $crate::parser::expression::Expression::Char("O"),
      $crate::parser::expression::Expression::Char("P"),
      $crate::parser::expression::Expression::Char("Q"),
      $crate::parser::expression::Expression::Char("R"),
      $crate::parser::expression::Expression::Char("S"),
      $crate::parser::expression::Expression::Char("T"),
      $crate::parser::expression::Expression::Char("U"),
      $crate::parser::expression::Expression::Char("V"),
      $crate::parser::expression::Expression::Char("W"),
      $crate::parser::expression::Expression::Char("X"),
      $crate::parser::expression::Expression::Char("Y"),
      $crate::parser::expression::Expression::Char("Z"),
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
    $crate::parser::expression::Expression::Empty
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
    $crate::parser::expression::Expression::Char($expr)
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
    $crate::parser::expression::Expression::Sequence(vec![ $( $expr ),* ])
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
    $crate::parser::expression::Expression::Choice(vec![ $( $expr ),* ])
  };
}

/// Creates a `Not` expression from a given expression.
///
/// ## Example
/// ```rust
/// use ppeg_core::{not, c};
///
/// let expr = not!(c!("a"));
/// ```
#[macro_export]
macro_rules! not {
  ( $expr:expr ) => {
    $crate::parser::expression::Expression::Not(Box::new($expr))
  };
}

/// Creates an `Optional` expression from a given expression.
///
/// ## Example
/// ```rust
/// use ppeg_core::{opt, c};
///
/// let expr = opt!(c!("a"));
/// ```
#[macro_export]
macro_rules! opt {
  ( $expr:expr ) => {
    $crate::parser::expression::Expression::Optional(Box::new($expr))
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
    $crate::parser::expression::Expression::ZeroOrMore(Box::new($expr))
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
    $crate::parser::expression::Expression::OneOrMore(Box::new($expr))
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
    $crate::parser::expression::Expression::NamedRule($expr)
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
    $crate::parser::rule::Rule::new($name, $expr)
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
    $crate::parser::grammar::Grammar::default()
      $( .with($rule) )*
  };
}

/// Creates a `Grammar` from a meta grammar definition string.
///
/// ## Example
/// ```rust
/// use ppeg_core::meta;
///
/// let peg = meta!(r#"
///   c := { 'a' }
/// "#);
/// ```
#[macro_export]
macro_rules! meta {
  ( $meta:expr ) => {{
    let parser = $crate::meta::parser::Meta::new();
    parser.generate($meta).unwrap()
  }};
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
    $crate::parser::peg::Parser::new($grammar)
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
    let mut parser = $crate::parser::peg::Parser::new($grammar);
    parser.parse($input, $start_rule)
  }};
}

#[cfg(test)]
mod tests {
  use crate::parser::{expression::Expression, grammar::Grammar, peg::Parser, rule::Rule};

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
  fn test_not_macro() {
    let expr = not!(c!("a"));

    assert_eq!(expr, Expression::Not(Box::new(Expression::Char("a"))));
  }

  #[test]
  fn test_optional_macro() {
    let expr = opt!(c!("a"));

    assert_eq!(expr, Expression::Optional(Box::new(Expression::Char("a"))));
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
      Rule::new(
        "a_and_b",
        Expression::Sequence(vec![Expression::Char("a"), Expression::Char("b")])
      )
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
  fn test_meta_macro() {
    let grammar = meta!(
      r#"
        a_and_b := { 'a', 'b' }
        c := { 'c' }
    "#
    );

    let expected_grammar = Grammar::default()
      .with(rule!("a_and_b" => seq!(c!("a"), c!("b"))))
      .with(rule!("c" => c!("c")));

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
