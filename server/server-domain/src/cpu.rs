use async_trait::async_trait;
use shaku::Interface;

use crate::error::DomainResult;

pub struct ProcessorInfo {
    pub vendor: String,
    pub brand: String,
    pub family: u8,
    pub model: u8,
}

pub struct CpuInfo {
    pub processor: ProcessorInfo,
}

#[async_trait]
pub trait CpuAccessor: Interface {
    async fn get_info(&self) -> DomainResult<CpuInfo>;
}
