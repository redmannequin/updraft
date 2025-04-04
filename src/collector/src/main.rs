use anyhow::Context;
use collector::AppConfig;
use config::{Config, Environment};
use dotenv::dotenv;

#[tokio::main(flavor = "current_thread")]
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

    collector::run(config).await
}
