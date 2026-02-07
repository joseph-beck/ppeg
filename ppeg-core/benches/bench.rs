use criterion::criterion_main;

mod parser;

criterion_main! {
  parser::arithmetic_expression::arithmetic_expression_group,
  parser::direct_left_recursive::direct_left_recursive_group,
  parser::indirect_left_recursive::indirect_left_recursive_group,
}
