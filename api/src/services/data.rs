use log::{error, info};
use tonic::Status;

use crate::{
    infra::timeseries::Timeseries,
    server::{
        io_t_data_services_server::IoTDataServices, IoTData, ListIoTDataRequest,
        ListIoTDataResponse,
    },
};

pub struct IoTDataServicesImpl {
    client: Timeseries,
}

impl IoTDataServicesImpl {
    pub fn new(client: Timeseries) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn list_io_t_data(
        &self,
        _req: tonic::Request<ListIoTDataRequest>,
    ) -> Result<tonic::Response<ListIoTDataResponse>, tonic::Status> {
        let Ok(result) = self.client.get_all_iot_data().await else {
            error!("Erro on get all iot data!");
            return Err(Status::new(tonic::Code::Internal, "Internal error!"));
        };

        info!("{:?}", result);

        // info!("{:?}", result.column_info());
        // info!("{:?}", result.rows.last());

        Ok(tonic::Response::new(ListIoTDataResponse { data: result }))

        // Ok(tonic::Response::new(ListIoTDataResponse { data: vec![] }))
    }
}
