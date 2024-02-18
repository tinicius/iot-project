use async_trait::async_trait;
use futures_util::StreamExt;
use paho_mqtt::AsyncClient;

use crate::{
    services::bridge::{SourceMessaging, TargetMessaging},
    services::serializer::Serializer,
};

pub struct MQTTMessaging {
    client: AsyncClient,
    serializer: Serializer,
    messaging: Box<dyn TargetMessaging + Send + Sync>,
}

#[async_trait]
impl SourceMessaging for MQTTMessaging {
    async fn receive_messages(&mut self) -> Result<(), ()> {
        self.subscribe("IoTProject/#", 0);

        let mut stream = self.client.get_stream(2048);

        while let Some(option_message) = stream.next().await {
            let Some(message) = option_message else {
                continue;
            };

            if message.topic().contains("status") {
                let data = self
                    .serializer
                    .get_serialized_status_data(message.payload(), message.topic())?;

                self.messaging.publish_status(&data).await?;
                continue;
            };

            if message.topic().contains("services") {
                let (data, service_type) = self
                    .serializer
                    .get_serialized_transmit_service_data(message.payload(), message.topic())?;

                self.messaging.publish_service(&data, &service_type).await?;
                continue;
            };
        }

        Ok(())
    }
}

impl MQTTMessaging {
    pub fn new(
        client: AsyncClient,
        serializer: Serializer,
        messaging: Box<dyn TargetMessaging + Send + Sync>,
    ) -> Self {
        Self {
            client,
            serializer,
            messaging,
        }
    }

    fn subscribe(&self, topic: &str, qos: i32) {
        self.client.subscribe(topic, qos);
    }
}
