use async_trait::async_trait;

#[async_trait]
pub trait BridgeService {
    async fn subscribe(&self, topic: String, qos: u8);
    async fn run(&self) -> Result<(), ()>;
}

#[async_trait]
pub trait SourceMessaging {
    async fn listen_messages(&mut self, handler: &dyn FnMut() -> Result<(), ()>) -> Result<(), ()>;
}

#[async_trait]
pub trait TargetMessaging {
    async fn publish(&self);
}

pub struct BridgeServiceImpl {
    source_messaging: Box<dyn SourceMessaging>,
    target_messaging: Box<dyn TargetMessaging>,
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
