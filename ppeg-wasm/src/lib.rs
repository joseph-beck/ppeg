use ppeg_core::{meta::parser::Meta, parser::cst, parser::peg::Parser};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Output {
  pub remaining: String,
  pub cst: Option<CST>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CST {
  pub value: String,
  pub children: Vec<CST>,
  pub label: Option<Label>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Label {
  pub hidden: bool,
}

fn map_cst(node: &cst::CST<'_>) -> CST {
  CST {
    value: node.get().to_string(),
    children: node.children().iter().map(map_cst).collect(),
    label: node.label().map(|label| Label {
      hidden: label.is_hidden(),
    }),
  }
}

#[wasm_bindgen(js_name = "parse")]
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

#[wasm_bindgen(js_name = "getRules")]
pub fn get_rules(grammar: &str) -> Result<JsValue, JsValue> {
  let grammar = Meta::new()
    .generate(grammar)
    .map_err(|e| JsValue::from_str(&format!("grammar: error generating grammar {}", e)))?;

  let rules: Vec<String> = grammar.rules().iter().map(|rule| rule.name().to_string()).collect();

  serde_wasm_bindgen::to_value(&rules).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
