use crate::data::{MemSnapshot, PhysRegion};

pub struct App {
    pub mem: MemSnapshot,
    pub regions: Vec<PhysRegion>,
    pub ram_history: Vec<u64>,    // used bytes over time
    pub swap_history: Vec<u64>,   // used swap bytes over time
    pub max_history: usize,
}

impl App {
    pub fn new() -> Self {
        let mem = crate::data::read_meminfo();
        let regions = crate::data::read_iomem();
        Self {
            mem,
            regions,
            ram_history: Vec::new(),
            swap_history: Vec::new(),
            max_history: 120,
        }
    }

    pub fn refresh(&mut self) {
        self.mem = crate::data::read_meminfo();
        self.regions = crate::data::read_iomem();

        self.ram_history.push(self.mem.used);
        self.swap_history.push(self.mem.swap_total - self.mem.swap_free);

        if self.ram_history.len() > self.max_history {
            self.ram_history.remove(0);
        }
        if self.swap_history.len() > self.max_history {
            self.swap_history.remove(0);
        }
    }

    pub fn ram_percent(&self) -> f64 {
        (self.mem.used as f64 / self.mem.total as f64) * 100.0
    }

    pub fn swap_percent(&self) -> f64 {
        let used_swap = self.mem.swap_total - self.mem.swap_free;
        if self.mem.swap_total == 0 {
            0.0
        } else {
            (used_swap as f64 / self.mem.swap_total as f64) * 100.0
        }
    }
}
