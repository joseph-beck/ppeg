use criterion::{Criterion, criterion_group};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
  digit = @{ '0'..'9' }
  number = @{ digit+ }
"#]
pub struct ArithmeticExpressionParser;

fn arithmetic_expression() {
  let _ = ArithmeticExpressionParser::parse(Rule::number, "100000000");
}

fn bench_arithmetic_expression(c: &mut Criterion) {
  c.bench_function("pest_arithmetic_expression", |b| b.iter(|| arithmetic_expression()));
}

criterion_group!(arithmetic_expression_group, bench_arithmetic_expression);
