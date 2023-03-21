use std::sync::Arc;

use async_trait::async_trait;
use server_domain::{
    error::DomainResult,
    sys::{SystemAccessor, SystemInfo},
};
use shaku::{Component, Interface};

#[async_trait]
pub trait SystemUsecase: Interface {
    async fn get_sys_info(&self) -> DomainResult<SystemInfo>;
}

#[derive(Component)]
#[shaku(interface=SystemUsecase)]
pub struct SystemUsecaseImpl {
    #[shaku(inject)]
    sys_accessor: Arc<dyn SystemAccessor>,
}

#[async_trait]
impl SystemUsecase for SystemUsecaseImpl {
    async fn get_sys_info(&self) -> DomainResult<SystemInfo> {
        self.sys_accessor.get_info().await
    }
}
