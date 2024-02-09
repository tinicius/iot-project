use async_trait::async_trait;

#[async_trait]
pub trait BridgeService {
    fn subscribe(&mut self, topic: String, qos: u8);
    async fn run(&mut self) -> Result<(), ()>;
}

#[async_trait]
pub trait SourceMessaging {
    async fn listen_messages(&mut self) -> Result<(), ()>;
    fn subscribe(&mut self, topic: String, qos: u8);
}

#[async_trait]
pub trait TargetMessaging {
    async fn publish(&self);
}

pub struct BridgeServiceImpl {
    source_messaging: Box<dyn SourceMessaging + Send + Sync>,
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {
    fn subscribe(&mut self, topic: String, qos: u8) {
        self.source_messaging.subscribe(topic, qos);
    }

    async fn run(&mut self) -> Result<(), ()> {
        self.source_messaging
            .listen_messages()
            .await
            .expect("Error on source messaging!");

        Ok(())
    }
}

impl BridgeServiceImpl {
    pub fn new(source_messaging: Box<dyn SourceMessaging + Send + Sync>) -> Self {
        return BridgeServiceImpl { source_messaging };
    }
}
