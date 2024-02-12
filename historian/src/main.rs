use infra::{
    consumer::ConsumerService, rmq_connection::RabbitMQConnection, timeseries::Timeseries,
    tms_connnection::TimestreamConnection,
};

use serde::Deserialize;
use services::data_convert::DataConvert;

#[derive(Deserialize, Debug)]
pub struct MQTTMessage {
    pub device: String,
    pub value: String,
    pub typ: String,
}

mod infra;
mod services;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    let (_connection, channel) = RabbitMQConnection::new()
        .connect()
        .await
        .expect("Error on create rabbitmq connection!");

    let data_convert = DataConvert::new();

    let timestream_connection = TimestreamConnection::new()
        .connect()
        .await
        .expect("Erro on create aws connection!");

    let timeseries = Timeseries::new(timestream_connection, "iot-database".to_string());

    let consumer = ConsumerService::new(channel, data_convert, timeseries);

    let _ = consumer.listen().await;

    return Ok(());

    // let mut counter = 1;

    // while let Some(event) = consumer.next().await {
    //     println!("-------- {}", counter);
    //     counter = counter + 1;

    //     match event {
    //         Ok(delivery) => {
    //             info!("{:?}", delivery.data);

    //             continue;

    //             let Ok(msg) = serde_json::from_slice::<MQTTMessage>(delivery.data.as_slice())
    //             else {
    //                 error!("Error on deseralize data!");
    //                 continue;
    //             };

    //             info!("{:?}", msg);

    //             if (msg.value.parse::<f32>().unwrap() < 10.0) {
    //                 match delivery
    //                     .nack(BasicNackOptions {
    //                         requeue: true,
    //                         ..Default::default()
    //                     })
    //                     .await
    //                 {
    //                     Ok(_) => info!("NACK ok!"),
    //                     Err(err) => {
    //                         error!("NACK error!");
    //                         error!("{:?}", err);
    //                     }
    //                 }
    //             } else {
    //                 match delivery.ack(BasicAckOptions::default()).await {
    //                     Ok(_) => info!("ack ok!"),
    //                     Err(err) => {
    //                         error!("ACK error!");
    //                         error!("{:?}", err);
    //                     }
    //                 };
    //             }
    //         }
    //         Err(error) => {
    //             error!("Error to receive rabbitmq message!");
    //             error!("{:?}", error);

    //             continue;
    //         }
    //     }
    // }

    // Ok(())
}
