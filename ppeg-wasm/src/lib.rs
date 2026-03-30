use ppeg_core::{cst, meta::parser::Meta, parser::peg::Parser};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
struct Output {
  remaining: String,
  cst: Option<CST>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
struct CST {
  value: String,
  children: Vec<CST>,
  label: Option<Label>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
struct Label {
  hidden: bool,
  productive: bool,
}

fn map_cst(node: &cst::CST<'_>) -> CST {
  CST {
    value: node.get().to_string(),
    children: node.children().iter().map(map_cst).collect(),
    label: node.label().map(|label| Label {
      hidden: label.is_hidden(),
      productive: label.is_productive(),
    }),
  }
}

#[wasm_bindgen]
pub fn parse(input: &str, grammar: &str, rule: &str) -> Result<JsValue, JsValue> {
  let grammar = Meta::new()
    .generate(grammar)
    .map_err(|e| JsValue::from_str(&format!("grammar: error generating grammar {}", e)))?;

  let mut parser = Parser::new(grammar);
  let mut input_mut = input;

  let (remaining, cst) = parser
    .parse(&mut input_mut, rule)
    .map_err(|e| JsValue::from_str(&format!("parser: error parsing input {}", e)))?;

  let output = Output {
    remaining: remaining.to_string(),
    cst: cst.as_ref().map(map_cst),
  };

  serde_wasm_bindgen::to_value(&output).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_rules(grammar: &str) -> Result<JsValue, JsValue> {
  let grammar = Meta::new()
    .generate(grammar)
    .map_err(|e| JsValue::from_str(&format!("grammar: error generating grammar {}", e)))?;

  let rules: Vec<String> = grammar.rules().iter().map(|rule| rule.name.to_string()).collect();

  serde_wasm_bindgen::to_value(&rules).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
