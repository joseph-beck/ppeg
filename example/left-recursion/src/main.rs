use ppeg_core::parser::{expression::Expression, grammar::Grammar, peg::Parser, rule::Rule};

// This example contains an indirect left-recursive grammar.
fn main() {
  let rule_num = Rule::new(
    "rule_num",
    Expression::OneOrMore(Box::new(Expression::Choice(vec![
      Expression::Char("1"),
      Expression::Char("2"),
    ]))),
  );
  let rule_x = Rule::new("rule_x", Expression::NamedRule("rule_expr"));
  let rule_expr = Rule::new(
    "rule_expr",
    Expression::Choice(vec![
      Expression::Sequence(vec![
        Expression::NamedRule("rule_x"),
        Expression::Char("+"),
        Expression::NamedRule("rule_num"),
      ]),
      Expression::NamedRule("rule_num"),
    ]),
  );

  let grammar = Grammar::default().with(rule_num).with(rule_x).with(rule_expr);

  let mut parser = Parser::new(grammar);
  let (remaining, cst) = parser.parse(&mut "1+2", "rule_expr").unwrap();

  println!("{}", remaining);
  if let Some(cst) = cst {
    cst.pretty_print(Some(0));
  }
}
