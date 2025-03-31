use serde::Deserialize;

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

pub async fn run(_config: AppConfig) -> anyhow::Result<()> {
    Ok(())
}
