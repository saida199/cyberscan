use sysinfo::{System, SystemExt, CpuExt, ProcessExt};

pub struct SystemData {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub process_count: usize,
}

pub fn collect_system_data() -> SystemData {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let process_count = sys.processes().len();

    SystemData {
        cpu_usage,
        total_memory,
        used_memory,
        process_count,
    }
}
