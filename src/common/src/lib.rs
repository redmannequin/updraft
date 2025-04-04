use std::{fmt, marker::PhantomData, ops};

use anyhow::Context;
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
// Common
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionId(Uuid);

impl TransactionId {
    pub fn from_signature(signature: &str) -> Self {
        TransactionId(Uuid::new_v5(&Uuid::NAMESPACE_OID, signature.as_bytes()))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn from_pubkey(key: &str) -> Self {
        UserId(Uuid::new_v5(&Uuid::NAMESPACE_OID, key.as_bytes()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tokens
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Updraft;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<T> {
    inner: u64,
    _ty: PhantomData<T>,
}

impl<T> Token<T> {
    pub const ZERO: Self = Self::zero();

    pub const fn zero() -> Self {
        Token {
            inner: 0,
            _ty: PhantomData,
        }
    }

    pub fn from_u64(amount: u64) -> Self {
        Token {
            inner: amount,
            _ty: PhantomData,
        }
    }

    pub fn to_u64(self) -> u64 {
        self.inner
    }
}

impl<T> ops::Add for Token<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Token {
            inner: self.inner + rhs.inner,
            _ty: PhantomData,
        }
    }
}

impl<T> ops::AddAssign for Token<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

impl<T> ops::Sub for Token<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Token {
            inner: self.inner + rhs.inner,
            _ty: PhantomData,
        }
    }
}

impl<T> ops::SubAssign for Token<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}

impl<T> ops::Mul for Token<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Token {
            inner: self.inner * rhs.inner,
            _ty: PhantomData,
        }
    }
}

impl<T> ops::MulAssign for Token<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.inner *= rhs.inner;
    }
}

impl<T> ops::Div for Token<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Token {
            inner: self.inner * rhs.inner,
            _ty: PhantomData,
        }
    }
}

impl<T> ops::DivAssign for Token<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.inner /= rhs.inner;
    }
}

impl<T> ops::Rem for Token<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Token {
            inner: self.inner % rhs.inner,
            _ty: PhantomData,
        }
    }
}

impl<T> ops::RemAssign for Token<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.inner %= rhs.inner;
    }
}

////////////////////////////////////////////////////////////////////////////////
// User
////////////////////////////////////////////////////////////////////////////////

pub struct User {
    pub user_id: UserId,
    pub rounds_participated: u64,
    pub rounds_won: u64,
    pub amount_won: Token<Updraft>,
    pub amount_clamied: Token<Updraft>,
}

impl User {
    pub fn new(key: &str) -> Self {
        User {
            user_id: UserId::from_pubkey(key),
            rounds_participated: 0,
            rounds_won: 0,
            amount_won: Token::ZERO,
            amount_clamied: Token::ZERO,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Round
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
    pub round_id: RoundId,
    pub round_status: RoundStatus,
    pub round_winner: Option<RoundWinner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundStatus {
    Active,
    ReconcileDue,
    Reconciled,
    Prosessing,
    Done,
}

impl fmt::Display for RoundStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundWinner {
    pub user_id: UserId,
    pub tx_id: TransactionId,
}

////////////////////////////////////////////////////////////////////////////////
// Transaction
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: TransactionId,
    pub tx_signature: String,
    pub user_id: UserId,
    pub round_id: RoundId,
    pub token_amount: Token<Updraft>,
    pub sol_amount: Token<Sol>,
    pub dex: Dex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dex {
    Raydium,
}

////////////////////////////////////////////////////////////////////////////////
// Database Mappings
////////////////////////////////////////////////////////////////////////////////

impl From<db::entities::User> for User {
    fn from(value: db::entities::User) -> Self {
        let (rounds_participated, rounds_won, amount_won, amount_clamied) = match value.user_data.0
        {
            db::entities::UserData::V1(user_data_v1) => (
                user_data_v1.rounds_participated,
                user_data_v1.rounds_won,
                Token::from_u64(user_data_v1.amount_won),
                Token::from_u64(user_data_v1.amount_clamied),
            ),
        };
        User {
            user_id: UserId(value.user_id),
            rounds_participated,
            rounds_won,
            amount_won,
            amount_clamied,
        }
    }
}

impl From<User> for db::entities::User {
    fn from(value: User) -> Self {
        db::entities::User {
            user_id: value.user_id.0,
            user_data: db::entities::Json(db::entities::UserData::V1(db::entities::UserDataV1 {
                rounds_participated: value.rounds_participated,
                rounds_won: value.rounds_won,
                amount_won: value.amount_won.inner,
                amount_clamied: value.amount_clamied.inner,
            })),
        }
    }
}

impl From<db::entities::Round> for Round {
    fn from(value: db::entities::Round) -> Self {
        let (round_status, round_winner) = match value.round_data.0 {
            db::entities::RoundData::V1(round_data_v1) => {
                let status = match round_data_v1.status {
                    db::entities::RoundStatus::Active => RoundStatus::Active,
                    db::entities::RoundStatus::ReconcileDue => RoundStatus::ReconcileDue,
                    db::entities::RoundStatus::Reconciled => RoundStatus::Reconciled,
                    db::entities::RoundStatus::Prosessing => RoundStatus::Prosessing,
                    db::entities::RoundStatus::Done => RoundStatus::Done,
                };

                let winner = round_data_v1.winner.map(|w| RoundWinner {
                    user_id: UserId(w.user_id),
                    tx_id: TransactionId(w.tx_id),
                });

                (status, winner)
            }
        };

        Round {
            round_id: RoundId(value.round_id),
            round_status,
            round_winner,
        }
    }
}

impl From<Round> for db::entities::Round {
    fn from(value: Round) -> Self {
        db::entities::Round {
            round_id: value.round_id.0,
            round_data: db::entities::Json(db::entities::RoundData::V1(
                db::entities::RoundDataV1 {
                    status: match value.round_status {
                        RoundStatus::Active => db::entities::RoundStatus::Active,
                        RoundStatus::ReconcileDue => db::entities::RoundStatus::ReconcileDue,
                        RoundStatus::Reconciled => db::entities::RoundStatus::Reconciled,
                        RoundStatus::Prosessing => db::entities::RoundStatus::Prosessing,
                        RoundStatus::Done => db::entities::RoundStatus::Done,
                    },
                    winner: value.round_winner.map(|w| db::entities::RoundWinner {
                        user_id: w.user_id.0,
                        tx_id: w.tx_id.0,
                    }),
                },
            )),
        }
    }
}

impl From<db::entities::Transaction> for Transaction {
    fn from(value: db::entities::Transaction) -> Self {
        let (token_amount, sol_amount, dex) = match value.transaction_data.0 {
            db::entities::TransactionData::V1(transaction_data_v1) => {
                let token_amount = Token::from_u64(transaction_data_v1.token_amount);
                let sol_amount = Token::from_u64(transaction_data_v1.sol_amount);
                let dex = match transaction_data_v1.dex {
                    db::entities::Dex::Raydium => Dex::Raydium,
                };
                (token_amount, sol_amount, dex)
            }
        };

        Transaction {
            tx_id: TransactionId(value.tx_id),
            tx_signature: value.tx_signature,
            user_id: UserId(value.user_id),
            round_id: RoundId(value.round_id),
            token_amount,
            sol_amount,
            dex,
        }
    }
}

impl From<Transaction> for db::entities::Transaction {
    fn from(value: Transaction) -> Self {
        db::entities::Transaction {
            tx_id: value.tx_id.0,
            tx_signature: value.tx_signature,
            user_id: value.user_id.0,
            round_id: value.round_id.0,
            transaction_data: db::entities::Json(db::entities::TransactionData::V1(
                db::entities::TransactionDataV1 {
                    dex: match value.dex {
                        Dex::Raydium => db::entities::Dex::Raydium,
                    },
                    token_amount: value.token_amount.inner,
                    sol_amount: value.sol_amount.inner,
                },
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Macros
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_uuid_ty {
    ($T:ty) => {
        impl $T {
            pub const NIL: Self = Self::from_uuid(Uuid::nil());

            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn parse_str(uuid: &str) -> anyhow::Result<Self> {
                Uuid::parse_str(uuid)
                    .map(Self::from_uuid)
                    .context("Invalid Uuid")
            }

            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub const fn into_uuid(self) -> Uuid {
                self.0
            }

            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Uuid> for $T {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$T> for Uuid {
            fn from(value: $T) -> Self {
                value.0
            }
        }

        impl AsRef<Uuid> for $T {
            fn as_ref(&self) -> &Uuid {
                &self.0
            }
        }
    };
}

impl_uuid_ty!(UserId);
impl_uuid_ty!(TransactionId);
impl_uuid_ty!(RoundId);
