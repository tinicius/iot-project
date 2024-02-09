use async_trait::async_trait;
use futures_util::stream::StreamExt;
use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder};
use std::env::var;

use crate::services::bridge::SourceMessaging;

struct MQTTConfigs {
    user: String,
    password: String,
    protocol: String,
    host: String,
    port: String,
    client_id: String,
}

pub struct MQTTMessaging {
    subscribes: Vec<(String, u8)>,
    client: AsyncClient,
    // stream: Receiver<Option<Message>>,
}

impl MQTTMessaging {
    fn envs(&self) -> Result<MQTTConfigs, ()> {
        let Ok(user) = var("MQTT_USER") else {
            error!("Failed to read MQTT_USER env");
            return Err(());
        };

        let Ok(password) = var("MQTT_PASSWORD") else {
            error!("Failed to read MQTT_PASSWORD env");
            return Err(());
        };

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

        let Ok(client_id) = var("MQTT_CLIENT_ID") else {
            error!("Failed to read MQTT_CLIENT_ID env");
            return Err(());
        };

        Ok(MQTTConfigs {
            user,
            password,
            protocol,
            host,
            port,
            client_id,
        })
    }

    pub fn subscribe(&mut self, topic: String, qos: u8) {
        self.subscribes.push((topic, qos));
    }

    fn create_client(&self, uri: String, client_id: String) -> Result<(), ()> {
        let configs = CreateOptionsBuilder::new()
            .server_uri(uri)
            .client_id(client_id)
            .finalize();

        match AsyncClient::new(configs) {
            Ok(client) => {
                self.client = client;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    async fn connect(&self, user: String, password: String) -> Result<(), ()> {
        let connect_options = ConnectOptionsBuilder::new()
            .user_name(user)
            .password(password)
            .finalize();

        match self.client.connect(connect_options).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
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
    async fn listen_messages(&mut self, handler: &dyn FnMut() -> Result<(), ()>) -> Result<(), ()> {
        let env: MQTTConfigs = self.envs()?;

        let uri = format!("{}://{}:{}", env.protocol, env.host, env.port);

        self.create_client(uri, env.client_id)
            .expect("Erro on create mqtt client!");

        self.connect(env.user, env.password)
            .await
            .expect("Error on connect in mqtt!");

        self.subscribe_topics();

        let mut stream = self.client.get_stream(2048);

        while let Some(opt_infos) = stream.next().await {
            info!("Message receive!");
            info!("{:?}", opt_infos);

            // self.handler(opt_infos).await;
        }

        Ok(())
    }
}
