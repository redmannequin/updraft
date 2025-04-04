use bincode::error::DecodeError;
use db::error::DbError;

#[derive(thiserror::Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    SolanaRpc(#[from] solana_client::client_error::ClientError),
    #[error(transparent)]
    Db(#[from] DbError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<HandlerError> for msg_broker::HandlerError<HandlerError> {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::Decode(_) => msg_broker::HandlerError::fatal(value),
            HandlerError::Db(ref db_error) => match db_error {
                DbError::Postgres(_) => msg_broker::HandlerError::fatal(value),
                DbError::ConcurrentUpdate => msg_broker::HandlerError::transient(value),
                DbError::Unknown(_) => msg_broker::HandlerError::fatal(value),
            },
            HandlerError::Other(_) => msg_broker::HandlerError::fatal(value),
            HandlerError::SolanaRpc(ref error) => match &error.kind {
                solana_client::client_error::ClientErrorKind::Io(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::Reqwest(error) => {
                    if error.is_timeout() {
                        msg_broker::HandlerError::transient(value)
                    } else {
                        msg_broker::HandlerError::fatal(value)
                    }
                }
                solana_client::client_error::ClientErrorKind::Middleware(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::RpcError(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::SerdeJson(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::SigningError(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::TransactionError(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
                solana_client::client_error::ClientErrorKind::Custom(_) => {
                    msg_broker::HandlerError::fatal(value)
                }
            },
        }
    }
}
