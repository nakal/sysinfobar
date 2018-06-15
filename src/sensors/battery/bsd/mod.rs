extern crate libc;
use std::ffi::{CString};
use self::libc::{c_uchar,c_uint};
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
        let nm = CString::new("/dev/apm").unwrap();
        let nmc = nm.as_ptr();
        let req = 0x40204103;
        let mut buf = PowerInfo::new();
        unsafe {
            let fd = libc::open(nmc, libc::O_RDONLY);
            libc::ioctl(fd, req, &mut buf);
            libc::close(fd);
        }
        buf
    }
}
