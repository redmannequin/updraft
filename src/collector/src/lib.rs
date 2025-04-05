use std::{pin::Pin, sync::Arc};

use anyhow::Context;
use bincode::{Decode, Encode};
use db::{DbClient, DbConfig};
use futures::StreamExt;
use handlers::Raydium;
use msg_broker::{MessageBroker, MessageHandler, Publisher};
use serde::Deserialize;
use solana_client::{
    nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_commitment_config::{CommitmentConfig, CommitmentLevel};
use tokio::{
    sync::mpsc::{UnboundedReceiver, unbounded_channel},
    task::JoinHandle,
};

mod error;
mod handlers;
mod program;

#[derive(Debug, Deserialize)]
pub struct SolanaConfig {
    pub rpc_uri: String,
    pub ws_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub db_config: DbConfig,
    pub solana_config: SolanaConfig,
}

pub struct AppContext {
    pub db_client: DbClient,
    pub solana_rpc_client: RpcClient,
}

impl AppContext {
    pub async fn init(config: &AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {
            db_client: DbClient::connect(&config.db_config)
                .await
                .context("failed to connect to db")?,
            solana_rpc_client: RpcClient::new(config.solana_config.rpc_uri.clone()),
        })
    }
}

#[derive(Debug, Decode, Encode)]
pub struct Msg {
    signature: String,
}

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let ctx = AppContext::init(&config).await?;
    let borker = MessageBroker::new(ctx, vec![MessageHandler::new(Raydium)]);

    let publisher = borker.get_publisher();

    let (join_handle, mut unsubscribe_rx) = run_ws(&config.solana_config, publisher).await?;
    borker.run().await;

    while let Some((unsubscribe, name)) = unsubscribe_rx.recv().await {
        println!("unsubscribing from {}", name);
        unsubscribe().await
    }

    join_handle
        .await
        .context("join failed")?
        .context("producer failed")?;

    Ok(())
}

pub async fn run_ws(
    config: &SolanaConfig,
    publisher: Publisher,
) -> anyhow::Result<(
    JoinHandle<Result<(), anyhow::Error>>,
    UnboundedReceiver<(
        Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>,
        &'static str,
    )>,
)> {
    let (ready_sender, mut ready_receiver) = unbounded_channel::<()>();
    let (unsubscribe_sender, unsubscribe_receiver) = unbounded_channel::<(_, &'static str)>();

    let client = Arc::new(
        PubsubClient::new(&config.ws_uri)
            .await
            .context("Failed to connect to ws")?,
    );

    let join_handler = tokio::spawn({
        let ready_sender = ready_sender.clone();
        let unsubscribe_sender = unsubscribe_sender.clone();
        let client = client.clone();

        async move {
            let (mut log_notifications, log_unsubscribe) = client
                .logs_subscribe(
                    RpcTransactionLogsFilter::All,
                    RpcTransactionLogsConfig {
                        commitment: Some(CommitmentConfig {
                            commitment: CommitmentLevel::Processed,
                        }),
                    },
                )
                .await
                .context("Failed to log_subcribe")?;

            ready_sender
                .send(())
                .context("failed to send ready signal")?;
            unsubscribe_sender
                .send((log_unsubscribe, "log"))
                .map_err(|e| anyhow::anyhow!("failed to send log unsubribe: {e}"))?;
            drop((ready_sender, unsubscribe_sender));

            while let Some(log_info) = log_notifications.next().await {
                if log_info.value.err.is_some() {
                    continue;
                }

                for log in log_info.value.logs {
                    let mut log = log.split_whitespace();

                    let tokens = log
                        .next()
                        .zip(log.next())
                        .zip(log.next())
                        .map(|((a, b), c)| (a, b, c));

                    if let Some(("Program", id, "invoke")) = tokens {
                        match id {
                            Raydium::PROGRAM_ID => {
                                publisher
                                    .send::<Raydium>(Msg {
                                        signature: log_info.value.signature.to_string(),
                                    })
                                    .await
                            }
                            _ => continue,
                        }
                    }
                }
            }
            anyhow::Ok(())
        }
    });

    drop((ready_sender, unsubscribe_sender));
    while let Some(_) = ready_receiver.recv().await {}
    drop(ready_receiver);

    Ok((join_handler, unsubscribe_receiver))
}
