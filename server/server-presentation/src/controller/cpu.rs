use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use server_application::usecase::cpu::CpuUsecase;

use super::AppRouter;
use crate::AppState;

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

async fn get_cpu_info(State(state): State<AppState>) -> Json<CpuInfo> {
    let module = state.module;
    let cpu_usecase: &dyn CpuUsecase = module.resolve_ref();
    let cpu_info = cpu_usecase.get_cpu_info().await.unwrap();
    Json(cpu_info.into())
}

pub(crate) fn cpuinfo_router() -> AppRouter {
    Router::new().route("/", get(get_cpu_info))
}
