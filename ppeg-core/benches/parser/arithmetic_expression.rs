use criterion::{Criterion, criterion_group};
use ppeg_core::{c, digit, expr, grammar, one_or_more, or, parse, rule, seq, zero_or_more};

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

fn bench_arithmetic_expression(c: &mut Criterion) {
  c.bench_function("arithmetic_expression", |b| b.iter(|| arithmetic_expression()));
}

criterion_group!(arithmetic_expression_group, bench_arithmetic_expression);
