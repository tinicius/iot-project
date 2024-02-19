pub mod server {
    tonic::include_proto!("server");
}

use std::error::Error;

use log::info;
use tonic::transport::Server;

use crate::{
    infra::{timeseries::Timeseries, tms_connection::TimestreamConnection},
    server::io_t_services_server::IoTServicesServer,
    services::service::ServicesImpl,
};

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

    let service = ServicesImpl::new(timeseries);

    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(IoTServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
