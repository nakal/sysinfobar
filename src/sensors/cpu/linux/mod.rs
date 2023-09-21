use std::io::{BufRead, BufReader};
use std::fs::File;

const CPUSTATES: usize = 4;

#[repr(C,packed)]
pub struct CPUInfo {
    state: [u64; CPUSTATES],
}

impl CPUInfo {
    fn new() -> CPUInfo {
        CPUInfo {
            state: [0, 0, 0, 0],
        }
    }

    pub fn fetch() -> CPUInfo {
        let mut info = CPUInfo::new();
        let f = match File::open("/proc/stat") {
            Err(_) => return info,
            Ok(v) => v,
        };

        let reader = BufReader::new(f);
        for line in reader.lines() {
            let l = match line {
                Err(_) => return info,
                Ok(l) => l
            };
            if l.starts_with("cpu ") {
                let v: Vec<&str> = l.split_whitespace().collect();
                if v.len() < CPUSTATES {
                    return info
                }
                for i in 0..CPUSTATES {
                    info.state[i] = match v[i + 1].parse::<u64>() {
                        Err(_) => return info,
                        Ok(u) => u,
                    };
                }
                break;
            }
        }
        info
    }

    pub fn ncpus() -> u32 {
        // fake 1 CPU, because we have accumulated results in /proc/stat
        1
    }

    pub fn get_busy(&self, old: &CPUInfo) -> u32 {
        let mut sum = 0;
        for i in 0..CPUSTATES {
            sum += self.state[i] - old.state[i];
        }
        let idle = self.state[3] - old.state[3];
        if sum > 0 {
            match (100 * (sum - idle) / sum) as u32 {
                x @ 0..=100 => x,
                _ => 0,
            }
        } else {
            0
        }
    }
}
