use async_trait::async_trait;

#[async_trait]
pub trait Bridge {
    fn new(&self, source_messaging: &impl SourceMessaging);
}

#[async_trait]
pub trait TargetMessaging {
    async fn publishService(&self, data: &[u8], service_type: u8) -> Result<(), ()>;
    async fn publishStatus(&self, data: &[u8]) -> Result<(), ()>;
}

#[async_trait]
pub trait SourceMessaging {
    async fn receive_messages(&self) -> Result<(), ()>;
}
