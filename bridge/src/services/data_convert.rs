use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReceiveAlgorithmData {
    value: f32,
    time: u64,
}

#[derive(Deserialize)]
pub struct ReceiveHealthyData {
    #[serde(rename(deserialize = "batteryVoltage"))]
    battery_voltage: f32,
    service: Vec<String>,
    time: u64,
}

#[derive(Serialize)]
pub struct TransmitAlgorithmData {
    value: f32,
    time: u64,
    device: String,
    #[serde(rename(serialize = "type"))]
    typ: String,
}

#[derive(Serialize)]
pub struct TransmitHealthyData {
    time: u64,
    device: String,
    battery_voltage: f32,
    services: Vec<String>,
}

pub struct DataConvert {}

impl DataConvert {
    pub fn new() -> Self {
        DataConvert {}
    }

    pub fn get_serialized_transmit_algorithm_data(
        &self,
        data: &[u8],
        topic: String,
    ) -> Result<Vec<u8>, ()> {
        let Ok(transmit_data) = self.deserialize_algorithm_data(data, topic) else {
            return Err(());
        };

        let Ok(serialized) = serde_json::to_vec::<TransmitAlgorithmData>(&transmit_data) else {
            return Err(());
        };

        Ok(serialized)
    }

    pub fn get_serialized_healthy_data(&self, data: &[u8], topic: String) -> Result<Vec<u8>, ()> {
        let Ok(transmit_data) = self.deserialize_healthy_data(data, topic) else {
            return Err(());
        };

        let Ok(serialized) = serde_json::to_vec::<TransmitHealthyData>(&transmit_data) else {
            return Err(());
        };

        Ok(serialized)
    }

    fn deserialize_healthy_data(
        &self,
        data: &[u8],
        topic: String,
    ) -> Result<TransmitHealthyData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<ReceiveHealthyData>(data) else {
            error!("Error on deserialize receive message!");
            return Err(());
        };

        let device = self.get_device_from_healthy_topic(topic);

        Ok(TransmitHealthyData {
            time: deserialized.time,
            device,
            battery_voltage: deserialized.battery_voltage,
            services: deserialized.service,
        })
    }

    fn deserialize_algorithm_data(
        &self,
        data: &[u8],
        topic: String,
    ) -> Result<TransmitAlgorithmData, ()> {
        let Ok(deserialized) = serde_json::from_slice::<ReceiveAlgorithmData>(data) else {
            error!("Error on deserialize receive message!");
            return Err(());
        };

        let (device, typ) = self.get_device_and_type_from_topic(topic);

        Ok(TransmitAlgorithmData {
            value: deserialized.value,
            time: deserialized.time,
            device,
            typ,
        })
    }

    fn get_device_and_type_from_topic(&self, topic: String) -> (String, String) {
        let split = topic.split("/");

        let v: Vec<&str> = split.collect();

        let device = v[2];
        let typ = v[3];

        (device.to_string(), typ.to_string())
    }

    fn get_device_from_healthy_topic(&self, topic: String) -> String {
        let split = topic.split("/");

        let v: Vec<&str> = split.collect();

        let device = v[2];

        device.to_string()
    }
}
