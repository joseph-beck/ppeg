use criterion::{Criterion, criterion_group};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    letter = @{ 'a'..'z' | 'A'..'Z' }
    word = @{ letter+ }
    punctuation = @{ "." | "," }
    sentence = @{ word ~ ((" " ~ word | punctuation)*) }
"#]
pub struct SentenceParser;

fn sentence() {
  let _ = SentenceParser::parse(
    Rule::sentence,
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at tellus eros. Donec semper tempus facilisis. Mauris elementum non quam in aliquet. Nulla massa ligula, vehicula in aliquam varius, venenatis ut quam. Nam feugiat enim nec euismod pretium. Proin accumsan iaculis dolor id dignissim. Sed mollis sit amet mi a tincidunt. Proin at felis eget nisl cursus venenatis molestie nec augue. Morbi ut mauris nec risus gravida fringilla vehicula vitae orci. Pellentesque accumsan et mauris id ultricies. Donec tincidunt lorem tortor, id imperdiet lorem imperdiet ut. Pellentesque non quam at sem suscipit vestibulum eget non risus. Mauris auctor volutpat leo, vel interdum velit posuere sed. Donec ultricies justo non leo vestibulum, eu vehicula arcu commodo.",
  );
}

fn bench_sentence(c: &mut Criterion) {
  c.bench_function("pest_sentence", |b| b.iter(|| sentence()));
}

criterion_group!(sentence_group, bench_sentence);
