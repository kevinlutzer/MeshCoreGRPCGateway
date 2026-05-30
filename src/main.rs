mod server;

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::{error, info};

use meshcore_rs::MeshCore;

pub mod meshcore_proto {
    tonic::include_proto!("meshcore");
}

use meshcore_proto::mesh_core_service_server::MeshCoreServiceServer;
use server::MeshCoreServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise tracing; honour RUST_LOG env var (default: info).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // ── Configuration ────────────────────────────────────────────────────────
    let serial_port =
        std::env::var("MESHCORE_SERIAL_PORT").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());

    let baud_rate: u32 = std::env::var("MESHCORE_BAUD_RATE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(115_200);

    let listen_addr =
        std::env::var("GRPC_LISTEN_ADDR").unwrap_or_else(|_| "[::]:50051".to_string());

    info!(
        port = %serial_port,
        baud = baud_rate,
        listen = %listen_addr,
        "Starting meshcore-grpc"
    );

    // ── Initialise MeshCore SDK over serial ──────────────────────────────────
    let meshcore = MeshCore::serial(&serial_port, baud_rate)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to open serial connection");
            e
        })?;

    // Verify connectivity and log the device name.
    let self_info = meshcore
        .commands()
        .lock()
        .await
        .send_appstart()
        .await
        .map_err(|e| {
            error!(error = %e, "send_appstart failed – is the device connected?");
            e
        })?;

    info!(device = %self_info.name, "Connected to MeshCore device");

    // Shared command handler passed into every RPC handler.
    let commands: Arc<Mutex<meshcore_rs::commands::CommandHandler>> = meshcore.commands();

    // ── gRPC server ──────────────────────────────────────────────────────────
    let addr = listen_addr.parse().map_err(|e| {
        error!(addr = %listen_addr, error = ?e, "Invalid listen address");
        Box::<dyn std::error::Error>::from(format!("invalid GRPC_LISTEN_ADDR: {e}"))
    })?;

    let service = MeshCoreServiceImpl::new(commands);

    info!(%addr, "gRPC server listening");

    Server::builder()
        .add_service(MeshCoreServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
