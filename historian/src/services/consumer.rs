use async_trait::async_trait;
use futures_util::StreamExt;
use log::{error, info};

use std::pin::Pin;

use super::serializer::{Serializer, ServiceData, StatusData};
use futures_util::{stream::FuturesUnordered, Future};
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
    Channel,
};

#[async_trait]
pub trait Timeseries {
    async fn save_service_data(&self, table_name: String, data: ServiceData) -> Result<(), ()>;
    async fn save_status_data(&self, table_name: String, data: StatusData) -> Result<(), ()>;
}

pub struct ConsumerService {
    channel: Channel,
    serializer: Serializer,
    timeseries: Box<dyn Timeseries>,
}

impl ConsumerService {
    pub fn new(channel: Channel, serializer: Serializer, timeseries: Box<dyn Timeseries>) -> Self {
        ConsumerService {
            channel,
            serializer,
            timeseries,
        }
    }

    pub async fn listen(&self) -> Result<(), ()> {
        let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<(), ()>>>>>::new();

        futures.push(Box::pin(self.service_consumer(String::from("TEMP"))));
        futures.push(Box::pin(self.service_consumer(String::from("HUMIDITY"))));

        futures.push(Box::pin(self.status_consumer(String::from("STATUS"))));

        while let Some(_) = futures.next().await {}

        Ok(())
    }

    async fn service_consumer(&self, queue: String) -> Result<(), ()> {
        let mut consumer = self
            .channel
            .basic_consume(
                &queue,
                &format!("{}-tag", queue),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("");

        while let Some(event) = consumer.next().await {
            match event {
                Ok(delivery) => {
                    let _ = self.handler_service_data(delivery).await;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        Ok(())
    }

    async fn handler_service_data(&self, delivery: Delivery) -> Result<(), ()> {
        let Ok(service_data) = self.serializer.get_deserialize_service_data(&delivery.data) else {
            error!("Error on deserialize algorithm data!");
            return Err(());
        };

        info!("{:?}", service_data.device);

        match self
            .timeseries
            .save_service_data("data".to_string(), service_data)
            .await
        {
            Ok(_) => match delivery.ack(BasicAckOptions { multiple: false }).await {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            },
            Err(_) => {
                match delivery
                    .nack(BasicNackOptions {
                        multiple: false,
                        requeue: true,
                    })
                    .await
                {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(()),
                }
            }
        }
    }

    async fn status_consumer(&self, queue: String) -> Result<(), ()> {
        let Ok(mut consumer) = self
            .channel
            .basic_consume(
                &queue,
                &format!("{}-tag", queue),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            return Err(());
        };

        while let Some(event) = consumer.next().await {
            match event {
                Ok(delivery) => {
                    let _ = self.handler_status_data(delivery).await;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        Ok(())
    }

    async fn handler_status_data(&self, delivery: Delivery) -> Result<(), ()> {
        let Ok(status_data) = self.serializer.get_deserialize_status_data(&delivery.data) else {
            return Err(());
        };

        match self
            .timeseries
            .save_status_data(String::from("status"), status_data)
            .await
        {
            Ok(_) => match delivery.ack(BasicAckOptions { multiple: false }).await {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            },
            Err(_) => {
                match delivery
                    .nack(BasicNackOptions {
                        multiple: false,
                        requeue: true,
                    })
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(()),
                }
            }
        }
    }
}
