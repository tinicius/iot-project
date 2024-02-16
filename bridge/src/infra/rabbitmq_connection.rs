use std::env;

use lapin::{Channel, Connection, ConnectionProperties};
use log::{error, info};

struct RabbitMQConfig {
    protocol: String,
    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct RabbitMQConnection {}

impl RabbitMQConnection {
    pub fn new() -> Self {
        RabbitMQConnection {}
    }

    fn envs(&self) -> Result<RabbitMQConfig, ()> {
        let Ok(host) = env::var("RABBITMQ_HOST") else {
            error!("Failed to read RABBITMQ_HOST!");
            return Err(());
        };

        let Ok(port) = env::var("RABBITMQ_PORT") else {
            error!("Failed to read RABBITMQ_PORT!");
            return Err(());
        };

        let Ok(protocol) = env::var("RABBITMQ_PROTOCOL") else {
            error!("Failed to read RABBITMQ_PROTOCOL!");
            return Err(());
        };

        let Ok(user) = env::var("RABBITMQ_USER") else {
            error!("Failed to read RABBITMQ_USER!");
            return Err(());
        };

        let Ok(password) = env::var("RABBITMQ_PASSWORD") else {
            error!("Failed to read RABBITMQ_PASSWORD!");
            return Err(());
        };

        Ok(RabbitMQConfig {
            protocol,
            host,
            port,
            user,
            password,
        })
    }

    pub async fn connect(&mut self) -> Result<Channel, ()> {
        let envs = self.envs()?;

        info!("Starting RABBITMQ connection...");

        let addr = format!(
            "{}://{}:{}@{}:{}",
            envs.protocol, envs.user, envs.password, envs.host, envs.port
        );

        info!("{}", addr);

        let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
            error!("Failed to connect in rabbitmq!");
            return Err(());
        };

        let Ok(channel) = conn.create_channel().await else {
            error!("Failed create channel in rabbitmq!");
            return Err(());
        };

        info!("RABBITMQ connected!");

        Ok(channel)
    }
}
