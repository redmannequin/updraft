use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use anyhow::Context;
use serde::Deserialize;

mod app;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub http_port: u16,
}

pub struct AppContext {}

impl AppContext {
    pub async fn init(_config: &AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {})
    }
}

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let app_context = web::Data::new(AppContext::init(&config).await?);

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .wrap(Logger::default())
            .service(web::resource("/health_check").get(HttpResponse::Ok))
            .service(app::app_scope())
    })
    .bind(("0.0.0.0", config.http_port))?
    .workers(1)
    .run();

    http_server.await.context("http_server")
}
