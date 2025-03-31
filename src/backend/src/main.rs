use anyhow::Context;
use backend::AppConfig;
use config::{Config, Environment};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("failed to load .env")?;

    let config: AppConfig = Config::builder()
        .add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("config build")?
        .try_deserialize()
        .context("config deserialize")?;

    backend::run(config).await
}
