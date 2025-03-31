use error::DbError;
use serde::Deserialize;
use tokio_postgres::{Config, NoTls};

mod error;

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct DbClient {
    inner: tokio_postgres::Client,
}

impl DbClient {
    pub async fn connect(db_config: DbConfig) -> Result<Self, DbError> {
        let (client, connection) = Config::new()
            .dbname(&db_config.name)
            .host(&db_config.host)
            .port(db_config.port)
            .user(&db_config.username)
            .password(db_config.password)
            .connect(NoTls)
            .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(DbClient { inner: client })
    }
}
