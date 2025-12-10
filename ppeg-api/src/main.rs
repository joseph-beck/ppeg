use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

pub mod model;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| {
    App::new()
      .service(service::health)
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
