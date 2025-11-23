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
  let factor = Expression::ZeroOrMore(Box::new(digit.clone()));
  let term = Expression::Choice(vec![
    factor.clone(),
    Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
      factor.clone(),
      Expression::Char("*"),
      factor.clone(),
    ]))),
  ]);
  let arithmetic_expression = Expression::Choice(vec![
    term.clone(),
    Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
      term.clone(),
      Expression::Char("+"),
      term.clone(),
    ]))),
  ]);

  let mut grammar = Grammar::new();
  grammar.insert(Rule::new("expr", arithmetic_expression));

  let mut parser = Parser::new(grammar);
  let (_, cst) = parser.parse(&mut "1+2*3", "expr").unwrap();

  match cst {
    Some(c) => println!("Parsed successfully: {:?}", c.pretty_print(Some(0))),
    None => println!("Parsed successfully with no CST."),
  }
}
