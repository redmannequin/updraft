use std::marker::PhantomData;

use anyhow::Context;
use entities::{Round, Transaction, User};
use error::DbError;
use serde::Deserialize;
use tokio_postgres::{Config, NoTls, Row};
use uuid::Uuid;

pub mod entities;
mod error;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct DataVersion<T> {
    inner: i32,
    _ty: PhantomData<T>,
}

impl<T> DataVersion<T> {
    pub fn new(inner: i32) -> Self {
        DataVersion {
            inner,
            _ty: PhantomData,
        }
    }

    pub fn next(self) -> Result<i32> {
        Ok(self.inner.checked_add(1).context("version overflow")?)
    }
}

#[derive(Debug)]
pub struct DbClient {
    inner: tokio_postgres::Client,
}

impl DbClient {
    pub async fn connect(db_config: &DbConfig) -> Result<Self> {
        let (client, connection) = Config::new()
            .dbname(&db_config.name)
            .host(&db_config.host)
            .port(db_config.port)
            .user(&db_config.username)
            .password(db_config.password.as_bytes())
            .connect(NoTls)
            .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(DbClient { inner: client })
    }

    pub async fn get_user<T>(
        &self,
        user_id: impl Into<Uuid>,
    ) -> Result<Option<(T, DataVersion<User>)>>
    where
        T: From<User>,
    {
        let user_id = user_id.into();
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    user_id,
                    user_data,
                    data_version
                FROM users
                WHERE user_id = $1
            "#,
                &[&user_id],
            )
            .await?;
        row.map(user_from_row).transpose()
    }

    pub async fn upsert_user(
        &self,
        user: impl Into<User>,
        data_version: DataVersion<User>,
    ) -> Result<()> {
        let user = user.into();
        let data_version = data_version.next()?;
        let affected_rows = self
            .inner
            .execute(
                r#"
                    INSERT INTO users (
                        user_id,
                        user_data,
                        data_version,
                        created_at,
                        updated_at
                    )
                    VALUES ($1, $2, $3, NOW(), NOW())
                    ON CONFLICT (user_id) DO UPDATE SET
                        user_data = $2,
                        data_version = $3,
                        updated_at = NOW()
                    WHERE users.data_version = $2 - 1
                "#,
                &[&user.user_id, &user.user_data, &data_version],
            )
            .await?;

        match affected_rows {
            0 => Err(DbError::ConcurrentUpdate),
            1 => Ok(()),
            n => Err(DbError::Unknown(anyhow::anyhow!(
                "More than one({}) rows was updated",
                n
            ))),
        }
    }

    pub async fn get_round<T>(
        &self,
        round_id: impl Into<Uuid>,
    ) -> Result<Option<(T, DataVersion<Round>)>>
    where
        T: From<Round>,
    {
        let round_id = round_id.into();
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    round_id,
                    round_data,
                    data_version
                FROM rounds
                WHERE round_id = $1
            "#,
                &[&round_id],
            )
            .await?;
        row.map(round_from_row).transpose()
    }

    pub async fn upsert_round(
        &self,
        round: impl Into<Round>,
        data_version: DataVersion<Round>,
    ) -> Result<()> {
        let round = round.into();
        let data_version = data_version.next()?;
        let affected_rows = self
            .inner
            .execute(
                r#"
                    INSERT INTO rounds (
                        round_id,
                        round_data,
                        data_version,
                        created_at,
                        updated_at
                    )
                    VALUES ($1, $2, $3, NOW(), NOW())
                    ON CONFLICT (round_id) DO UPDATE SET
                        round_data = $2,
                        data_version = $3,
                        updated_at = NOW()
                    WHERE rounds.data_version = $2 - 1
                "#,
                &[&round.round_id, &round.round_data, &data_version],
            )
            .await?;

        match affected_rows {
            0 => Err(DbError::ConcurrentUpdate),
            1 => Ok(()),
            n => Err(DbError::Unknown(anyhow::anyhow!(
                "More than one({}) rows was updated",
                n
            ))),
        }
    }

    pub async fn get_round_transactions<T>(&self, round_id: impl Into<Uuid>) -> Result<Vec<T>>
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
                        user_id,
                        round_id,
                        transaction_data
                    FROM transactions
                    WHERE round_id = $1 
                "#,
                &[&round_id],
            )
            .await?;

        rows.into_iter()
            .map(transaction_from_row)
            .collect::<Result<_>>()
    }
}

fn user_from_row<T>(row: Row) -> Result<(T, DataVersion<User>)>
where
    T: From<User>,
{
    let user = User {
        user_id: row.try_get(0)?,
        user_data: row.try_get(1)?,
    };
    let data_version = DataVersion::new(row.try_get::<_, i32>(2)?);
    Ok((T::from(user), data_version))
}

fn round_from_row<T>(row: Row) -> Result<(T, DataVersion<Round>)>
where
    T: From<Round>,
{
    let round = Round {
        round_id: row.try_get(0)?,
        round_data: row.try_get(1)?,
    };
    let data_version = DataVersion::new(row.try_get::<_, i32>(2)?);
    Ok((T::from(round), data_version))
}

fn transaction_from_row<T>(row: Row) -> Result<T>
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
