use criterion::{Criterion, criterion_group, criterion_main};
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

fn bench_direct_left_recursive(c: &mut Criterion) {
  c.bench_function("direct_left_recursive", |b| b.iter(|| direct_left_recursive()));
}

fn bench_indirect_left_recursive(c: &mut Criterion) {
  c.bench_function("indirect_left_recursive", |b| b.iter(|| indirect_left_recursive()));
}

criterion_group!(benches, bench_direct_left_recursive, bench_indirect_left_recursive);

criterion_main!(benches);
