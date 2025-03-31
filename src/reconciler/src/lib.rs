use std::collections::HashMap;

use anyhow::Context;
use common::{RoundId, Sol, Token, Transaction, TransactionId, Updraft, UserId};
use db::{DbClient, DbConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    db_config: DbConfig,
}

pub struct AppContext {
    db_client: DbClient,
}

impl AppContext {
    pub async fn init(config: &AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {
            db_client: DbClient::connect(&config.db_config)
                .await
                .context("failed to connect to db")?,
        })
    }
}

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let ctx = AppContext::init(&config).await?;

    // TODO: get round from db
    let round_id = RoundId::new();

    let transactions = ctx
        .db_client
        .get_round_transactions::<Transaction>(round_id)
        .await
        .context("failed to get round transactions")?;

    let mut top_score = 0.0;
    let mut top_user = UserId::NIL;
    let mut user_score_map = HashMap::new();

    for tx in transactions {
        user_score_map
            .entry(tx.user_id)
            .and_modify(|info: &mut UserRoundInfo| {
                info.update(tx);
                if top_score < info.score {
                    top_score = info.score;
                    top_user = tx.user_id;
                }
            })
            .or_insert(UserRoundInfo::new(tx));
    }

    // TODO: update round info
    let top = user_score_map
        .get(&top_user)
        .context("somthing has really gone wrong")?;

    println!("{:?}", top);

    Ok(())
}

#[derive(Debug)]
pub struct UserRoundInfo {
    score: f64,
    bid: Bid,
}

impl UserRoundInfo {
    pub fn new(tx: Transaction) -> Self {
        let bid = Bid::new(tx);
        UserRoundInfo {
            score: bid.score(),
            bid: bid,
        }
    }

    pub fn update(&mut self, tx: Transaction) {
        let bid = Bid::new(tx);
        let score = bid.score();
        if score > self.score {
            self.score = score;
            self.bid = bid;
        }
    }
}

#[derive(Debug)]
pub struct Bid {
    pub tx_id: TransactionId,
    pub token_amount: Token<Updraft>,
    pub sol_amount: Token<Sol>,
}

impl Bid {
    pub fn new(tx: Transaction) -> Self {
        Bid {
            tx_id: tx.tx_id,
            token_amount: tx.token_amount,
            sol_amount: tx.sol_amount,
        }
    }

    pub fn score(&self) -> f64 {
        let token_amount = self.token_amount.to_u64() as f64;
        let sol_amount = self.sol_amount.to_u64() as f64;
        (token_amount / sol_amount) * (1.0 + token_amount).log10()
    }
}
