use criterion::{Criterion, criterion_group};
use ppeg_core::{c, expr, grammar, letter, one_or_more, or, parse, rule, seq};

fn sentence() {
  let word = rule!(
    "word" => one_or_more!(letter!())
  );

  let sentence = rule!(
    "sentence" => one_or_more!(seq!(expr!("word"), or!(c!(" "), c!("."), c!(","))))
  );

  let grammar = grammar!(word, sentence);

  let _ = parse!(
    grammar,
    &mut "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at tellus eros. Donec semper tempus facilisis. Mauris elementum non quam in aliquet. Nulla massa ligula, vehicula in aliquam varius, venenatis ut quam. Nam feugiat enim nec euismod pretium. Proin accumsan iaculis dolor id dignissim. Sed mollis sit amet mi a tincidunt. Proin at felis eget nisl cursus venenatis molestie nec augue. Morbi ut mauris nec risus gravida fringilla vehicula vitae orci. Pellentesque accumsan et mauris id ultricies. Donec tincidunt lorem tortor, id imperdiet lorem imperdiet ut. Pellentesque non quam at sem suscipit vestibulum eget non risus. Mauris auctor volutpat leo, vel interdum velit posuere sed. Donec ultricies justo non leo vestibulum, eu vehicula arcu commodo.",
    "sentence"
  );
}

fn bench_sentence(c: &mut Criterion) {
  c.bench_function("sentence", |b| b.iter(|| sentence()));
}

criterion_group!(sentence_group, bench_sentence);
