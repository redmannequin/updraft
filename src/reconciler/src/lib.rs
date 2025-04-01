use anyhow::Context;
use common::{Round, RoundId, RoundStatus, RoundWinner, Transaction, TransactionId, UserId};
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

    // TODO get latest round id
    let round_id = RoundId::new();

    let (mut round, round_version) = ctx
        .db_client
        .get_round::<Round>(round_id)
        .await?
        .context("round not found")?;

    if round.round_status != RoundStatus::ReconcileDue {
        return Err(anyhow::anyhow!(
            "Invalid round status: {}",
            round.round_status
        ));
    }

    let mut top_score = 0.0;
    let mut top_user_id = UserId::NIL;
    let mut top_tx_id = TransactionId::NIL;

    {
        let transactions = ctx
            .db_client
            .get_round_transactions::<Transaction>(round_id)
            .await
            .context("failed to get round transactions")?;

        for tx in transactions.into_iter() {
            let token_amount = tx.token_amount.to_u64() as f64;
            let sol_amount = tx.sol_amount.to_u64() as f64;
            let score = (token_amount / sol_amount) * (1.0 + token_amount).log10();

            if score > top_score {
                top_score = top_score;
                top_tx_id = tx.tx_id;
                top_user_id = tx.user_id;
            }
        }
    }

    round.round_status = RoundStatus::Reconciled;
    round.round_winner = Some(RoundWinner {
        user_id: top_user_id,
        tx_id: top_tx_id,
    });

    ctx.db_client.upsert_round(round, round_version).await?;

    Ok(())
}
