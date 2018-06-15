extern crate libc;
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
            spare2: [0, 0, 0, 0, 0, 0],
        }
    }

    pub fn fetch() -> PowerInfo {
        PowerInfo::new()
    }
}
