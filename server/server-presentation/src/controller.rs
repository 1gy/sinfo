use axum::{routing::get, Json, Router};
use serde::Serialize;

use self::cpu::cpu_info;
use crate::AppState;

pub mod cpu;

pub(crate) fn create_router(state: AppState) -> Router {
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
