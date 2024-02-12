use std::env;

use async_trait::async_trait;
use lapin::{
    options::BasicPublishOptions, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties,
};
use log::{error, info};

use crate::services::bridge::TargetMessaging;

struct RabbitMQConfig {
    protocol: String,
    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct RabbitMQMessaging {
    _connection: Connection,
    channel: Channel,
}

pub struct RabbitMQConnection {}

#[async_trait]
impl TargetMessaging for RabbitMQMessaging {
    async fn publish(
        &self,
        data: &[u8],
        exchange_name: String,
        rounting_key: String,
    ) -> Result<(), ()> {
        let options = BasicPublishOptions {
            mandatory: false,
            immediate: false,
        };

        match self
            .channel
            .basic_publish(
                &exchange_name,
                &rounting_key,
                options,
                data,
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => {
                info!("published to rabbitmq!");
                return Ok(());
            }
            Err(error) => {
                error!("Failed to publish to rabbitmq: {}", error);
                return Err(());
            }
        };
    }

    async fn create_exchange(&self, exchange_name: String) -> Result<(), ()> {
        match self
            .channel
            .exchange_declare(
                &exchange_name,
                lapin::ExchangeKind::Direct,
                lapin::options::ExchangeDeclareOptions {
                    passive: true,
                    durable: true,
                    auto_delete: true,
                    internal: true,
                    nowait: true,
                },
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => {
                info!("Declared iot exchange!");
                return Ok(());
            }
            Err(err) => {
                error!("Error on declare iot exchange!");
                error!("{}", err);
                return Err(());
            }
        };
    }

    async fn create_queue(&self, queue_name: String) -> Result<(), ()> {
        match self
            .channel
            .queue_declare(
                &queue_name,
                lapin::options::QueueDeclareOptions {
                    passive: true,
                    durable: true,
                    exclusive: true,
                    auto_delete: true,
                    nowait: true,
                },
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => {
                info!("Declare {} queue!", queue_name);
                return Ok(());
            }
            Err(err) => {
                info!("Error on declare {} queue!", queue_name);
                info!("{}", err);
                return Err(());
            }
        };
    }

    async fn bind_queue(
        &self,
        exchange_name: String,
        queue_name: String,
        rounting_key: String,
    ) -> Result<(), ()> {
        match self
            .channel
            .queue_bind(
                &queue_name,
                &exchange_name,
                &rounting_key,
                lapin::options::QueueBindOptions { nowait: true },
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => {
                info!("Bind {} queue with {} exchange!", queue_name, exchange_name);
                // return Ok(());
            }
            Err(err) => {
                info!(
                    "Error bind {} queue with {} exchange!",
                    queue_name, exchange_name
                );
                info!("{}", err);
                return Err(());
            }
        };

        info!("{:?}", self.channel.status());
        return Ok(());
    }
}

impl RabbitMQMessaging {
    pub fn new(channel: Channel, connection: Connection) -> Self {
        info!("{:?}", channel.status());

        RabbitMQMessaging {
            channel,
            _connection: connection,
        }
    }
}

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

    pub async fn connect(&mut self) -> Result<(Connection, Channel), ()> {
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

        Ok((conn, channel))
    }
}
