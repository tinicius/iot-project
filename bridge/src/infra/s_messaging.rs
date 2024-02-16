use async_trait::async_trait;
use futures_util::StreamExt;
use paho_mqtt::AsyncClient;

use crate::services_::bridge::{Serializer, SourceMessaging, TargetMessaging};

pub struct MQTTMessaging {
    client: AsyncClient,
    decoder: Box<dyn Serializer + Send + Sync>,
    messaging: Box<dyn TargetMessaging + Send + Sync>,
}

#[async_trait]
impl SourceMessaging for MQTTMessaging {
    async fn receive_messages(&self) -> Result<(), ()> {
        let mut stream = self.client.get_stream(2048);

        while let Some(option_message) = stream.next().await {
            let Some(message) = option_message else {
                continue;
            };

            if message.topic().contains("status") {
                // self.messaging.publishStatus(data);
                continue;
            }
        }

        Ok(())
    }
}

impl MQTTMessaging {
    pub fn new(
        client: AsyncClient,
        decoder: Box<dyn Serializer + Send + Sync>,
        messaging: Box<dyn TargetMessaging + Send + Sync>,
    ) -> Self {
        Self {
            client,
            decoder,
            messaging,
        }
    }
}
