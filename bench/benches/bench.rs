use criterion::criterion_main;

mod nom;
mod pest;
mod ppeg;

criterion_main! {
  nom::arithmetic_expression::arithmetic_expression_group,
  nom::number::number_group,
  nom::sentence::sentence_group,
  nom::word::word_group,
  pest::arithmetic_expression::arithmetic_expression_group,
  pest::number::number_group,
  pest::sentence::sentence_group,
  pest::word::word_group,
  ppeg::arithmetic_expression::arithmetic_expression_group,
  ppeg::direct_left_recursive::direct_left_recursive_group,
  ppeg::indirect_left_recursive::indirect_left_recursive_group,
  ppeg::number::number_group,
  ppeg::sentence::sentence_group,
  ppeg::word::word_group,
}
