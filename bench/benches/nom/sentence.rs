use criterion::{Criterion, criterion_group};

fn sentence() {
  let _ = 1;
}

fn bench_sentence(c: &mut Criterion) {
  c.bench_function("nom_sentence", |b| b.iter(|| sentence()));
}

criterion_group!(sentence_group, bench_sentence);
