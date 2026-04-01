use criterion::{Criterion, criterion_group};
use ppeg_core::{grammar, letter, one_or_more, parse, rule};

fn word() {
  let word = rule!(
    "word" => one_or_more!(letter!())
  );

  let grammar = grammar!(word);

  let _ = parse!(grammar, &mut "helloworld", "word");
}

fn bench_word(c: &mut Criterion) {
  c.bench_function("ppeg_word", |b| b.iter(|| word()));
}

criterion_group!(word_group, bench_word);
