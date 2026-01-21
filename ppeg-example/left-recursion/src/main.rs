use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

// This example contains an indirect left-recursive grammar.
fn main() {
  let rule_num = Rule::new(
    "rule_num",
    Expression::OneOrMore(Box::new(Expression::Choice(vec![Expression::Char("1")]))),
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

  let mut grammar = Grammar::new();
  grammar.insert(rule_num);
  grammar.insert(rule_x);
  grammar.insert(rule_expr);

  let mut parser = Parser::new(grammar);
  let (remaining, cst) = parser.parse(&mut "1+1", "rule_expr").unwrap();

  println!("{}", remaining);
  if let Some(cst) = cst {
    cst.pretty_print(Some(0));
  }
}
