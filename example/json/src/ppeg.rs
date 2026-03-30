use ppeg_core::parser::{expression::Expression, grammar::Grammar, peg::Parser, rule::Rule};

pub fn parse() {
  let space = Expression::Choice(vec![
    Expression::Char(" "),
    Expression::Char("\n"),
    Expression::Char("\t"),
    Expression::Char("\r"),
  ]);
  let ws = Expression::ZeroOrMore(Box::new(space));

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

  let letter = Expression::Choice(vec![
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
  ]);
  let string_inner = Expression::OneOrMore(Box::new(letter.clone()));
  let string_expr = Expression::Sequence(vec![Expression::Char("\""), string_inner, Expression::Char("\"")]);
  let string = Rule::new("string", string_expr);

  let value_expr = Expression::Choice(vec![
    Expression::NamedRule("string"),
    Expression::NamedRule("number"),
    Expression::NamedRule("object"),
    Expression::NamedRule("array"),
    Expression::Char("true"),
    Expression::Char("false"),
    Expression::Char("null"),
  ]);
  let value = Rule::new("value", value_expr);

  let pair = Rule::new(
    "pair",
    Expression::Sequence(vec![
      Expression::NamedRule("string"),
      ws.clone(),
      Expression::Char(":"),
      ws.clone(),
      Expression::NamedRule("value"),
    ]),
  );

  let members_expr = Expression::Sequence(vec![
    Expression::NamedRule("pair"),
    Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
      ws.clone(),
      Expression::Char(","),
      ws.clone(),
      Expression::NamedRule("pair"),
    ]))),
  ]);
  let members = Rule::new("members", members_expr);

  let object_expr = Expression::Sequence(vec![
    Expression::Char("{"),
    ws.clone(),
    Expression::Choice(vec![Expression::NamedRule("members"), Expression::Empty]),
    ws.clone(),
    Expression::Char("}"),
  ]);
  let object = Rule::new("object", object_expr);

  let elements_expr = Expression::Sequence(vec![
    Expression::NamedRule("value"),
    Expression::ZeroOrMore(Box::new(Expression::Sequence(vec![
      ws.clone(),
      Expression::Char(","),
      ws.clone(),
      Expression::NamedRule("value"),
    ]))),
  ]);
  let elements = Rule::new("elements", elements_expr);

  let array_expr = Expression::Sequence(vec![
    Expression::Char("["),
    ws.clone(),
    Expression::Choice(vec![Expression::NamedRule("elements"), Expression::Empty]),
    ws.clone(),
    Expression::Char("]"),
  ]);
  let array = Rule::new("array", array_expr);

  let mut grammar = Grammar::default();
  grammar.insert(number);
  grammar.insert(string);
  grammar.insert(pair);
  grammar.insert(members);
  grammar.insert(object);
  grammar.insert(elements);
  grammar.insert(array);
  grammar.insert(value);

  let mut parser = Parser::new(grammar);
  let mut input = r#"{ "a" : [1, 2, 3], "b": "xyz", "c": true }"#;
  let (_rem, cst) = parser.parse(&mut input, "object").unwrap();

  match cst {
    Some(c) => {
      println!("Parsed successfully:");
      c.pretty_print(Some(0));
    }
    None => println!("Parse succeeded but CST is empty"),
  }
}
