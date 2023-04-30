use lapin::{options::BasicConsumeOptions, types::FieldTable, Channel, ConsumerDelegate};

use crate::{error::Error, RabbitClient};

#[derive(Clone, Debug)]
pub struct Consumer {
    pub client: RabbitClient,
    pub channel: Channel,
    pub queue: String,
    pub consumer_tag: String,
    pub options: BasicConsumeOptions,
    pub arguments: FieldTable,
}

impl Consumer {
    pub async fn new(
        client: RabbitClient,
        queue: String,
        consumer_tag: String,
    ) -> Result<Self, Error> {
        let channel = client.get_channel().await?;
        let options = BasicConsumeOptions::default();
        let arguments = FieldTable::default();

        let consumer = Self {
            client,
            channel,
            queue,
            consumer_tag,
            options,
            arguments,
        };

        Ok(consumer)
    }

    pub async fn spawn_delegate<D: ConsumerDelegate + 'static>(
        self,
        delegate: D,
    ) -> Result<(), Error> {
        let consumer = self
            .channel
            .basic_consume(
                self.queue.as_str(),
                self.consumer_tag.as_str(),
                self.options.clone(),
                self.arguments.clone(),
            )
            .await?;

        consumer.set_delegate(delegate);

        Ok(())
    }
}
