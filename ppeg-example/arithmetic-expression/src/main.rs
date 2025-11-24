use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

fn main() {
  let digit = Expression::OneOrMore(Box::new(Expression::Sequence(vec![
    Expression::Char("1"),
    Expression::Char("2"),
    Expression::Char("3"),
    Expression::Char("4"),
    Expression::Char("5"),
    Expression::Char("6"),
    Expression::Char("7"),
    Expression::Char("8"),
    Expression::Char("9"),
    Expression::Char("0"),
  ])));
  let factor = Rule::new("factor", Expression::ZeroOrMore(Box::new(digit.clone())));
  let term = Rule::new(
    "term",
    Expression::Choice(vec![
      Expression::NamedRule("factor"),
      Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
        Expression::NamedRule("factor"),
        Expression::Char("*"),
        Expression::NamedRule("term"),
      ]))),
    ]),
  );
  let arithmetic_expression = Rule::new(
    "arithmetic_expression",
    Expression::Choice(vec![
      Expression::NamedRule("term"),
      Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
        Expression::NamedRule("term"),
        Expression::Char("*"),
        Expression::NamedRule("arithmetic_expression"),
      ]))),
    ]),
  );

  let mut grammar = Grammar::new();
  grammar.insert(factor);
  grammar.insert(term);
  grammar.insert(arithmetic_expression);

  let mut parser = Parser::new(grammar);
  let (_, cst) = parser.parse(&mut "1+2*3", "arithmetic_expression").unwrap();

  match cst {
    Some(c) => println!("Parsed successfully: {:?}", c.pretty_print(Some(0))),
    None => println!("Parsed successfully with no CST."),
  }
}
