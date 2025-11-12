use ppeg_core::parser::{Expression, Grammar, Parser, Rule};

fn main() {
  let mut grammar = Grammar::new();
  grammar.insert(Rule::new("WHITESPACE", vec![" "], Expression::Empty));
  grammar.insert(Rule::new(
    "DIGIT",
    vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    Expression::OneOrMore,
  ));
  grammar.insert(Rule::new("L_BRACKET", vec!["("], Expression::Choice));
  grammar.insert(Rule::new("R_BRACKET", vec![")"], Expression::Choice));
  grammar.insert(Rule::new("STAR", vec!["*"], Expression::Choice));
  grammar.insert(Rule::new("PLUS", vec!["+"], Expression::Choice));

  let mut parser = Parser::new("1+(2*3)", 0, grammar);
  let cst = parser.parse();

  match cst {
    Ok(cst) => cst.pretty_print(None),
    Err(err) => eprintln!("Error parsing input: {:?}", err),
  }
}
