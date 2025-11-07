use ppeg_core::parser::{Grammar, Parser, Rule};

fn main() {
  let mut grammar = Grammar::new();
  grammar.insert(Rule::new("1", ppeg_core::parser::Expression::OneOrMore));
  grammar.insert(Rule::new("2", ppeg_core::parser::Expression::OneOrMore));
  grammar.insert(Rule::new("3", ppeg_core::parser::Expression::OneOrMore));
  grammar.insert(Rule::new("*", ppeg_core::parser::Expression::OneOrOne));
  grammar.insert(Rule::new("+", ppeg_core::parser::Expression::OneOrOne));

  let mut parser = Parser::new("1+2*3", 0, grammar);
  let cst = parser.parse();

  match cst {
    Ok(cst) => cst.pretty_print(None),
    Err(err) => eprintln!("Error parsing input: {:?}", err),
  }
}
