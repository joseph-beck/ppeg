use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

// {'a':1}
fn main() {
  let character = Expression::Choice(vec![Expression::Char("a"), Expression::Char("1")]);
  let key = Expression::Sequence(vec![
    Expression::Char("'"),
    Expression::ZeroOrMore(Box::new(character.clone())),
    Expression::Char("'"),
  ]);
  let separator = Expression::Char(":");
  let data = Expression::ZeroOrMore(Box::new(character.clone()));

  let data = Rule::new(
    "data",
    Expression::Sequence(vec![
      Expression::Char("{"),
      key.clone(),
      separator.clone(),
      data.clone(),
      Expression::Char("}"),
    ]),
  );

  let mut grammar = Grammar::new();
  grammar.insert(data);

  let mut parser = Parser::new(grammar);
  let (_, cst) = parser.parse(&mut "{'a1':1a}", "data").unwrap();
  match cst {
    Some(c) => c.pretty_print(Some(0)),
    None => println!("no cst output"),
  }
}
