use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, routing::get, Json, Router, Server};
use serde::Serialize;
use server_application::usecase::cpu::CpuUsecase;
use server_domain::{console::ConsoleAccessor, cpu::CpuInfoAccessor};
use shaku::HasComponent;

pub trait AppModule:
    HasComponent<dyn CpuInfoAccessor> + HasComponent<dyn ConsoleAccessor> + HasComponent<dyn CpuUsecase>
{
}

#[derive(Clone)]
struct AppState {
    module: Arc<dyn AppModule>,
}

pub async fn start_server(
    addr: &SocketAddr,
    module: impl AppModule,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::bind(addr);

    let state = AppState {
        module: Arc::new(module),
    };

    Ok(server
        .serve(create_router(state).into_make_service())
        .await?)
}

fn create_router(state: AppState) -> Router {
    Router::new().route("/", get(version)).with_state(state)
}

#[derive(Serialize)]
struct VersionInfo {
    name: String,
    version: String,
}

async fn version(State(state): State<AppState>) -> Json<VersionInfo> {
    let module = state.module;
    let cpu_info_accessor: &dyn CpuInfoAccessor = module.resolve_ref();
    let console_accessor: &dyn ConsoleAccessor = module.resolve_ref();
    let cpu_usecase: &dyn CpuUsecase = module.resolve_ref();
    cpu_info_accessor.get().await;
    console_accessor.println();
    cpu_usecase.get_cpu_info().await;

    Json(VersionInfo {
        name: "sinfo".to_string(),
        version: "0.1.0".to_string(),
    })
}
