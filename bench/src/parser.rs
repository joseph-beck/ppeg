use ppeg_core::{c, expr, grammar, or, parse, rule, seq};

extern crate test;

#[allow(dead_code)]
fn direct_left_recursive() {
  let number = rule!(
    "number" => or!(c!("1"), c!("2"), c!("3"))
  );

  let expr = rule!(
    "expr" => or!(
      seq!(
        expr!("expr"),
        c!("+"),
        expr!("number")
      ),
      expr!("expr")
    )
  );

  let grammar = grammar!(number, expr);

  let _ = parse!(grammar, &mut "1+2+3", "expr");
}

#[bench]
fn bench_parse_direct_left_recursive(b: &mut test::Bencher) {
  b.iter(|| direct_left_recursive());
}
