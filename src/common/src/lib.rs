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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(Uuid);

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
// Transaction
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: TransactionId,
    pub user_id: UserId,
    pub round_id: RoundId,
    pub token_amount: Token<Updraft>,
    pub sol_amount: Token<Sol>,
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
                let token_amount = Token::from_u64(transaction_data_v1.token_amount);
                let sol_amount = Token::from_u64(transaction_data_v1.sol_amount);
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
