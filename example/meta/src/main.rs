use ppeg_core::meta::Meta;

fn main() {
  let grammar = Meta::new()
    .generate(
      r#"
        char_c := { 'c' }

        // Comment about rule
        rule := {
          'a' | // Inline comment
          { char_c, 'b' } |
          { 'd' }+
        }
      "#,
    )
    .unwrap();

  println!("\n{:?}", grammar);
}
