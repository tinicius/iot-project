use infra::{
    source_messaging::{MQQTConnection, MQTTMessaging},
    target_messaging::RabbitMQMessaging,
};
use log::info;
use services::bridge::{BridgeService, BridgeServiceImpl};

use crate::{infra::target_messaging::RabbitMQConnection, services::data_convert::DataConvert};
mod infra;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    info!("Starting application...");

    let (connection, channel) = RabbitMQConnection::new()
        .connect()
        .await
        .expect("Erro on create rabbitmq connection!");

    info!("{:?}", channel.status());

    let target_messaging = RabbitMQMessaging::new(channel, connection);

    let client = MQQTConnection::new()
        .create_client("client_id".to_string())
        .expect("Erro on create mqtt client!");

    let data_convert = DataConvert::new();

    let source_messaging = MQTTMessaging::new(client, Box::new(target_messaging), data_convert);

    let mut service = BridgeServiceImpl::new(Box::new(source_messaging));

    let _ = service.subscribe("IoTProject/#".to_string(), 0);

    service.run().await.expect("Error running!");
}
