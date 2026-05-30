use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::{error, info};

mod contact;
mod healtcheck;
mod message;
mod util;

use meshcore_rs::commands::CommandHandler;

use crate::server::message::{receive_message, send_message};

use crate::meshcore_proto::{
    HealthcheckRequest, HealthcheckResponse, ReceiveMessageRequest, ReceiveMessageResponse,
    ResetRequest, ResetResponse, SendMessageRequest, SendMessageResponse,
    mesh_core_service_server::MeshCoreService as MeshCoreServiceGrpc,
};

pub struct MeshCoreService {
    commands: Arc<Mutex<CommandHandler>>,
}

impl MeshCoreService {
    pub fn new(commands: &Arc<Mutex<CommandHandler>>) -> Self {
        Self {
            commands: commands.clone(),
        }
    }
}

#[tonic::async_trait]
impl MeshCoreServiceGrpc for MeshCoreService {
    async fn receive_message(
        &self,
        _request: Request<ReceiveMessageRequest>,
    ) -> Result<Response<ReceiveMessageResponse>, Status> {
        receive_message(&self.commands, _request).await
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        send_message(&self.commands, request).await
    }

    async fn reset(&self, _: Request<ResetRequest>) -> Result<Response<ResetResponse>, Status> {
        Ok(Response::new(ResetResponse {}))
    }

    async fn create_contact(
        &self,
        request: Request<crate::meshcore_proto::CreateContactRequest>,
    ) -> Result<Response<crate::meshcore_proto::CreateContactResponse>, Status> {
        contact::create_contact(&self.commands, request).await
    }

    async fn search_contact(
        &self,
        request: Request<crate::meshcore_proto::SearchContactRequest>,
    ) -> Result<Response<crate::meshcore_proto::SearchContactResponse>, Status> {
        contact::search_contact(&self.commands, request).await
    }

    async fn delete_contact(
        &self,
        request: Request<crate::meshcore_proto::DeleteContactRequest>,
    ) -> Result<Response<crate::meshcore_proto::DeleteContactResponse>, Status> {
        contact::delete_contact(&self.commands, request).await
    }

    async fn healthcheck(
        &self,
        _request: Request<HealthcheckRequest>,
    ) -> Result<Response<HealthcheckResponse>, Status> {
        info!("Healthcheck");

        let cmd = self.commands.lock().await;
        match cmd.send_appstart().await {
            Ok(info) => {
                info!(device = %info.name, "Healthcheck OK");
                Ok(Response::new(HealthcheckResponse {
                    ok: true,
                    device_name: info.name,
                    error: String::new(),
                }))
            }
            Err(e) => {
                error!(error = %e, "Healthcheck failed");
                Ok(Response::new(HealthcheckResponse {
                    ok: false,
                    device_name: String::new(),
                    error: e.to_string(),
                }))
            }
        }
    }
}
