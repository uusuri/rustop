use sysinfo::System;
use nvml_wrapper::Nvml;

pub struct Monitor {
    system: System,
    nvml: Option<Nvml>,
}

pub struct CpuInfo {
    pub name: String,
    pub frequency: u64,
    pub usage: f32,
}

pub struct GpuInfo {
    pub name: String,
    pub memory_total: u64,
    pub memory_used: u64,
    pub usage: u32,
}

pub struct RamInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

pub struct Stats {
    pub cpu: Vec<CpuInfo>,
    pub ram: RamInfo,
    pub gpu: Option<GpuInfo>,
}

impl Monitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu_usage();

        let nvml = Nvml::init().ok();

        Self { system, nvml }
    }

    pub fn get_gpu_info(&self) -> Option<GpuInfo> {
        let nvml = self.nvml.as_ref()?;
        let device = nvml.device_by_index(0).ok()?;

        let memory = device.memory_info().ok()?;
        let utilization = device.utilization_rates().ok()?;

        Some(GpuInfo {
            name: device.name().ok()?,
            memory_total: memory.total,
            memory_used: memory.used,
            usage: utilization.gpu,
        })
    }

    pub fn update(&mut self) -> Stats {
        self.system.refresh_cpu_frequency();
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        let cpu = self.system
            .cpus()
            .iter()
            .map(|cpu| CpuInfo {
                name: cpu.name().to_string(),
                frequency: cpu.frequency(),
                usage: cpu.cpu_usage(),
            })
            .collect();

        let ram = RamInfo {
            total: self.system.total_memory(),
            used: self.system.used_memory(),
            available: self.system.available_memory(),
        };

        let gpu = self.get_gpu_info();

        Stats { cpu, ram, gpu }
    }
}

fn main () {
    println!("Hello, world!");
}