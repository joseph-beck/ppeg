use criterion::{Criterion, criterion_group};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    letter = @{ 'a'..'z' | 'A'..'Z' }
    word = @{ letter+ }
"#]
pub struct WordParser;

fn word() {
  let _ = WordParser::parse(Rule::word, "helloworld");
}

fn bench_word(c: &mut Criterion) {
  c.bench_function("pest_word", |b| b.iter(|| word()));
}

criterion_group!(word_group, bench_word);
