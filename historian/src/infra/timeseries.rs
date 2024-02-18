use async_trait::async_trait;
use aws_sdk_timestreamwrite::{
    types::{Dimension, MeasureValue, MeasureValueType, Record, TimeUnit},
    Client,
};
use log::error;

use crate::services::{
    consumer::Timeseries,
    serializer::{ServiceData, StatusData},
};

pub struct Timestream {
    client: Client,
    database: String,
}

#[async_trait]
impl Timeseries for Timestream {
    async fn save_service_data(&self, table_name: String, data: ServiceData) -> Result<(), ()> {
        let Ok(record) = self.create_service_data_record(data) else {
            return Err(());
        };

        match self
            .client
            .write_records()
            .set_database_name(Some(self.database.to_string()))
            .set_table_name(Some(table_name))
            .set_records(Some(vec![record]))
            .send()
            .await
        {
            Ok(_) => return Ok(()),
            Err(err) => {
                println!("Error: {}", err.to_string());
                return Err(());
            }
        };
    }

    async fn save_status_data(&self, table_name: String, data: StatusData) -> Result<(), ()> {
        let Ok(record) = self.create_status_record(data) else {
            return Err(());
        };

        match self
            .client
            .write_records()
            .set_database_name(Some(self.database.to_string()))
            .set_table_name(Some(table_name))
            .set_records(Some(vec![record]))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error: {}", err.to_string());
                Err(())
            }
        }
    }
}

impl Timestream {
    pub fn new(client: Client, database: String) -> Self {
        Self { client, database }
    }

    fn create_service_data_record(&self, data: ServiceData) -> Result<Record, ()> {
        let Ok(device) = self.create_dimension("device".into(), data.device.to_string()) else {
            return Err(());
        };

        let Ok(typ) = self.create_dimension("type".into(), data.typ.to_string()) else {
            return Err(());
        };

        Ok(Record::builder()
            .set_time(Some(data.time.to_string()))
            .set_time_unit(Some(TimeUnit::Milliseconds))
            .set_measure_name(Some("value".to_string()))
            .set_dimensions(Some(vec![device, typ]))
            .set_measure_value(Some(data.value.to_string()))
            .set_measure_value_type(Some(MeasureValueType::Double))
            .build())
    }

    fn create_status_record(&self, data: StatusData) -> Result<Record, ()> {
        let Ok(device) = self.create_dimension("device".into(), data.device.to_string()) else {
            return Err(());
        };

        let Ok(battery_voltage) = self.create_measure(
            String::from("battery"),
            MeasureValueType::Double,
            data.battery_voltage.to_string(),
        ) else {
            return Err(());
        };

        let Ok(signal) = self.create_measure(
            String::from("signal"),
            MeasureValueType::Double,
            data.signal.to_string(),
        ) else {
            return Err(());
        };

        Ok(Record::builder()
            .set_time(Some(data.time.to_string()))
            .set_time_unit(Some(TimeUnit::Milliseconds))
            .set_dimensions(Some(vec![device]))
            .set_measure_name(Some("status".to_string()))
            .set_measure_values(Some(vec![battery_voltage, signal]))
            .set_measure_value_type(Some(MeasureValueType::Multi))
            .build())
    }

    fn create_dimension(&self, name: String, value: String) -> Result<Dimension, ()> {
        match Dimension::builder()
            .set_name(Some(name))
            .set_value(Some(value))
            .build()
        {
            Ok(dimension) => Ok(dimension),
            Err(_) => {
                error!("Error on create dimension");
                Err(())
            }
        }
    }

    fn create_measure(
        &self,
        name: String,
        typ: MeasureValueType,
        value: String,
    ) -> Result<MeasureValue, ()> {
        match MeasureValue::builder()
            .set_name(Some(name))
            .set_type(Some(typ))
            .set_value(Some(value))
            .build()
        {
            Ok(measure) => Ok(measure),
            Err(_) => {
                error!("Error on create measure!");
                Err(())
            }
        }
    }
}
