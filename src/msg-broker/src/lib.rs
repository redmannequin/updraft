use std::{fmt, sync::Arc};

use bincode::{Decode, Encode, error::DecodeError};
use handler_trait::InnerHandler;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct HandlerError<E> {
    pub inner_error: E,
    pub error_kind: ErrorKind,
}

impl<E> HandlerError<E> {
    pub fn fatal(err: E) -> Self {
        HandlerError {
            inner_error: err,
            error_kind: ErrorKind::Fatal,
        }
    }

    pub fn transient(err: E) -> Self {
        HandlerError {
            inner_error: err,
            error_kind: ErrorKind::Transient,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Transient,
    Fatal,
}

mod handler_trait {
    use std::{pin::Pin, sync::Arc};

    use bincode::error::DecodeError;

    use crate::{ErrorKind, Handler, HandlerError};

    pub trait InnerHandler: Send + Sync + 'static {
        type Context;
        type Error;

        fn handle<'a>(
            &'a self,
            ctx: Arc<Self::Context>,
            msg: Vec<u8>,
        ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError<Self::Error>>> + Send + 'a>>;
    }

    impl<T, Ctx, Err> InnerHandler for T
    where
        T: Handler<Context = Ctx, Error = Err>,
        Ctx: Send + Sync + 'static,
        Err: From<DecodeError> + Into<HandlerError<Err>> + Send + Sync + 'static,
    {
        type Context = Ctx;
        type Error = Err;

        fn handle<'a>(
            &'a self,
            ctx: Arc<Ctx>,
            msg: Vec<u8>,
        ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError<Self::Error>>> + Send + 'a>>
        {
            let msg = match bincode::decode_from_slice(&msg, bincode::config::standard()) {
                Ok(msg) => msg.0,
                Err(err) => {
                    return Box::pin(async {
                        Err(HandlerError {
                            inner_error: err.into(),
                            error_kind: ErrorKind::Fatal,
                        })
                    });
                }
            };
            Box::pin(async move { T::handle(&self, ctx, msg).await.map_err(Into::into) })
        }
    }
}

pub trait Handler: Send + Sync + 'static {
    type Context;
    type Error: From<DecodeError> + Into<HandlerError<Self::Error>>;
    type Msg: Encode + Decode<()> + Send + Sync;

    const ROUTING_KEY: &str;

    fn handle(
        &self,
        ctx: Arc<Self::Context>,
        msg: Self::Msg,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub struct MessageHandler<Ctx, Err>
where
    Ctx: Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    routing_key: &'static str,
    handler: Arc<dyn InnerHandler<Context = Ctx, Error = Err>>,
}

impl<Ctx, Err> MessageHandler<Ctx, Err>
where
    Ctx: Send + Sync + 'static,
    Err: From<DecodeError> + Into<HandlerError<Err>> + Send + Sync + 'static,
{
    pub fn new<T>(handler: T) -> Self
    where
        T: Handler<Context = Ctx, Error = Err>,
    {
        MessageHandler {
            routing_key: T::ROUTING_KEY,
            handler: Arc::new(handler),
        }
    }
}

pub struct Message {
    routing_key: String,
    data: Vec<u8>,
}

pub struct Publisher {
    tx: Sender<Message>,
}

impl Publisher {
    pub async fn send<T>(&self, msg: T::Msg)
    where
        T: Handler,
    {
        self.tx
            .send(Message {
                routing_key: T::ROUTING_KEY.to_string(),
                data: bincode::encode_to_vec(msg, bincode::config::standard()).expect("test"),
            })
            .await
            .expect("test")
    }
}

pub struct MessageBroker<Ctx, Err>
where
    Ctx: Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    tx: Sender<Message>,
    rx: Receiver<Message>,
    context: Arc<Ctx>,
    handlers: Vec<MessageHandler<Ctx, Err>>,
}

impl<Ctx, Err> MessageBroker<Ctx, Err>
where
    Ctx: Sync + Send + 'static,
    Err: fmt::Debug + Sync + Send + 'static,
{
    pub fn new(ctx: Ctx, handlers: Vec<MessageHandler<Ctx, Err>>) -> Self {
        let (tx, rx) = mpsc::channel(12);
        MessageBroker {
            tx,
            rx,
            context: Arc::new(ctx),
            handlers,
        }
    }

    pub fn get_publisher(&self) -> Publisher {
        Publisher {
            tx: self.tx.clone(),
        }
    }

    pub async fn run(mut self) {
        // TODO refactor to use recv_many
        // TODO create a pool of workers
        // TODO reply on transient error / record signatures for fatal errors
        while let Some(msg) = self.rx.recv().await {
            match self
                .handlers
                .iter()
                .find(|h| h.routing_key == msg.routing_key)
            {
                Some(handler) => {
                    let res = handler.handler.handle(self.context.clone(), msg.data).await;
                    if res.is_err() {
                        if !self.rx.is_closed() {
                            self.rx.close();
                        }
                        println!("WARN: handler err for {}", msg.routing_key);
                    }
                }
                None => {
                    println!("WARN: no handler for {}", msg.routing_key);
                    self.rx.close();
                }
            };
        }
    }
}
