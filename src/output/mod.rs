pub mod xmobar;
pub use super::battery::{PowerInfo};
pub use super::cpu::{CPUInfo};

pub struct StatusData {
    pub power_info: PowerInfo,
    pub time: String,
    pub load: u32,
    pub cpus: u32,
    pub memused: u32,
    pub swpused: u32,
    pub net_rx: u32,
    pub net_tx: u32,
}

pub trait Output {
    fn refresh(&StatusData);
}
