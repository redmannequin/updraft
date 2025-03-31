use entities::Transaction;
use error::DbError;
use serde::Deserialize;
use tokio_postgres::{Config, NoTls, Row};
use uuid::Uuid;

pub mod entities;
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

    pub async fn get_round_transactions<T>(
        &self,
        round_id: impl Into<Uuid>,
    ) -> Result<Vec<T>, DbError>
    where
        T: From<Transaction>,
    {
        let round_id = round_id.into();
        let rows = self
            .inner
            .query(
                r#"
                    SELECT
                        transaction_id
                        round_id,
                        user_id,
                        transaction_data
                    FROM transactions
                    WHERE round_id = $1 
                "#,
                &[&round_id],
            )
            .await?;

        rows.into_iter()
            .map(transaction_from_row)
            .collect::<Result<_, _>>()
    }
}

fn transaction_from_row<T>(row: Row) -> Result<T, DbError>
where
    T: From<Transaction>,
{
    let transaction = Transaction {
        tx_id: row.try_get(0)?,
        user_id: row.try_get(1)?,
        round_id: row.try_get(2)?,
        transaction_data: row.try_get(3)?,
    };
    Ok(T::from(transaction))
}
