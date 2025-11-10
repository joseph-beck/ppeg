use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

fn main() {
  let mut grammar = Grammar::new();
  grammar.insert(Rule::new("WHITESPACE", " ", Expression::Empty));
  grammar.insert(Rule::new("ONE", "1", Expression::OneOrMore));
  grammar.insert(Rule::new("TWO", "2", Expression::OneOrMore));
  grammar.insert(Rule::new("THREE", "3", Expression::OneOrMore));
  grammar.insert(Rule::new("L_BRACKET", "(", Expression::OneOrOne));
  grammar.insert(Rule::new("R_BRACKET", ")", Expression::OneOrOne));
  grammar.insert(Rule::new("STAR", "*", Expression::OneOrOne));
  grammar.insert(Rule::new("PLUS", "+", Expression::OneOrOne));

  let mut parser = Parser::new("1+2*3", 0, grammar);
  let cst = parser.parse();

  match cst {
    Ok(cst) => cst.pretty_print(None),
    Err(err) => eprintln!("Error parsing input: {:?}", err),
  }
}
