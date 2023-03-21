use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use server_application::usecase::sys::SystemUsecase;

use crate::AppState;

use super::AppRouter;

#[derive(Serialize)]
pub struct Cpu {
    pub usage: f32,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub cpus: Vec<Cpu>,
}

impl Into<Cpu> for server_domain::sys::Cpu {
    fn into(self) -> Cpu {
        Cpu { usage: self.usage }
    }
}

impl Into<SystemInfo> for server_domain::sys::SystemInfo {
    fn into(self) -> SystemInfo {
        SystemInfo {
            cpus: self.cpus.into_iter().map(|c| c.into()).collect(),
        }
    }
}

async fn get_sys_info(State(state): State<AppState>) -> Json<SystemInfo> {
    let module = state.module;
    let sys_usecase: &dyn SystemUsecase = module.resolve_ref();
    let sys_info = sys_usecase.get_sys_info().await.unwrap();
    Json(sys_info.into())
}

pub(crate) fn sysinfo_router() -> AppRouter {
    Router::new().route("/", get(get_sys_info))
}
