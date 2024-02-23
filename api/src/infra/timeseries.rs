use aws_sdk_timestreamquery::{
    operation::query::QueryOutput,
    types::{ColumnInfo, Row},
    Client,
};
use log::error;

use crate::server::{Data, Device, Service};

pub struct ServiceTimeseriesData {
    device: String,
    r#type: String,
    value: String,
    time: String,
}

pub struct Timeseries {
    client: Client,
}

impl Timeseries {
    pub fn new(client: Client) -> Self {
        Timeseries { client }
    }

    pub async fn gel_all_services(&self) -> Result<Vec<Device>, ()> {
        let query_output = self
            .call_query("select * from \"iot-database\".data order by device")
            .await?;

        self.process_devices(query_output.rows(), query_output.column_info())
    }

    async fn call_query(&self, query: &str) -> Result<QueryOutput, ()> {
        match self
            .client
            .query()
            .set_query_string(Some(query.to_owned()))
            .send()
            .await
        {
            Ok(query_output) => Ok(query_output),
            Err(_) => Err(()),
        }
    }

    fn process_devices(
        &self,
        rows: &[Row],
        columns_info: &[ColumnInfo],
    ) -> Result<Vec<Device>, ()> {
        let mut devices: Vec<Device> = vec![];

        let mut temp_data: Vec<Data> = vec![];
        let mut humidity_data: Vec<Data> = vec![];

        let mut device = String::from("");

        for (index, row) in rows.iter().enumerate() {
            let data = self
                .process_service_timeseries_data(row, columns_info)
                .expect("Error on process data from row and columns!");

            if data.device != device && index != 0 {
                self.push_device(
                    &mut devices,
                    &mut temp_data,
                    &mut humidity_data,
                    &data.device,
                );
            }

            if data.r#type == "0" {
                temp_data.push(Data {
                    time: data.time,
                    value: self.parse_f64(data.value).expect("Error!"),
                })
            } else if data.r#type == "1" {
                humidity_data.push(Data {
                    time: data.time,
                    value: self.parse_f64(data.value).expect("Error!"),
                })
            }

            if index == rows.len() - 1 {
                self.push_device(
                    &mut devices,
                    &mut temp_data,
                    &mut humidity_data,
                    &data.device,
                );
            }

            device = data.device.clone();
        }

        Ok(devices)
    }

    fn process_service_timeseries_data(
        &self,
        row: &Row,
        columns_info: &[ColumnInfo],
    ) -> Result<ServiceTimeseriesData, ()> {
        let mut data: ServiceTimeseriesData = ServiceTimeseriesData {
            device: "".to_owned(),
            r#type: "".to_owned(),
            value: "".to_owned(),
            time: "".to_owned(),
        };

        let len = row.data.len();

        for index in 0..len {
            let Some(column_name) = &columns_info[index].name else {
                return Err(());
            };

            let Some(column_value) = &row.data[index].scalar_value else {
                return Err(());
            };

            match column_name.as_str() {
                "type" => data.r#type = column_value.to_string(),
                "device" => data.device = column_value.to_string(),
                "measure_value::double" => data.value = column_value.to_string(),
                "time" => data.time = column_value.to_string(),
                "measure_name" => continue,
                &_ => {
                    error!("Invalid column name {}!", column_name);
                    return Err(());
                }
            }
        }

        Ok(data)
    }

    fn push_device(
        &self,
        devices: &mut Vec<Device>,
        temp_data: &mut Vec<Data>,
        humidity_data: &mut Vec<Data>,
        actual_device: &String,
    ) {
        let mut services: Vec<Service> = vec![];

        if temp_data.len() > 0 {
            services.push(Service {
                service: "TEMP".to_owned(),
                data: temp_data.clone(),
            });

            temp_data.clear();
        }

        if humidity_data.len() > 0 {
            services.push(Service {
                service: "HUMIDITY".to_owned(),
                data: humidity_data.clone(),
            });

            humidity_data.clear();
        }

        devices.push(Device {
            device: actual_device.clone(),
            services,
        });
    }

    fn parse_f64(&self, str: String) -> Result<f64, ()> {
        match str.parse::<f64>().map(|n| n + 1.5) {
            Ok(value) => Ok(value),
            Err(_) => Err(()),
        }
    }
}
