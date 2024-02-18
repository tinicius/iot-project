use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReceiveServiceData {
    value: f32,
    time: u64,
}

#[derive(Deserialize)]
pub struct ReceiveStatusData {
    #[serde(rename(deserialize = "batteryVoltage"))]
    battery_voltage: f32,
    signal: f32,
    time: u64,
}

#[derive(Serialize)]
pub struct TransmitServiceData {
    value: f32,
    time: u64,
    device: String,
    #[serde(rename(serialize = "type"))]
    typ: String,
}

#[derive(Serialize)]
pub struct TransmitStatusData {
    time: u64,
    device: String,
    #[serde(rename(serialize = "batteryVoltage"))]
    battery_voltage: f32,
    signal: f32,
}

pub struct Serializer {}

impl Serializer {
    pub fn new() -> Self {
        Serializer {}
    }

    pub fn get_serialized_transmit_service_data(
        &self,
        data: &[u8],
        topic: &str,
    ) -> Result<(Vec<u8>, String), ()> {
        let Ok(transmit_data) = self.deserialize_service_data(data, topic) else {
            return Err(());
        };

        let Ok(serialized) = serde_json::to_vec::<TransmitServiceData>(&transmit_data) else {
            return Err(());
        };

        Ok((serialized, transmit_data.typ))
    }

    pub fn get_serialized_status_data(&self, data: &[u8], topic: &str) -> Result<Vec<u8>, ()> {
        let Ok(transmit_data) = self.deserialize_status_data(data, topic) else {
            return Err(());
        };

        let Ok(serialized) = serde_json::to_vec::<TransmitStatusData>(&transmit_data) else {
            return Err(());
        };

        Ok(serialized)
    }

    fn deserialize_status_data(&self, data: &[u8], topic: &str) -> Result<TransmitStatusData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<ReceiveStatusData>(data) else {
            error!("Error on deserialize receive message!");
            return Err(());
        };

        let device = self.get_device_from_status_topic(topic);

        Ok(TransmitStatusData {
            time: deserialized.time,
            device,
            battery_voltage: deserialized.battery_voltage,
            signal: deserialized.signal,
        })
    }

    fn deserialize_service_data(
        &self,
        data: &[u8],
        topic: &str,
    ) -> Result<TransmitServiceData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<ReceiveServiceData>(data) else {
            error!("Error on deserialize receive message!");
            return Err(());
        };

        let (device, typ) = self.get_device_and_type_from_topic(topic);

        Ok(TransmitServiceData {
            value: deserialized.value,
            time: deserialized.time,
            device,
            typ,
        })
    }

    fn get_device_and_type_from_topic(&self, topic: &str) -> (String, String) {
        let split = topic.split("/");

        let v: Vec<&str> = split.collect();

        let device = v[2];
        let typ = v[3];

        (device.to_string(), typ.to_string())
    }

    fn get_device_from_status_topic(&self, topic: &str) -> String {
        let split = topic.split("/");

        let v: Vec<&str> = split.collect();

        let device = v[2];

        device.to_string()
    }
}
