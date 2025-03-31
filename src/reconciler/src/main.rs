use anyhow::Context;
use config::{Config, Environment};
use dotenv::dotenv;
use reconciler::AppConfig;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().context("failed to load .env")?;

    let config: AppConfig = Config::builder()
        .add_source(Environment::with_prefix("APP"))
        .build()
        .context("config build")?
        .try_deserialize()
        .context("config deserialize")?;

    reconciler::run(config).await
}
