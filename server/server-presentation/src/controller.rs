use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::AppState;

use self::{cpu::cpuinfo_router, sys::sysinfo_router};

pub mod cpu;
pub mod sys;

pub(crate) type AppRouter = Router<AppState>;

pub(crate) fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(version))
        .nest("/cpu", cpuinfo_router())
        .nest("/sys", sysinfo_router())
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
