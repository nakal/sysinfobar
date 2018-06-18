extern crate libc;

use self::libc::{size_t, c_void, c_int, c_uint};
#[cfg(target_os = "freebsd")]
use self::libc::{c_char};
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
        #[cfg(target_os = "openbsd")]
        let mut mib: [c_int; 2] = [ libc::CTL_KERN, libc::KERN_CPTIME ];
        #[cfg(target_os = "freebsd")]
        let mut mib: [c_int; 4] = CPUInfo::cp_time_mib();
        let mut info = CPUInfo::new();
        unsafe {
            let buf: *mut c_void = info.state.as_mut_ptr() as *mut c_void;
            let mut size = mem::size_of::<CPUInfo>() as size_t;
            let outbuf: *mut c_void = ptr::null_mut() as *mut c_void;
            libc::sysctl(&mut mib[0], mib.len() as c_uint, buf,
                         &mut size, outbuf, 0);
        }
        info
    }

    #[cfg(target_os = "freebsd")]
    fn cp_time_mib() -> [c_int; 4] {
        let mut mib: [c_int; 4] = [0, 0, 0, 0];
        unsafe {
            let mut size: size_t = mib.len();
            let sctlname = "kern.cp_time";
            let name: *const c_char =
                sctlname.as_ptr() as *const c_char;
            libc::sysctlnametomib(name, &mut mib[0], &mut size);
        }
        mib
    }

    pub fn ncpus() -> u32 {
        unsafe {
            libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as u32
        }
    }

    pub fn get_busy(&self, old: &CPUInfo) -> u32 {
        let mut diff = unsafe {
            self.state.clone()
        };
        let mut sum = 0;
        for i in 0..CPUSTATES {
            diff[i] = self.state[i] - old.state[i];
            sum += diff[i];
        }
        if sum == 0 { sum += 1 }
        (((sum - diff[CPUSTATES - 1]) * 100) / sum) as u32
    }
}
