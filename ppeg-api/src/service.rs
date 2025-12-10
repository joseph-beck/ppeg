use actix_web::{HttpResponse, Responder, get, post, web};

use crate::model;

/// Health check, returns "ok!" and HTTP 200.
#[get("/v1/health")]
async fn health() -> impl Responder {
  HttpResponse::Ok().body("ok!")
}

#[post("/v1/parse")]
async fn parse(parse_request: web::Json<model::Parse>) -> impl Responder {
  let grammar = parse_request.grammar().to_parser_grammar();
  let input = parse_request.input();
  let rule = parse_request.rule();

  let mut parser = ppeg_core::parser::Parser::new(grammar);
  let result = parser.parse(&mut input.as_str(), &rule);

  match result {
    Ok((remaining, cst)) => HttpResponse::Ok().body("ok!"),
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
    let parse_data = model::Parse::new(
      Some(grammar),
      Some("a".to_string()),
      Some("rule_a".to_string()),
    );

    let req = test::TestRequest::default()
      .uri("/v1/parse")
      .method(Method::POST)
      .set_json(parse_data)
      .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;

    assert_eq!(body, "ok!");
  }
}
