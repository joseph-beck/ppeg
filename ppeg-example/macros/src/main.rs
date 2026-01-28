use ppeg_core::{c, grammar, one_or_more, or, parse, rule};

fn main() {
  let (remaining, cst) = parse!(
    grammar!(rule!("number" => one_or_more!(or!(
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
    )))),
    &mut "12345",
    "number"
  )
  .unwrap();

  println!("{:?}", remaining);

  if let Some(cst) = cst {
    println!("{:#?}", cst);
  }
}
