use async_trait::async_trait;
use raw_cpuid::CpuId;
use server_domain::{
    cpu::{CpuAccessor, CpuInfo, ProcessorInfo},
    error::DomainResult,
};
use shaku::Component;

use crate::error::InfrastructureError;

#[derive(Component)]
#[shaku(interface=CpuAccessor)]
pub struct CpuAccessorImpl;

#[async_trait]
impl CpuAccessor for CpuAccessorImpl {
    async fn get_info(&self) -> DomainResult<CpuInfo> {
        let processor = {
            let cpuid = CpuId::new();

            let vendor = cpuid
                .get_vendor_info()
                .ok_or(InfrastructureError::UnsupportedCpu().into())?
                .to_string();

            let brand = cpuid
                .get_processor_brand_string()
                .ok_or(InfrastructureError::UnsupportedCpu().into())?
                .as_str()
                .into();

            let feature_info = cpuid
                .get_feature_info()
                .ok_or(InfrastructureError::UnsupportedCpu().into())?;
            let family = feature_info.family_id();
            let model = feature_info.model_id();

            ProcessorInfo {
                vendor,
                brand,
                family,
                model,
            }
        };

        Ok(CpuInfo { processor })
    }
}
