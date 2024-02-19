use aws_sdk_timestreamquery::{
    types::{ColumnInfo, Row},
    Client,
};
use log::{error, info};

use crate::server::{Data, Device, Service};

pub struct Timeseries {
    client: Client,
}

impl Timeseries {
    pub fn new(client: Client) -> Self {
        Timeseries { client }
    }

    pub async fn gel_all_services(&self) -> Result<Vec<Device>, ()> {
        let mut devices: Vec<Device> = vec![];

        let Ok(devices_names) = self.get_all_devices_name().await else {
            error!("Error on query get all devices name!");
            return Err(());
        };

        for device_name in devices_names {
            let query: String = format!(
                "select * from \"iot-database\".data where device = \'{}\'",
                device_name
            );

            info!("{}", query);

            let Ok(services_query) = self
                .client
                .query()
                .set_query_string(Some(query))
                .send()
                .await
            else {
                return Err(());
            };

            let Ok(services) =
                self.process_services(services_query.column_info(), services_query.rows())
            else {
                return Err(());
            };

            devices.push(Device {
                device: device_name,
                services,
            })
        }

        Ok(devices)
    }

    pub async fn get_all_devices_name(&self) -> Result<Vec<String>, ()> {
        let query: String = String::from("select distinct \"device\" from \"iot-database\".data");

        let Ok(response) = self
            .client
            .query()
            .set_query_string(Some(query))
            .send()
            .await
        else {
            return Err(());
        };

        let mut devices: Vec<String> = vec![];

        for row in response.rows() {
            let device = Option::expect(
                row.data[0].scalar_value.clone(),
                "Error in device scalar_value!",
            );

            devices.push(device);
        }

        Ok(devices)
    }

    fn process_services(
        &self,
        column_info: &[ColumnInfo],
        rows: &[Row],
    ) -> Result<Vec<Service>, ()> {
        let mut temp_data: Vec<Data> = vec![];
        let mut humidity_data: Vec<Data> = vec![];

        for row in rows {
            let len = row.data.len();

            let mut data_type: String = String::from("");
            let mut data_device: String = String::from("");
            let mut data_value: String = String::from("");
            let mut data_time: String = String::from("");

            for index in 0..len {
                let info = &column_info[index];

                if let Some(info) = &info.name {
                    if info == "type" {
                        if let Some(scalar_value) = &row.data[index].scalar_value {
                            data_type = scalar_value.clone()
                        }
                    }

                    if info == "device" {
                        if let Some(scalar_value) = &row.data[index].scalar_value {
                            data_device = scalar_value.clone()
                        }
                    }

                    if info == "measure_value::double" {
                        if let Some(scalar_value) = &row.data[index].scalar_value {
                            data_value = scalar_value.clone()
                        }
                    }

                    if info == "time" {
                        if let Some(scalar_value) = &row.data[index].scalar_value {
                            data_time = scalar_value.clone()
                        }
                    }
                };
            }

            if data_type == "0" {
                temp_data.push(Data {
                    time: data_time,
                    value: data_value
                        .parse::<f64>()
                        .map(|n| n + 1.5)
                        .expect("Error parsing data_value to f64!"),
                })
            }
        }

        Ok(vec![Service {
            service: String::from("1"),
            data: temp_data,
        }])
    }
}
