use async_trait::async_trait;
use shaku::Interface;

use crate::error::DomainResult;

pub struct Cpu {
    pub usage: f32,
}

pub struct SystemInfo {
    pub cpus: Vec<Cpu>,
}

#[async_trait]
pub trait SystemAccessor: Interface {
    async fn get_info(&self) -> DomainResult<SystemInfo>;
}
