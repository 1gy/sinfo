use std::{net::SocketAddr, sync::Arc};

use axum::Server;
use controller::create_router;
use server_application::usecase::{cpu::CpuUsecase, sys::SystemUsecase};
use server_domain::{console::ConsoleAccessor, cpu::CpuAccessor, sys::SystemAccessor};
use shaku::HasComponent;

mod controller;

pub trait AppModule:
    HasComponent<dyn CpuAccessor>
    + HasComponent<dyn SystemAccessor>
    + HasComponent<dyn ConsoleAccessor>
    + HasComponent<dyn CpuUsecase>
    + HasComponent<dyn SystemUsecase>
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
