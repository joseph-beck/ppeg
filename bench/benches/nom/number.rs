use criterion::{Criterion, criterion_group};

fn number() {
  let _ = 1;
}

fn bench_number(c: &mut Criterion) {
  c.bench_function("nom_number", |b| b.iter(|| number()));
}

criterion_group!(number_group, bench_number);
