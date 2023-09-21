extern crate libc;

use std::fs;

pub struct MemInfo {
    mem_total: u64,
    mem_free: u64,
    swap_total: u64,
    swap_free: u64,
}

impl MemInfo {
    fn new() -> MemInfo {
        MemInfo {
            mem_total: 0,
            mem_free: 0,
            swap_total: 0,
            swap_free: 0,
        }
    }

    pub fn fetch() -> MemInfo {
        let mut info = MemInfo::new();
        let contents = fs::read_to_string("/proc/meminfo")
                    .expect("Cannot access /proc/meminfo");
        for line in contents.lines() {
            let v: Vec<&str> = line.split_whitespace().collect();
            if v[0].eq("MemTotal:") {
                match v[1].parse::<u64>() {
                    Ok(v) => info.mem_total = v,
                    Err(_) => info.mem_total = 0,
                };
            }
            if v[0].eq("MemFree:") {
                match v[1].parse::<u64>() {
                    Ok(v) => info.mem_free = v,
                    Err(_) => info.mem_free = 0,
                };
            }
            if v[0].eq("SwapTotal:") {
                match v[1].parse::<u64>() {
                    Ok(v) => info.swap_total = v,
                    Err(_) => info.swap_total = 0,
                };
            }
            if v[0].eq("SwapFree:") {
                match v[1].parse::<u64>() {
                    Ok(v) => info.swap_free = v,
                    Err(_) => info.swap_free = 0,
                };
            }
        }

        info
    }

    pub fn memused(&self) -> u32 {
        if self.mem_total == 0 {
            0
        } else {
            (100 - ((self.mem_free * 100) / self.mem_total)) as u32
        }
    }

    pub fn swpused(&self) -> u32 {
        if self.swap_total == 0 {
            0
        } else {
            (100 - ((self.swap_free * 100) / self.swap_total)) as u32
        }
    }
}
