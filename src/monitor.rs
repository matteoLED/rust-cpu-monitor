use std::time::Instant;
use sysinfo::{ProcessorExt, System, SystemExt};

pub struct SystemMonitor {
    system: System,
    last_frame_time: Instant,
    frame_count: u32,
    fps: f64,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let system = System::new_all();
        let last_frame_time = Instant::now();
        SystemMonitor {
            system,
            last_frame_time,
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
    }

    pub fn get_cpu_usage(&self) -> f32 {
        let cpu_usage = self.system.global_processor_info().cpu_usage();
        if cpu_usage.is_nan() {
            0.0
        } else {
            cpu_usage
        }
    }

    pub fn get_memory_usage(&self) -> u64 {
        self.system.used_memory()
    }

    pub fn get_total_memory(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn get_available_memory(&self) -> u64 {
        self.system.available_memory()
    }

    pub fn update_fps(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(self.last_frame_time).as_secs_f64();

        if duration >= 1.0 {
            self.fps = self.frame_count as f64 / duration;
            self.frame_count = 0;
            self.last_frame_time = now;
        }
    }

    pub fn get_fps(&self) -> f64 {
        self.fps
    }
}
