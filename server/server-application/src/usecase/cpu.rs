use std::sync::Arc;

use async_trait::async_trait;
use server_domain::{
    cpu::{CpuAccessor, CpuInfo},
    error::DomainResult,
};
use shaku::{Component, Interface};

#[async_trait]
pub trait CpuUsecase: Interface {
    async fn get_cpu_info(&self) -> DomainResult<CpuInfo>;
}

#[derive(Component)]
#[shaku(interface=CpuUsecase)]
pub struct CpuUsecaseImpl {
    #[shaku(inject)]
    cpu_accessor: Arc<dyn CpuAccessor>,
}

#[async_trait]
impl CpuUsecase for CpuUsecaseImpl {
    async fn get_cpu_info(&self) -> DomainResult<CpuInfo> {
        self.cpu_accessor.get_info().await
    }
}
