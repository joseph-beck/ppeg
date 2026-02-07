use criterion::{Criterion, criterion_group, criterion_main};
use ppeg_core::{c, digit, expr, grammar, one_or_more, or, parse, rule, seq, zero_or_more};

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
      expr!("number")
    )
  );

  let grammar = grammar!(number, expr);

  let _ = parse!(grammar, &mut "1+2+3", "expr");
}

fn indirect_left_recursive() {
  let number = rule!(
    "number" => or!(c!("1"), c!("2"), c!("3"))
  );

  let x = rule!("x" => expr!("expr"));

  let expr = rule!(
    "expr" => or!(
      seq!(
        expr!("x"),
        c!("+"),
        expr!("number")
      ),
      expr!("number")
    )
  );

  let grammar = grammar!(number, x, expr);

  let _ = parse!(grammar, &mut "1+2+3", "expr");
}

fn arithmetic_expression() {
  let number = rule!("number" => one_or_more!(digit!()));

  let factor = rule!(
    "factor" =>
    or!(expr!("number"), seq!(c!("("), expr!("arithmetic_expression"), c!(")")))
  );

  let term = rule!(
    "term" =>
    seq!(
      expr!("factor"),
      zero_or_more!(seq!(c!("*"), expr!("factor")))
    )
  );

  let arithmetic_expression = rule!(
    "arithmetic_expression" =>
    seq!(
      expr!("term"),
      zero_or_more!(seq!(c!("+"), expr!("term")))
    )
  );

  let grammar = grammar!(number, factor, term, arithmetic_expression);

  let _ = parse!(grammar, &mut "1+(2*3)", "arithmetic_expression");
}

fn bench_direct_left_recursive(c: &mut Criterion) {
  c.bench_function("direct_left_recursive", |b| b.iter(|| direct_left_recursive()));
}

fn bench_indirect_left_recursive(c: &mut Criterion) {
  c.bench_function("indirect_left_recursive", |b| b.iter(|| indirect_left_recursive()));
}

fn bench_arithmetic_expression(c: &mut Criterion) {
  c.bench_function("arithmetic_expression", |b| b.iter(|| arithmetic_expression()));
}

criterion_group!(
  benches,
  bench_direct_left_recursive,
  bench_indirect_left_recursive,
  bench_arithmetic_expression
);

criterion_main!(benches);
