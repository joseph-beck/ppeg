use criterion::{Criterion, criterion_group};

fn arithmetic_expression() {
  let _ = 1;
}

fn bench_arithmetic_expression(c: &mut Criterion) {
  c.bench_function("nom_arithmetic_expression", |b| b.iter(|| arithmetic_expression()));
}

criterion_group!(arithmetic_expression_group, bench_arithmetic_expression);
