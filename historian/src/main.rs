use infra::{
    consumer::ConsumerService, rmq_connection::RabbitMQConnection, timeseries::Timeseries,
    tms_connnection::TimestreamConnection,
};

use serde::Deserialize;
use services::data_convert::DataConvert;

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

    let (_connection, channel) = RabbitMQConnection::new()
        .connect()
        .await
        .expect("Error on create rabbitmq connection!");

    let data_convert = DataConvert::new();

    let timestream_connection = TimestreamConnection::new()
        .connect()
        .await
        .expect("Erro on create aws connection!");

    let timeseries = Timeseries::new(timestream_connection, "iot-database".to_string());

    let consumer = ConsumerService::new(channel, data_convert, timeseries);

    let _ = consumer.listen().await;

    return Ok(());
}
