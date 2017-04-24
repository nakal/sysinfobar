pub mod xmobar;
pub use super::battery::{PowerInfo};

pub struct StatusData {
    pub power_info: PowerInfo,
    pub time: String,
}

pub trait Output {
    fn refresh(&StatusData);
}
