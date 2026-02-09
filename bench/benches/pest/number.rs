use criterion::{Criterion, criterion_group};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
  digit = @{ '0'..'9' }
  number = @{ digit+ }
"#]
pub struct NumberParser;

fn number() {
  let _ = NumberParser::parse(Rule::number, "100000000");
}

fn bench_number(c: &mut Criterion) {
  c.bench_function("pest_number", |b| b.iter(|| number()));
}

criterion_group!(number_group, bench_number);
