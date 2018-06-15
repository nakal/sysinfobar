#[cfg(target_os = "freebsd")]
mod bsd;
#[cfg(target_os = "freebsd")]
pub use self::bsd::*;

#[cfg(target_os = "openbsd")]
mod bsd;
#[cfg(target_os = "openbsd")]
pub use self::bsd::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

extern crate libc;
use self::libc::{c_uchar,c_uint};

#[repr(C)]
#[derive(Debug)]
pub struct PowerInfo {
    pub battery_powered: bool,
    pub battery_state: c_uchar,
    pub ac_state: c_uchar,
    pub battery_life: c_uchar,
    spare1: c_uchar,
    pub minutes_left: c_uint,
    spare2: [c_uint; 6],
}
