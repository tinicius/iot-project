use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlgorithmData {
    pub value: f32,
    pub time: u64,
    pub device: String,
    #[serde(rename(deserialize = "type"))]
    pub typ: String,
}

#[derive(Deserialize, Debug)]
pub struct HealthyData {
    pub time: u64,
    pub device: String,
    pub battery_voltage: f32,
    pub services: Vec<String>,
}

pub struct DataConvert {}

impl DataConvert {
    pub fn new() -> Self {
        DataConvert {}
    }

    pub fn get_deserialize_algorithm_data(&self, data: &[u8]) -> Result<AlgorithmData, ()> {
        match serde_json::from_slice::<AlgorithmData>(data) {
            Ok(deserialized) => {
                info!("{:?}", deserialized);
                Ok(deserialized)
            }
            Err(err) => {
                error!("Error on deserialize receive message!");
                error!("{}", err);
                Err(())
            }
        }
    }

    pub fn get_deserialize_healthy_data(&self, data: &[u8]) -> Result<HealthyData, ()> {
        match serde_json::from_slice::<HealthyData>(data) {
            Ok(desialized) => Ok(desialized),
            Err(err) => {
                error!("Error on deserialize receive message!");
                error!("{}", err);
                Err(())
            }
        }
    }
}
