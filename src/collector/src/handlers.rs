use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use base64::Engine;
use borsh::BorshDeserialize;
use common::{RoundId, Sol, Token, Transaction, TransactionId, Updraft, User, UserId};
use db::DataVersion;
use msg_broker::Handler;
use solana_signature::Signature;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage, UiTransactionEncoding,
};

use crate::{AppContext, Msg, error::HandlerError};

pub struct RaydiumSwap {
    wallet_id: String,
    token_amount: Token<Updraft>,
    sol_amount: Token<Sol>,
}

type Pubkey = [u8; 32];

#[derive(BorshDeserialize)]
pub struct SwapEvent {
    pub _pool_state: Pubkey,
    pub _sender: Pubkey,
    pub _token_account_0: Pubkey,
    pub _token_account_1: Pubkey,
    pub amount_0: u64,
    pub _transfer_fee_0: u64,
    pub amount_1: u64,
    pub _transfer_fee_1: u64,
    pub _zero_for_one: bool,
    pub _sqrt_price_x64: u128,
    pub _liquidity: u128,
    pub _tick: i32,
}

impl SwapEvent {
    pub const DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
}

impl RaydiumSwap {
    pub fn parse(tx: EncodedConfirmedTransactionWithStatusMeta) -> anyhow::Result<Self> {
        let transaction = match tx.transaction.transaction {
            EncodedTransaction::Json(transaction) => match transaction.message {
                UiMessage::Parsed(_) => {
                    anyhow::bail!("Expected raw transaction message got Parsed")
                }
                UiMessage::Raw(raw_message) => raw_message,
            },
            _ => {
                anyhow::bail!("Expected json transaction");
            }
        };

        let swap_ix = transaction
            .instructions
            .iter()
            .filter(|ix| {
                transaction.account_keys[ix.program_id_index as usize] == Raydium::PROGRAM_ID
                    && ix.data.starts_with(Raydium::SWAP_DISCRIMINATOR)
            })
            .next()
            .context("Not a SWAP")?;

        let data_logs = tx
            .transaction
            .meta
            .context("missing meta")?
            .log_messages
            .map(|x| x)
            .context("missing logs")?
            .iter()
            .filter_map(|log| {
                let mut log = log.split_whitespace();
                let tokens = log
                    .next()
                    .zip(log.next())
                    .zip(log.next())
                    .map(|((a, b), c)| (a, b, c));

                match tokens {
                    Some(("Program", "data:", log)) => {
                        Some(base64::prelude::BASE64_STANDARD.decode(log))
                    }
                    _ => None,
                }
            })
            .next()
            .transpose()
            .context("Invalid data log")?
            .and_then(|log| match log.starts_with(&SwapEvent::DISCRIMINATOR) {
                true => Some(log),
                false => None,
            })
            .context("No log found")?;

        let swap_event =
            borsh::from_slice::<SwapEvent>(&data_logs).context("failed to deseralize log")?;

        Ok(RaydiumSwap {
            wallet_id: transaction.account_keys[swap_ix.accounts[2] as usize].clone(),
            token_amount: Token::from_u64(swap_event.amount_1),
            sol_amount: Token::from_u64(swap_event.amount_0),
        })
    }
}

pub struct Raydium;

impl Raydium {
    pub const PROGRAM_ID: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
    pub const SWAP_DISCRIMINATOR: &str = "H83MW2TviE";
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
            .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
            .await?;
        let swap = RaydiumSwap::parse(tx)?;
        ctx.db_client
            .upsert_user(User::new(&swap.wallet_id), DataVersion::init())
            .await?;

        ctx.db_client
            .upsert_transctions(
                Transaction {
                    tx_id,
                    tx_signature: msg.signature.clone(),
                    user_id: UserId::from_pubkey(&swap.wallet_id),
                    round_id: RoundId::new(), // TODO generater Round id from timestamp
                    token_amount: swap.token_amount,
                    sol_amount: swap.sol_amount,
                    dex: common::Dex::Raydium,
                },
                DataVersion::init(),
            )
            .await?;

        Ok(())
    }
}
