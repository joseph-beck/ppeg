use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger};
use env_logger::Env;

pub mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin("http://localhost:5173")
          .allowed_methods(vec!["GET", "POST", "OPTIONS"])
          .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
          .supports_credentials()
          .max_age(3600),
      )
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
      .service(service::health)
      .service(service::parse)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
