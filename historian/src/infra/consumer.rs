use super::timeseries::Timeseries;
use crate::services::data_convert::DataConvert;
use futures_util::{stream::FuturesUnordered, Future, StreamExt};
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
    Channel,
};
use log::error;
use std::pin::Pin;

pub struct ConsumerService {
    channel: Channel,
    data_convert: DataConvert,
    timeseries: Timeseries,
}

impl ConsumerService {
    pub fn new(channel: Channel, data_convert: DataConvert, timeseries: Timeseries) -> Self {
        ConsumerService {
            channel,
            data_convert,
            timeseries,
        }
    }

    pub async fn listen(&self) -> Result<(), ()> {
        let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<(), ()>>>>>::new();

        futures.push(Box::pin(self.data_consumer("data".to_string())));
        futures.push(Box::pin(self.healthy_consumer("healthy".to_string())));

        while let Some(_) = futures.next().await {}

        Ok(())
    }

    async fn data_consumer(&self, queue: String) -> Result<(), ()> {
        let mut consumer = self
            .channel
            .basic_consume(
                &queue,
                "data-consumer",
                BasicConsumeOptions {
                    no_ack: false,
                    nowait: false,
                    exclusive: true,
                    no_local: true,
                },
                FieldTable::default(),
            )
            .await
            .expect("");

        while let Some(event) = consumer.next().await {
            match event {
                Ok(delivery) => {
                    let _ = self.handler_algorithm_data(delivery).await;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        Ok(())
    }

    async fn handler_algorithm_data(&self, delivery: Delivery) -> Result<(), ()> {
        let Ok(algorithm_data) = self
            .data_convert
            .get_deserialize_algorithm_data(&delivery.data)
        else {
            error!("Erro on deserialize algorithm data!");
            return Err(());
        };

        match self
            .timeseries
            .save_algorithm_data("data".to_string(), algorithm_data)
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

    async fn healthy_consumer(&self, queue: String) -> Result<(), ()> {
        let Ok(mut consumer) = self
            .channel
            .basic_consume(
                &queue,
                "healthy-consumer",
                BasicConsumeOptions {
                    no_ack: false,
                    nowait: false,
                    exclusive: true,
                    no_local: true,
                },
                FieldTable::default(),
            )
            .await
        else {
            return Err(());
        };

        while let Some(event) = consumer.next().await {
            match event {
                Ok(delivery) => {
                    let _ = self.handler_healthy_data(delivery).await;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        Ok(())
    }

    async fn handler_healthy_data(&self, delivery: Delivery) -> Result<(), ()> {
        let Ok(healthy_data) = self
            .data_convert
            .get_deserialize_healthy_data(&delivery.data)
        else {
            return Err(());
        };

        match self
            .timeseries
            .save_healthy_data(String::from("healthy"), healthy_data)
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
