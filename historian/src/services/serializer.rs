use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServiceData {
    pub value: f32,
    pub time: u64,
    pub device: String,
    #[serde(rename(deserialize = "type"))]
    pub typ: String,
}

#[derive(Deserialize)]
pub struct StatusData {
    pub time: u64,
    pub device: String,
    #[serde(rename(deserialize = "battery_voltage", deserialize = "batteryVoltage"))]
    pub battery_voltage: f32,
    pub signal: f32,
}

pub struct Serializer {}

impl Serializer {
    pub fn new() -> Self {
        Serializer {}
    }

    pub fn get_deserialize_status_data(&self, data: &[u8]) -> Result<StatusData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<StatusData>(data) else {
            error!("Error on deserialize receive status message!");
            return Err(());
        };

        Ok(deserialized)
    }

    pub fn get_deserialize_service_data(&self, data: &[u8]) -> Result<ServiceData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<ServiceData>(data) else {
            error!("Error on deserialize receive service message!");
            return Err(());
        };

        Ok(deserialized)
    }
}
