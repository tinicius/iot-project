use infra::{
    rabbitmq_connection::RabbitMQConnection, timeseries::Timestream,
    tms_connnection::TimestreamConnection,
};

use serde::Deserialize;
use services::{consumer::ConsumerService, serializer::Serializer};

#[derive(Deserialize, Debug)]
pub struct MQTTMessage {
    pub device: String,
    pub value: String,
    pub typ: String,
}

mod infra;
mod services;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    let channel = RabbitMQConnection::new()
        .connect()
        .await
        .expect("Error on create rabbitmq connection!");

    let serializer = Serializer::new();

    let timestream_connection = TimestreamConnection::new()
        .connect()
        .await
        .expect("Erro on create aws connection!");

    let timeseries = Timestream::new(timestream_connection, "iot-database".to_string());

    let consumer = ConsumerService::new(channel, serializer, Box::new(timeseries));

    let _ = consumer.listen().await;

    return Ok(());
}
