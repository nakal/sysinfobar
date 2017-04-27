extern crate libc;

use self::libc::{size_t, c_void};
use std::ptr;
use std::mem;

const CPUSTATES: usize = 5;

#[repr(C,packed)]
pub struct CPUInfo {
    pub state: [isize; CPUSTATES],
}

impl CPUInfo {
    fn new() -> CPUInfo {
        CPUInfo {
            state: [0, 0, 0, 0, 0],
        }
    }

    pub fn fetch() -> CPUInfo {
        let mut mib: [libc::c_int; 2] = [ libc::CTL_KERN, libc::KERN_CPTIME ];
        let mut info = CPUInfo::new();
        let buf: *mut c_void = info.state.as_mut_ptr() as *mut c_void;
        let mut size = mem::size_of::<CPUInfo>() as size_t;
        let outbuf: *mut c_void = ptr::null_mut() as *mut c_void;
        unsafe {
            libc::sysctl(&mut mib[0], 2, buf,
                         &mut size, outbuf, 0);
        }
        info
    }

    pub fn ncpus() -> u32 {
        unsafe {
            libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as u32
        }
    }

    pub fn get_busy(&self, old: &CPUInfo) -> u32 {
        let mut diff = self.state.clone();
        let mut sum = 0;
        for i in 0..CPUSTATES {
            diff[i] = self.state[i] - old.state[i];
            sum += diff[i];
        }
        if sum == 0 { sum += 1 }
        (((sum - diff[CPUSTATES - 1]) * 100) / sum) as u32
    }
}
