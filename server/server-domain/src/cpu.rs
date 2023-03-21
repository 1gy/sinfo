use async_trait::async_trait;
use shaku::Interface;

pub struct ProcessorInfo {
    pub name: String,
}

pub struct CpuInfo {
    pub processor: ProcessorInfo,
}

#[async_trait]
pub trait CpuInfoAccessor: Interface {
    async fn get(&self);
}
