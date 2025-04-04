use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use common::{RoundId, Token, Transaction, TransactionId, User, UserId};
use db::DataVersion;
use msg_broker::Handler;
use solana_signature::Signature;
use solana_transaction_status_client_types::{
    EncodedTransaction, UiMessage, UiTransactionEncoding,
};

use crate::{AppContext, Msg, error::HandlerError};

pub struct Raydium;

impl Raydium {
    pub const PROGRAM_ID: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
}

impl Handler for Raydium {
    type Context = AppContext;
    type Error = HandlerError;
    type Msg = Msg;

    const ROUTING_KEY: &str = "raydium";

    async fn handle(&self, ctx: Arc<AppContext>, msg: Msg) -> Result<(), Self::Error> {
        let tx_id = TransactionId::from_signature(&msg.signature);

        if ctx
            .db_client
            .get_transaction::<Transaction>(tx_id)
            .await?
            .is_some()
        {
            return Ok(());
        }

        let signature = Signature::from_str(&msg.signature).context("Failed to parse signautre")?;
        let tx = ctx
            .solana_rpc_client
            .get_transaction(&signature, UiTransactionEncoding::Json)
            .await?;

        const SWAP_DISCRIMINATOR: &str = "H83MW2TviE";

        match tx.transaction.transaction {
            EncodedTransaction::Json(tx) => match tx.message {
                UiMessage::Parsed(_) => {
                    println!("  Json Parsed transaction detected (skipping)");
                }
                UiMessage::Raw(raw_message) => {
                    for instruction in raw_message.instructions {
                        if !instruction.data.starts_with(SWAP_DISCRIMINATOR) {
                            continue;
                        }
                        let payer = &raw_message.account_keys[instruction.accounts[0] as usize];
                        ctx.db_client
                            .upsert_user(User::new(&payer), DataVersion::init())
                            .await?;

                        ctx.db_client
                            .upsert_transctions(
                                Transaction {
                                    tx_id,
                                    tx_signature: msg.signature.clone(),
                                    user_id: UserId::from_pubkey(&payer),
                                    round_id: RoundId::new(), // TODO generater Round id from timestamp
                                    token_amount: Token::from_u64(0), // TODO extract amount from data
                                    sol_amount: Token::from_u64(0), // TODO extract amount from data
                                    dex: common::Dex::Raydium,
                                },
                                DataVersion::init(),
                            )
                            .await?;
                    }
                }
            },
            EncodedTransaction::LegacyBinary(_) => {
                println!("  Legacy Binary transaction detected (skipping)");
            }
            EncodedTransaction::Binary(_, _) => {
                println!("  Binary transaction detected (skipping)");
            }
            EncodedTransaction::Accounts(_) => {
                println!("  Accounts transaction detected (skipping)");
            }
        }

        Ok(())
    }
}
