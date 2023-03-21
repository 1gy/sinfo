use async_trait::async_trait;
use server_domain::cpu::CpuInfoAccessor;
use shaku::Component;

#[derive(Component)]
#[shaku(interface=CpuInfoAccessor)]
pub struct CpuInfoAccessorImpl;

#[async_trait]
impl CpuInfoAccessor for CpuInfoAccessorImpl {
    async fn get(&self) {
        println!("gettttt")
    }
}
