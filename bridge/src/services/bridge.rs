use async_trait::async_trait;

#[async_trait]
pub trait TargetMessaging {
    async fn publish_service(&self, data: &[u8], service_type: &str) -> Result<(), ()>;
    async fn publish_status(&self, data: &[u8]) -> Result<(), ()>;
}

#[async_trait]
pub trait SourceMessaging {
    async fn receive_messages(&mut self) -> Result<(), ()>;
}

pub struct BridgeService {
    source_messaging: Box<dyn SourceMessaging + Send + Sync>,
}

impl BridgeService {
    pub fn new(source_messaging: Box<dyn SourceMessaging + Send + Sync>) -> Self {
        Self { source_messaging }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        self.source_messaging.receive_messages().await
    }
}
