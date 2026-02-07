use criterion::{Criterion, criterion_group};
use ppeg_core::{digit, grammar, one_or_more, parse, rule};

fn number() {
  let number = rule!(
    "number" => one_or_more!(digit!())
  );

  let grammar = grammar!(number);

  let _ = parse!(grammar, &mut "100000000", "number");
}

fn bench_number(c: &mut Criterion) {
  c.bench_function("number", |b| b.iter(|| number()));
}

criterion_group!(number_group, bench_number);
