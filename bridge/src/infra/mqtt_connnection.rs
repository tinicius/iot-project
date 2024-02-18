use std::env::var;

use log::error;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder};

struct MQTTConnectionConfigs {
    protocol: String,
    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct MQQTConnection {}

impl MQQTConnection {
    pub fn new() -> Self {
        MQQTConnection {}
    }

    pub async fn create_client(&self, client_id: String) -> Result<AsyncClient, ()> {
        let envs = self.envs()?;

        let uri = format!("{}://{}:{}", envs.protocol, envs.host, envs.port);

        let configs = CreateOptionsBuilder::new()
            .server_uri(uri)
            .client_id(client_id)
            .finalize();

        let Ok(client) = AsyncClient::new(configs) else {
            return Err(());
        };

        match self.connect(&client, &envs.user, &envs.password).await {
            Ok(_) => Ok(client),
            Err(_) => Err(()),
        }
    }

    async fn connect(&self, client: &AsyncClient, user: &str, password: &str) -> Result<(), ()> {
        let connect_options = ConnectOptionsBuilder::new()
            .user_name(user)
            .password(password)
            .finalize();

        match client.connect(connect_options).await {
            Ok(_) => Ok(()),
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

        let Ok(user) = var("MQTT_USER") else {
            error!("Failed to read MQTT_USER env");
            return Err(());
        };

        let Ok(password) = var("MQTT_PASSWORD") else {
            error!("Failed to read MQTT_PASSWORD env");
            return Err(());
        };

        Ok(MQTTConnectionConfigs {
            protocol,
            host,
            port,
            user,
            password,
        })
    }
}
