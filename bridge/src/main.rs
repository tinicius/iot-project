use infra::{
    source_messaging::{MQQTConnection, MQTTMessaging},
    target_messaging::{self, RabbitMQMessaging},
};
use log::info;
use services::bridge::{BridgeService, BridgeServiceImpl};
mod infra;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    info!("Starting application...");

    let target_messaging = RabbitMQMessaging::new();

    let client = MQQTConnection::new()
        .create_client("client_id".to_string())
        .expect("Erro on create mqtt client!");

    let source_messaging = MQTTMessaging::new(client, Box::new(target_messaging));

    let mut service = BridgeServiceImpl::new(Box::new(source_messaging));

    let _ = service.subscribe("IoTProject/#".to_string(), 0);

    service.run().await.expect("Error running!");
}
