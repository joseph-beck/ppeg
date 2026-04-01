use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

fn main() {
  let digit = Expression::Choice(vec![
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
  ]);

  let number = Rule::new("number", Expression::OneOrMore(Box::new(digit.clone())));

  let factor = Rule::new(
    "factor",
    Expression::Choice(vec![
      Expression::NamedRule("number"),
      Expression::Sequence(vec![
        Expression::Char("("),
        Expression::NamedRule("arithmetic_expression"),
        Expression::Char(")"),
      ]),
    ]),
  );

  let term = Rule::new(
    "term",
    Expression::Sequence(vec![
      Expression::NamedRule("factor"),
      Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
        Expression::Char("*"),
        Expression::NamedRule("factor"),
      ]))),
    ]),
  );

  let arithmetic_expression = Rule::new(
    "arithmetic_expression",
    Expression::Sequence(vec![
      Expression::NamedRule("term"),
      Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
        Expression::Char("+"),
        Expression::NamedRule("term"),
      ]))),
    ]),
  );

  let grammar = Grammar::default()
    .with(number)
    .with(factor)
    .with(term)
    .with(arithmetic_expression);

  let mut parser = Parser::new(grammar);
  let (_, cst) = parser.parse(&mut "1+(2*3)", "arithmetic_expression").unwrap();

  match cst {
    Some(c) => {
      c.pretty_print(Some(0));
    }
    None => println!("cst was empty"),
  }
}
