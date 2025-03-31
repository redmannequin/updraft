#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),
    #[error("Concurrent update error: version clash")]
    ConcurrentUpdate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
