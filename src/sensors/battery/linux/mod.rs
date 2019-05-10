use std::io::{BufRead, BufReader};
use std::fs::File;
use super::PowerInfo;

impl PowerInfo {
    fn new() -> PowerInfo {
        PowerInfo {
            battery_powered: false,
            battery_state: 0,
            ac_state: 0,
            battery_life: 0,
            spare1: 0,
            minutes_left: 0,
            supports_time: false,
            spare2: [0, 0, 0, 0, 0, 0],
        }
    }

    fn fetch_life(&mut self) {
        let f = match File::open("/sys/bus/acpi/drivers/battery/PNP0C0A:00/power_supply/BAT0/capacity") {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut reader = BufReader::new(f);
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Err(_) => return,
            Ok(_) => {},
        }
        self.battery_life = match line.trim_end().parse::<u8>() {
            Err(_) => 0,
            Ok(u) => u,
        };
        self.battery_powered = true;
    }

    fn fetch_powered(&mut self) {
        let f = match File::open("/sys/bus/acpi/drivers/battery/PNP0C0A:00/power_supply/BAT0/status") {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut reader = BufReader::new(f);
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Err(_) => return,
            Ok(_) => {},
        }
        self.ac_state = if line.starts_with("Discharging") {
            0
        } else {
            1
        }
    }

    pub fn fetch() -> PowerInfo {
        let mut info = PowerInfo::new();
        info.fetch_powered();
        info.fetch_life();
        info
    }
}
