use actix_web::{HttpResponse, Responder, get};

/// Health check, returns "ok!" and HTTP 200.
#[get("/v1/health")]
async fn health() -> impl Responder {
  HttpResponse::Ok().body("ok!")
}

#[cfg(test)]
mod health_tests {
  use super::*;
  use actix_web::{App, test};

  #[actix_web::test]
  async fn test_health_get() {
    let app = test::init_service(App::new().service(health)).await;
    let req = test::TestRequest::default().uri("/v1/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
  }
}
