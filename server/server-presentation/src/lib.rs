use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, routing::get, Json, Router, Server};
use serde::Serialize;
use server_application::usecase::cpu::CpuUsecase;
use server_domain::{console::ConsoleAccessor, cpu::CpuAccessor};
use shaku::HasComponent;

pub trait AppModule:
    HasComponent<dyn CpuAccessor> + HasComponent<dyn ConsoleAccessor> + HasComponent<dyn CpuUsecase>
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
    Router::new()
        .route("/", get(version))
        .route("/cpuinfo", get(cpu_info))
        .with_state(state)
}

#[derive(Serialize)]
struct VersionInfo {
    name: String,
    version: String,
}

async fn version() -> Json<VersionInfo> {
    Json(VersionInfo {
        name: "sinfo".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[derive(Serialize)]
pub struct ProcessorInfo {
    pub vendor: String,
    pub brand: String,
    pub family: u8,
    pub model: u8,
}

#[derive(Serialize)]
pub struct CpuInfo {
    pub processor: ProcessorInfo,
}

impl Into<ProcessorInfo> for server_domain::cpu::ProcessorInfo {
    fn into(self) -> ProcessorInfo {
        ProcessorInfo {
            vendor: self.vendor,
            brand: self.brand,
            family: self.family,
            model: self.model,
        }
    }
}

impl Into<CpuInfo> for server_domain::cpu::CpuInfo {
    fn into(self) -> CpuInfo {
        CpuInfo {
            processor: self.processor.into(),
        }
    }
}

async fn cpu_info(State(state): State<AppState>) -> Json<CpuInfo> {
    let module = state.module;
    let cpu_usecase: &dyn CpuUsecase = module.resolve_ref();
    let cpu_info = cpu_usecase.get_cpu_info().await.unwrap();
    Json(cpu_info.into())
}
