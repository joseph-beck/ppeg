use actix_web::{HttpResponse, Responder, get, post, web};
use ppeg_core::parser;

use crate::model::{self, CST, Output};

/// Health check, returns "ok!" and HTTP 200.
#[get("/v1/health")]
async fn health() -> impl Responder {
  HttpResponse::Ok().body("ok!")
}

/// Parse endpoint, returns the parse result or an error.
/// Either HTTP 200 with parse result or HTTP 400 with error message.
/// Request body looks like:
/// ```json
/// {
///   "grammar": { ... },
///   "input": "input string",
///   "rule": "rule"
/// }
/// ```
#[post("/v1/parse")]
async fn parse(parse_request: web::Json<model::Parse>) -> impl Responder {
  let grammar = parse_request.grammar().to_parser_grammar();
  let input = parse_request.input().clone();
  let rule = parse_request.rule().clone();

  let result = {
    // as ppeg uses lifetimes this is a bit tricky.
    // for now give them a 'static lifetime.
    // we must use String in these models for serde.
    let mut input_static: &'static str = Box::leak(input.into_boxed_str());
    let rule_static: &'static str = Box::leak(rule.into_boxed_str());

    let mut parser = parser::Parser::new(grammar);

    parser.parse(&mut input_static, rule_static)
  };

  match result {
    Ok((remaining, parser_cst)) => {
      let cst = parser_cst.map(|cst| CST::from_parser_cst(&cst));
      let output = Output::new(remaining.to_string(), cst);
      HttpResponse::Ok().json(output)
    }
    Err(err) => HttpResponse::BadRequest().body(err.to_string()),
  }
}

#[cfg(test)]
mod health_tests {
  use super::*;
  use actix_web::{App, http::Method, test};

  #[actix_web::test]
  async fn test_health_get() {
    let app = test::init_service(App::new().service(health)).await;

    let req = test::TestRequest::default()
      .uri("/v1/health")
      .method(Method::GET)
      .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;

    assert_eq!(body, "ok!");
  }
}

#[cfg(test)]
mod parse_tests {
  use super::*;
  use actix_web::{App, http::Method, test};

  #[actix_web::test]
  async fn test_parse_post() {
    let app = test::init_service(App::new().service(parse)).await;

    let grammar = model::Grammar::new(Some(vec![model::Rule::new(
      "rule_a".to_string(),
      model::Expression::Char("a".to_string()),
    )]));
    let parse_data = model::Parse::new(Some(grammar), Some("a".to_string()), Some("rule_a".to_string()));

    let req = test::TestRequest::default()
      .uri("/v1/parse")
      .method(Method::POST)
      .set_json(parse_data)
      .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: model::Output = test::read_body_json(resp).await;

    let res = model::Output::new(
      "".to_string(),
      Some(CST::new(
        "rule_a".to_string(),
        vec![CST::new(
          "char".to_string(),
          vec![CST::new("a".to_string(), vec![], None)],
          None,
        )],
        None,
      )),
    );

    assert_eq!(body, res);
  }
}
