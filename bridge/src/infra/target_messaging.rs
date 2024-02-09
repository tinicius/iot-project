use async_trait::async_trait;
use log::info;

use crate::services::bridge::TargetMessaging;

pub struct RabbitMQMessaging {}

#[async_trait]
impl TargetMessaging for RabbitMQMessaging {
    async fn publish(&self) {
        info!("Publishing...")
    }
}

impl RabbitMQMessaging {
    pub fn new() -> Self {
        return RabbitMQMessaging {};
    }
}
