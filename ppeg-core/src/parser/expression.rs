#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
  /// Matches what is an "empty" expression.
  /// This means that it matches with what it is given,
  /// consumes the output but gives no output.
  /// Expression for `ε` symbol.
  Empty,
  /// Matches a single character.
  /// For example, `A` matches the character A.
  /// Expression for `C`.
  Char(&'a str),
  /// Matches a sequence of the given expressions.
  /// For example, `AB` matches A followed by B.
  /// Expression for `EE'`.
  Sequence(Vec<Expression<'a>>),
  /// Matches a or b occurrences of an expression.
  /// For example, `A|B` matches either A or B.
  /// Expression for `E|E'`. ?
  Choice(Vec<Expression<'a>>),
  /// Matches the expression if it does not match.
  /// For example, `!A` matches if A does not match.
  /// Expression for `!E`.
  Not(Box<Expression<'a>>),
  /// Matches the expression regardless, but may not always consume input.
  /// For example, `A?` matches A if it exists, but also matches if A does not exist.
  /// Expression for `E?`.
  Optional(Box<Expression<'a>>),
  /// Matches one or more occurrences of the expression.
  /// For example, `A` matches one or more occurrences of A.
  /// Expression for `EE*`.
  OneOrMore(Box<Expression<'a>>),
  /// Matches zero or more occurrences of the expression.
  /// For example, `A` matches zero or more occurrences of A.
  /// Expression for `E*`.
  ZeroOrMore(Box<Expression<'a>>),
  /// Matches a named rule.
  /// Expression for `N`.
  NamedRule(&'a str),
}
