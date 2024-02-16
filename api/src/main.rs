pub mod server {
    tonic::include_proto!("server");
}

use std::error::Error;

use log::info;
use tonic::transport::Server;

use crate::infra::{timeseries::Timeseries, tms_connection::TimestreamConnection};
use crate::server::io_t_data_services_server::IoTDataServicesServer;
use crate::services::data::IoTDataServicesImpl;

pub mod infra;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::from_filename("./.env").expect("Failed to read .env");
    env_logger::init();

    info!("Starting gRPC server!");

    let client = TimestreamConnection::new()
        .connect()
        .await
        .expect("Error on create timestream connection!");

    let timeseries = Timeseries::new(client);

    // let _ = timeseries.get_all_iot_data().await;

    let service = IoTDataServicesImpl::new(timeseries);

    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(IoTDataServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
