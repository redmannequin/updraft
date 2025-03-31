use std::{fmt, ops::Deref};

use anyhow::Context;
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
// Common
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Updraft(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sol(u64);

////////////////////////////////////////////////////////////////////////////////
// Transaction
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: TransactionId,
    pub user_id: UserId,
    pub round_id: RoundId,
    pub token_amount: Updraft,
    pub sol_amount: Sol,
    pub dex: Dex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dex {
    Jupitor,
}

////////////////////////////////////////////////////////////////////////////////
// Database Mappings
////////////////////////////////////////////////////////////////////////////////

impl From<db::entities::Transaction> for Transaction {
    fn from(value: db::entities::Transaction) -> Self {
        let (token_amount, sol_amount, dex) = match value.transaction_data.0 {
            db::entities::TransactionData::V1(transaction_data_v1) => {
                let token_amount = Updraft(transaction_data_v1.token_amount);
                let sol_amount = Sol(transaction_data_v1.sol_amount);
                let dex = match transaction_data_v1.dex {
                    db::entities::Dex::Jupitor => Dex::Jupitor,
                };
                (token_amount, sol_amount, dex)
            }
        };

        Transaction {
            tx_id: TransactionId(value.tx_id),
            user_id: UserId(value.user_id),
            round_id: RoundId(value.round_id),
            token_amount,
            sol_amount,
            dex,
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

macro_rules! impl_token_ty {
    ($T:ty) => {
        impl $T {
            pub const ZERO: Self = Self(0);

            pub const fn to_u64(self) -> u64 {
                self.0
            }
        }

        impl Deref for $T {
            type Target = u64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

impl_token_ty!(Updraft);
impl_token_ty!(Sol);
