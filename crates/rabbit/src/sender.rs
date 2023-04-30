use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::error::Error;
use crate::RabbitClient;

#[derive(Clone, Debug)]
pub struct Sender {
    pub client: RabbitClient,
    channel: Channel,
    exchange: String,
    routing_key: String,
    options: BasicPublishOptions,
    proprieties: BasicProperties,
    sender_queue: UnboundedSender<Vec<u8>>,
}

impl Sender {
    pub async fn new(
        client: RabbitClient,
        exchange: String,
        routing_key: String,
    ) -> Result<Self, Error> {
        let channel = client.get_channel().await?;
        let (tx, rx) = unbounded_channel::<Vec<u8>>();
        let sender = Self {
            client,
            channel,
            exchange,
            routing_key,
            options: Default::default(),
            proprieties: Default::default(),
            sender_queue: tx,
        };
        sender.configure_mpsc(rx);

        Ok(sender)
    }

    fn configure_mpsc(&self, mut rx: UnboundedReceiver<Vec<u8>>) {
        let sender = self.clone();
        tokio::spawn(async move {
            while let Some(payload) = rx.recv().await {
                let _ = sender.send(payload).await;
            }
        });
    }

    pub async fn send(&self, payload: Vec<u8>) -> Result<(), Error> {
        let _confirm = self
            .channel
            .basic_publish(
                self.exchange.as_str(),
                self.routing_key.as_str(),
                self.options.clone(),
                payload.as_slice(),
                self.proprieties.clone(),
            )
            .await?
            .await?;

        Ok(())
    }

    pub async fn push(&self, payload: Vec<u8>) -> Result<(), Error> {
        Ok(self.sender_queue.send(payload)?)
    }
}
