use criterion::{Criterion, criterion_group};
use ppeg_core::{c, expr, grammar, or, parse, rule, seq};

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

fn bench_indirect_left_recursive(c: &mut Criterion) {
  c.bench_function("indirect_left_recursive", |b| b.iter(|| indirect_left_recursive()));
}

criterion_group!(indirect_left_recursive_group, bench_indirect_left_recursive);
