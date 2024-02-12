use async_trait::async_trait;
use futures_util::stream::StreamExt;
use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder};
use std::env::var;

use crate::services::{
    bridge::{SourceMessaging, TargetMessaging},
    data_convert::DataConvert,
};

struct MQTTConfigs {
    user: String,
    password: String,
}

pub struct MQTTMessaging {
    subscribes: Vec<(String, u8)>,
    client: AsyncClient,
    target_messaging: Box<dyn TargetMessaging + Send + Sync>,
    data_convert: DataConvert,
}

impl MQTTMessaging {
    pub fn new(
        client: AsyncClient,
        target_messaging: Box<dyn TargetMessaging + Send + Sync>,
        data_convert: DataConvert,
    ) -> Self {
        return MQTTMessaging {
            subscribes: vec![],
            client,
            target_messaging,
            data_convert,
        };
    }

    fn envs(&self) -> Result<MQTTConfigs, ()> {
        let Ok(user) = var("MQTT_USER") else {
            error!("Failed to read MQTT_USER env");
            return Err(());
        };

        let Ok(password) = var("MQTT_PASSWORD") else {
            error!("Failed to read MQTT_PASSWORD env");
            return Err(());
        };

        Ok(MQTTConfigs { user, password })
    }

    async fn connect(&self, user: String, password: String) -> Result<(), ()> {
        let connect_options = ConnectOptionsBuilder::new()
            .user_name(user)
            .password(password)
            .finalize();

        match self.client.connect(connect_options).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("{}", err);
                Err(())
            }
        }
    }

    fn subscribe_topics(&self) {
        for (topic, qos) in self.subscribes.clone() {
            self.client.subscribe(topic, qos.into());
        }
    }
}

#[async_trait]
impl SourceMessaging for MQTTMessaging {
    async fn listen_messages(&mut self) -> Result<(), ()> {
        let env: MQTTConfigs = self.envs()?;

        self.connect(env.user, env.password)
            .await
            .expect("Error on connect in mqtt!");

        self.subscribe_topics();

        self.target_messaging
            .create_exchange("iot".to_string())
            .await
            .expect("Error on create exchange!");

        self.target_messaging
            .create_queue("data".to_string())
            .await
            .expect("Error on create data queue!");

        self.target_messaging
            .create_queue("healthy".to_string())
            .await
            .expect("Error on create healthy queue!");

        self.target_messaging
            .bind_queue(
                "iot".to_string(),
                "data".to_string(),
                "data_key".to_string(),
            )
            .await
            .expect("Error on bind data queue!");

        self.target_messaging
            .bind_queue(
                "iot".to_string(),
                "healthy".to_string(),
                "healthy_key".to_string(),
            )
            .await
            .expect("Error on bind healthy queue!");

        let mut stream = self.client.get_stream(2048);

        while let Some(opt_infos) = stream.next().await {
            let Some(message) = opt_infos else {
                continue;
            };

            info!("Message receive!");
            info!("{:?}", message.topic().to_string());

            if message.topic().contains("healthy") {
                let data = self
                    .data_convert
                    .get_serialized_healthy_data(message.payload(), message.topic().to_string())
                    .expect("Error on serialize healthy data!");

                let exchange = String::from("iot");
                let rounting_key = String::from("healthy_key");

                self.target_messaging
                    .publish(&data, exchange, rounting_key)
                    .await
                    .expect("Error on publish message!");
            }

            if message.topic().contains("data") {
                let data = self
                    .data_convert
                    .get_serialized_transmit_algorithm_data(
                        message.payload(),
                        message.topic().to_string(),
                    )
                    .expect("Error on serialize transmit data!");

                let exchange = String::from("iot");
                let rounting_key = String::from("data_key");

                self.target_messaging
                    .publish(&data, exchange, rounting_key)
                    .await
                    .expect("Error on publish message!");
            }
        }

        Ok(())
    }

    fn subscribe(&mut self, topic: String, qos: u8) {
        self.subscribes.push((topic, qos));
    }
}

struct MQTTConnectionConfigs {
    protocol: String,
    host: String,
    port: String,
}

pub struct MQQTConnection {}

impl MQQTConnection {
    pub fn new() -> Self {
        return MQQTConnection {};
    }

    pub fn create_client(&self, client_id: String) -> Result<AsyncClient, ()> {
        let envs = self.envs()?;

        let uri = format!("{}://{}:{}", envs.protocol, envs.host, envs.port);

        let configs = CreateOptionsBuilder::new()
            .server_uri(uri)
            .client_id(client_id)
            .finalize();

        match AsyncClient::new(configs) {
            Ok(client) => Ok(client),
            Err(_) => Err(()),
        }
    }

    fn envs(&self) -> Result<MQTTConnectionConfigs, ()> {
        let Ok(protocol) = var("MQTT_PROTOCOL") else {
            error!("Failed to read MQTT_PROTOCOL env");
            return Err(());
        };

        let Ok(host) = var("MQTT_HOST") else {
            error!("Failed to read MQTT_HOST env");
            return Err(());
        };

        let Ok(port) = var("MQTT_PORT") else {
            error!("Failed to read MQTT_PORT env");
            return Err(());
        };

        Ok(MQTTConnectionConfigs {
            protocol,
            host,
            port,
        })
    }
}