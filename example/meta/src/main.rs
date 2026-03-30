use ppeg_core::{parse, prelude::*};

fn main() {
  let grammar = Meta::new()
    .generate(
      r#"
        char_c := { 'c' }

        // Comment about rule
        rule := {
          'a' | // Inline comment
          { char_c, 'b' } |
          { 'd' }+ |
          { '\n' } // Example using an escape character for a newline
        }
      "#,
    )
    .unwrap();

  println!("{:#?}", grammar);

  let (remaining, cst) = parse!(grammar, &mut "c", "char_c").unwrap();
  println!("{:#?}", remaining);
  println!("{:#?}", cst);
}
