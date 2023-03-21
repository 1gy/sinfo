use std::{env, net::SocketAddr};

use server_application::usecase::{cpu::CpuUsecaseImpl, sys::SystemUsecaseImpl};
use server_infrastructure::accessor::{
    console::ConsoleAccessorImpl, cpu::CpuAccessorImpl, sys::SystemAccessorImpl,
};
use server_presentation::{start_server, AppModule};
use shaku::module;

module! {
    pub AppModuleImpl: AppModule {
        components = [
            // infrastructure
            CpuAccessorImpl,
            ConsoleAccessorImpl,
            SystemAccessorImpl,
            // usecase
            CpuUsecaseImpl,
            SystemUsecaseImpl,
       ],
        providers = []
    }
}

fn create_module() -> impl AppModule {
    AppModuleImpl::builder().build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let server_port = env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(55720);

    let addr = SocketAddr::from(([127, 0, 0, 1], server_port));

    println!("server starting: http://{}", addr);
    Ok(start_server(&addr, create_module()).await?)
}
