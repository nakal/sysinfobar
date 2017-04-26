pub mod xmobar;
pub use super::battery::{PowerInfo};
pub use super::cpu::{CPUInfo};

pub struct StatusData {
    pub power_info: PowerInfo,
    pub time: String,
    pub load: u32,
    pub cpus: u32,
}

pub trait Output {
    fn refresh(&StatusData);
}
