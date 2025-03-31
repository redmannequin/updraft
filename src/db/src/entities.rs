use serde::{Deserialize, Serialize};
pub use tokio_postgres::types::Json;
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
// USER
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub user_data: Json<UserStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStats {
    V1(UserStatsV1),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsV1 {
    pub rounds_participated: u64,
    pub rounds_won: u64,
    pub amount_won: u64,
    pub amount_clamied: u64,
}

////////////////////////////////////////////////////////////////////////////////
// ROUND
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Round {
    pub round_id: Uuid,
    pub round_data: Json<RoundData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum RoundData {
    V1(RoundDataV1),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundDataV1 {
    status: RoundStatus,
    winner: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoundStatus {
    Active,
    Calculating,
    Distributing,
    Done,
}

////////////////////////////////////////////////////////////////////////////////
// TRANSACTION
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: Uuid,
    pub user_id: Uuid,
    pub round_id: Uuid,
    pub transaction_data: Json<TransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum TransactionData {
    V1(TransactionDataV1),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDataV1 {
    pub dex: Dex,
    pub amount: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Dex {
    Jupitor,
}
