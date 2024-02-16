use std::fmt::format;

use aws_sdk_timestreamquery::{
    types::{ColumnInfo, Row},
    Client,
};
use log::{error, info};

use crate::server::IoTData;

pub struct Timeseries {
    client: Client,
}

impl Timeseries {
    pub fn new(client: Client) -> Self {
        Timeseries { client }
    }

    pub async fn get_all_iot_data(&self) -> Result<Vec<IoTData>, ()> {
        let query = String::from("select * from \"iot-database\".data");

        match self
            .client
            .query()
            .set_query_string(Some(query))
            .send()
            .await
        {
            Ok(result) => match self.process_iot_data(result.column_info(), result.rows()) {
                Ok(data) => {
                    info!("{:?}", data);
                    Ok(data)
                }
                Err(_) => Err(()),
            },
            Err(err) => {
                error!("{}", err);
                Err(())
            }
        }
    }

    pub async fn get_iot_data_in_interval(
        &self,
        start_date_ms: u64,
        end_date_ms: u64,
    ) -> Result<Vec<IoTData>, ()> {
        let query = format!("select * from \"iot-database\".data where time >= ago");

        match self
            .client
            .query()
            .set_query_string(Some(query))
            .send()
            .await
        {
            Ok(result) => match self.process_iot_data(result.column_info(), result.rows()) {
                Ok(data) => {
                    info!("{:?}", data);
                    Ok(data)
                }
                Err(_) => Err(()),
            },
            Err(err) => {
                error!("{}", err);
                Err(())
            }
        }
    }

    fn process_iot_data(
        &self,
        column_info: &[ColumnInfo],
        rows: &[Row],
    ) -> Result<Vec<IoTData>, ()> {
        let mut data: Vec<IoTData> = vec![];

        info!("{:?}", rows);
        info!("{:?}", column_info);

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

            data.push(IoTData {
                device: data_device,
                r#type: data_type,
                value: data_value,
                time: data_time,
            })
        }

        Ok(data)
    }
}
