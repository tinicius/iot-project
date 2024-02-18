use async_trait::async_trait;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, ExchangeKind,
};
use log::error;

use crate::services::bridge::TargetMessaging;

pub struct RabbitMQMessaging {
    channel: Channel,
}

#[async_trait]
impl TargetMessaging for RabbitMQMessaging {
    async fn publish_service(&self, data: &[u8], service_type: &str) -> Result<(), ()> {
        match service_type {
            "0" => self.basic_publish("IOT_PROJECT", "TEMP_KEY", data).await,
            "1" => {
                self.basic_publish("IOT_PROJECT", "HUMIDITY_KEY", data)
                    .await
            }
            _ => Err(()),
        }
    }

    async fn publish_status(&self, data: &[u8]) -> Result<(), ()> {
        match self.basic_publish("IOT_PROJECT", "STATUS_KEY", data).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(())
            }
        }
    }
}

impl RabbitMQMessaging {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub async fn config(&self) -> Result<(), ()> {
        self.create_exchange("IOT_PROJECT").await?;

        self.create_queue("TEMP").await?;
        self.create_queue("HUMIDITY").await?;

        self.bind_queue("IOT_PROJECT", "TEMP", "TEMP_KEY").await?;
        self.bind_queue("IOT_PROJECT", "HUMIDITY", "HUMIDITY_KEY")
            .await?;

        Ok(())
    }

    async fn basic_publish(
        &self,
        exchange_name: &str,
        routing_key: &str,
        data: &[u8],
    ) -> Result<(), ()> {
        let options = BasicPublishOptions {
            mandatory: false,
            immediate: false,
        };

        let properties = BasicProperties::default();

        match self
            .channel
            .basic_publish(exchange_name, routing_key, options, data, properties)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("->>{}", e);
                Err(())
            }
        }
    }

    async fn create_exchange(&self, exchange_name: &str) -> Result<(), ()> {
        let options = ExchangeDeclareOptions {
            passive: true,
            durable: true,
            auto_delete: true,
            internal: true,
            nowait: true,
        };

        let arguments = FieldTable::default();

        match self
            .channel
            .exchange_declare(exchange_name, ExchangeKind::Direct, options, arguments)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn create_queue(&self, queue_name: &str) -> Result<(), ()> {
        let options = QueueDeclareOptions {
            passive: true,
            durable: true,
            exclusive: true,
            auto_delete: true,
            nowait: true,
        };

        let arguments = FieldTable::default();

        match self
            .channel
            .queue_declare(queue_name, options, arguments)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn bind_queue(
        &self,
        exchange_name: &str,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<(), ()> {
        let options = QueueBindOptions { nowait: true };

        let arguments = FieldTable::default();

        match self
            .channel
            .queue_bind(queue_name, exchange_name, routing_key, options, arguments)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
