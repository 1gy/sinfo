use std::sync::Arc;

use async_trait::async_trait;
use server_domain::cpu::CpuInfoAccessor;
use shaku::{Component, Interface};

#[async_trait]
pub trait CpuUsecase: Interface {
    async fn get_cpu_info(&self);
}

#[derive(Component)]
#[shaku(interface=CpuUsecase)]
pub struct CpuUsecaseImpl {
    #[shaku(inject)]
    cpu_info_accessor: Arc<dyn CpuInfoAccessor>,
}

#[async_trait]
impl CpuUsecase for CpuUsecaseImpl {
    async fn get_cpu_info(&self) {
        self.cpu_info_accessor.get().await;
    }
}
