use tonic::{Request, Response, Status};

use crate::{
    infra::timeseries::Timeseries,
    server::{io_t_services_server::IoTServices, ListAllServicesRequest, ListAllServicesResponse},
};

pub struct ServicesImpl {
    client: Timeseries,
}

#[tonic::async_trait]
impl IoTServices for ServicesImpl {
    async fn list_all_services(
        &self,
        _request: Request<ListAllServicesRequest>,
    ) -> Result<Response<ListAllServicesResponse>, Status> {
        let Ok(devices) = self.client.gel_all_services().await else {
            return Err(Status::new(tonic::Code::Internal, "Internal error!"));
        };

        Ok(Response::new(ListAllServicesResponse { devices }))
    }
}

impl ServicesImpl {
    pub fn new(client: Timeseries) -> Self {
        Self { client }
    }
}
