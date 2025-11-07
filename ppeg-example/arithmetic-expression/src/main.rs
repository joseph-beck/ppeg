use ppeg_core::parser::{Grammar, Parser};

fn main() {
  let grammar = Grammar::new();
  let parser = Parser::new("1+2*3", 0, grammar);
  let cst = parser.parse();

  match cst {
    Ok(cst) => cst.pretty_print(None),
    Err(err) => eprintln!("Error parsing input: {:?}", err),
  }
}
