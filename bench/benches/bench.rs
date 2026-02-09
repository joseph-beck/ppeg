use criterion::criterion_main;

mod ppeg;

criterion_main! {
  ppeg::arithmetic_expression::arithmetic_expression_group,
  ppeg::direct_left_recursive::direct_left_recursive_group,
  ppeg::indirect_left_recursive::indirect_left_recursive_group,
  ppeg::number::number_group,
  ppeg::sentence::sentence_group,
  ppeg::word::word_group,
}
