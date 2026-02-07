use criterion::{Criterion, criterion_group};
use ppeg_core::{c, expr, grammar, or, parse, rule, seq};

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

fn bench_direct_left_recursive(c: &mut Criterion) {
  c.bench_function("direct_left_recursive", |b| b.iter(|| direct_left_recursive()));
}

criterion_group!(direct_left_recursive_group, bench_direct_left_recursive);
