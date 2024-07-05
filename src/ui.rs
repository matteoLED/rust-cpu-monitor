use eframe::egui::{CentralPanel, Context};

pub struct App {
    monitor: crate::monitor::SystemMonitor,
    cpu_usage: f32,
    memory_usage: u64,
    total_memory: u64,
    available_memory: u64,
    fps: f64,
}

impl App {
    pub fn new() -> Self {
        let mut monitor = crate::monitor::SystemMonitor::new();
        monitor.refresh();
        let cpu_usage = monitor.get_cpu_usage();
        let memory_usage = monitor.get_memory_usage();
        let total_memory = monitor.get_total_memory();
        let available_memory = monitor.get_available_memory();
        let fps = monitor.get_fps();

        App {
            monitor,
            cpu_usage,
            memory_usage,
            total_memory,
            available_memory,
            fps,
        }
    }

    pub fn update(&mut self) {
        self.monitor.refresh();
        self.cpu_usage = self.monitor.get_cpu_usage();
        self.memory_usage = self.monitor.get_memory_usage();
        self.total_memory = self.monitor.get_total_memory();
        self.available_memory = self.monitor.get_available_memory();
        self.monitor.update_fps();
        self.fps = self.monitor.get_fps();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.update();
        CentralPanel::default().show(ctx, |ui| {
            let cpu_usage = if self.cpu_usage.is_nan() {
                0.0
            } else {
                self.cpu_usage
            };
            ui.label(format!("CPU Usage: {:.2}%", cpu_usage));
            let used_memory = self.total_memory - self.available_memory;
            let memory_usage_percentage = if self.total_memory > 0 {
                (used_memory as f64 / self.total_memory as f64) * 100.0
            } else {
                0.0
            };
            ui.label(format!("Memory Usage: {:.2}%", memory_usage_percentage));
            ui.label(format!("Total Memory: {} KB", self.total_memory));
            ui.label(format!("FPS: {:.2}", self.fps));
        });
    }
}
