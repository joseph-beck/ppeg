use criterion::{Criterion, criterion_group};
use nom::{IResult, character::complete::digit1};

fn number(input: &str) -> IResult<&str, &str> {
  digit1(input)
}

fn parse_number() {
  let _ = number("100000000");
}

fn bench_number(c: &mut Criterion) {
  c.bench_function("nom_number", |b| b.iter(|| parse_number()));
}

criterion_group!(number_group, bench_number);
