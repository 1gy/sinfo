use std::sync::Mutex;

use async_trait::async_trait;
use once_cell::sync::Lazy;
use server_domain::{
    error::DomainResult,
    sys::{Cpu, SystemAccessor, SystemInfo},
};
use shaku::Component;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Component)]
#[shaku(interface=SystemAccessor)]
pub struct SystemAccessorImpl;

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));

#[async_trait]
impl SystemAccessor for SystemAccessorImpl {
    async fn get_info(&self) -> DomainResult<SystemInfo> {
        let mut system = SYSTEM.lock().unwrap();
        system.refresh_cpu();

        let cpus = system
            .cpus()
            .iter()
            .map(|cpu| Cpu {
                usage: cpu.cpu_usage(),
            })
            .collect();

        Ok(SystemInfo { cpus })
    }
}
