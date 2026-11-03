use procfs::{FromRead, Meminfo};

pub struct MemSnapshot {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

pub fn read_meminfo() -> MemSnapshot {
    // `Meminfo::from_file` comes from the `FromRead` trait.
    let mi = Meminfo::from_file("/proc/meminfo").expect("failed to read /proc/meminfo");
    let total = mi.mem_total;
    let available = mi.mem_available.unwrap_or(0);
    let swap_total = mi.swap_total;
    let swap_free = mi.swap_free;

    MemSnapshot {
        total,
        available,
        used: total.saturating_sub(available),
        swap_total,
        swap_free,
    }
}

pub struct PhysRegion {
    pub start: u64,
    pub end: u64,
    pub label: String,
}

pub fn read_iomem() -> Vec<PhysRegion> {
    // `PhysicalMemoryMap` has fields `address: (u64, u64)` and `name: String`.
    let regions = procfs::iomem().unwrap_or_default();
    regions
        .into_iter()
        .map(|(_, map)| {
            let (start, end) = map.address;
            PhysRegion {
                start,
                end,
                label: map.name,
            }
        })
        .collect()
}
