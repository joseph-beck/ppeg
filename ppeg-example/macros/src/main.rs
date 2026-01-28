use ppeg_core::{c, grammar, one_or_more, or, parser::Parser, rule};

fn main() {
  let digit = or!(
    c!("0"),
    c!("1"),
    c!("2"),
    c!("3"),
    c!("4"),
    c!("5"),
    c!("6"),
    c!("7"),
    c!("8"),
    c!("9")
  );

  let number = one_or_more!(digit);

  let grammar = grammar!(rule!("number" => number));

  let mut parser = Parser::new(grammar);
  let (remaining, cst) = parser.parse(&mut "12345", "number").unwrap();

  println!("{:?}", remaining);

  if let Some(cst) = cst {
    println!("{:#?}", cst);
  }
}
