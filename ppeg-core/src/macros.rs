//! Macros support the creation of PEGs using macros for a more concise and readable syntax.
//! This helps simplify the definitions of grammars, rule and expressions.
//!
//! ## Example
//! ```rust
//! use ppeg_core::{grammar, rule, c, seq};
//!
//! // Create a rule that matches the sequence "a" followed by "b".
//! let a_and_b = rule!("my_rule" => seq!(c!("a"), c!("b")));
//!
//! let peg = grammar!(
//!   // Insert a predefined rule.
//!   a_and_b,
//!   // Or define a rule inline.
//!   rule!("c" => c!("c"))
//! );
//! ```

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

#[cfg(test)]
mod tests {
  use crate::parser::{Expression, Grammar, Rule};

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
}
