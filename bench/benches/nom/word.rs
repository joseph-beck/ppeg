use criterion::{Criterion, criterion_group};
use nom::{IResult, bytes::complete::take_while1};

fn is_letter(c: char) -> bool {
  c.is_ascii_alphabetic()
}

fn word(input: &str) -> IResult<&str, &str> {
  take_while1(is_letter)(input)
}

fn parse_word() {
  let _ = word("helloworld");
}

fn bench_word(c: &mut Criterion) {
  c.bench_function("nom_word", |b| b.iter(|| parse_word()));
}

criterion_group!(word_group, bench_word);
