use log::info;

use crate::{
    infra::{
        mqtt_connnection::MQQTConnection, rabbitmq_connection::RabbitMQConnection,
        source_messaging::MQTTMessaging, target_messaging::RabbitMQMessaging,
    },
    services::{bridge::BridgeService, serializer::Serializer},
};

mod infra;
mod services;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    info!("Starting application...");

    let client = MQQTConnection::new()
        .create_client("BRIDGE_CLIENT".into())
        .await
        .expect("Error on create mqtt client!");

    let serializer = Serializer::new();

    let channel = RabbitMQConnection::new()
        .connect()
        .await
        .expect("Erro on create rabbitmq channel!");

    let target_messaging = RabbitMQMessaging::new(channel);

    target_messaging
        .config()
        .await
        .expect("Error configuring rabbitmq!");

    let source_messaging = MQTTMessaging::new(client, serializer, Box::new(target_messaging));

    let mut service = BridgeService::new(Box::new(source_messaging));

    service.run().await?;

    Ok(())
}
