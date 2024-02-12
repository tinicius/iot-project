use aws_sdk_timestreamquery::Client;

pub struct TimestreamConnection {}

impl TimestreamConnection {
    pub fn new() -> Self {
        TimestreamConnection {}
    }

    pub async fn connect(&self) -> Result<Client, ()> {
        let config = aws_config::load_from_env().await;

        match Client::new(&config).with_endpoint_discovery_enabled().await {
            Ok((client, _reload_endpoint)) => return Ok(client),
            Err(_) => {
                return Err(());
            }
        };
    }
}
