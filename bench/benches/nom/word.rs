use criterion::{Criterion, criterion_group};

fn word() {
  let _ = 1;
}

fn bench_word(c: &mut Criterion) {
  c.bench_function("nom_word", |b| b.iter(|| word()));
}

criterion_group!(word_group, bench_word);
