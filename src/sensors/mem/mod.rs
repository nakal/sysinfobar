extern crate libc;

use self::libc::{size_t, c_void, c_int};
#[cfg(target_os = "freebsd")]
use self::libc::{c_char, c_uint};
use std::ptr;
use std::mem;

#[cfg(target_os = "openbsd")]
const VM_UVMEXP: libc::c_int = 4;

#[repr(C,packed)]
pub struct MemInfo {
    ignore1: [i32; 3],
    npages: i32,
    free: i32,
    ignore2: [i32; 21],
    swpages: i32,
    ignore3: i32,
    swpgonly: i32,
    ignore4: [i32; 57],
}

impl MemInfo {
    fn new() -> MemInfo {
        MemInfo {
            ignore1: [0; 3],
            npages: 0,
            free: 0,
            ignore2: [0; 21],
            swpages: 0,
            ignore3: 0,
            swpgonly: 0,
            ignore4: [0; 57],
        }
    }

    #[cfg(target_os = "openbsd")]
    pub fn fetch() -> MemInfo {
        let mut mib: [c_int; 2] = [ libc::CTL_VM, VM_UVMEXP ];
        #[cfg(target_os = "freebsd")]
        let mut mib: [c_int; 4] = MemInfo::sysctl_mib("");
        let mut info = MemInfo::new();
        let buf: *mut c_void = info.ignore1.as_mut_ptr() as *mut c_void;
        let mut size = mem::size_of::<MemInfo>() as size_t;
        let outbuf: *mut c_void = ptr::null_mut() as *mut c_void;
        unsafe {
            libc::sysctl(&mut mib[0], 2, buf,
                         &mut size, outbuf, 0);
        }
        info
    }

    #[cfg(target_os = "freebsd")]
    pub fn fetch() -> MemInfo {
        let mib_free: [c_int; 4] = MemInfo::sysctl_mib("vm.stats.vm.v_free_count");
        let mib_inact: [c_int; 4] = MemInfo::sysctl_mib("vm.stats.vm.v_inactive_count");
        let mib_total: [c_int; 4] = MemInfo::sysctl_mib("vm.stats.vm.v_page_count");
        let mut buf: [c_uint; 1] = [ 0 ];
        let buf_raw: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let mut size = mem::size_of::<c_uint>();
        let outbuf: *mut c_void = ptr::null_mut() as *mut c_void;
        let mut info = MemInfo::new();
        unsafe {
            libc::sysctl(&mib_total[0], mib_total.len() as c_uint, buf_raw, &mut size, outbuf, 0);
            info.npages = buf[0] as i32;
            libc::sysctl(&mib_free[0], mib_free.len() as c_uint, buf_raw, &mut size, outbuf, 0);
            info.free = buf[0] as i32;
            libc::sysctl(&mib_inact[0], mib_inact.len() as c_uint, buf_raw, &mut size, outbuf, 0);
            info.free += buf[0] as i32;
        }
        info
    }

    #[cfg(target_os = "freebsd")]
    fn sysctl_mib(sctlname: &'static str) -> [c_int; 4] {
        let mut mib: [c_int; 4] = [0, 0, 0, 0];
        unsafe {
            let mut size: size_t = mib.len();
            let name: *const c_char =
                sctlname.as_ptr() as *const c_char;
            libc::sysctlnametomib(name, &mut mib[0], &mut size);
        }
        mib
    }

    pub fn memused(&self) -> u32 {
        if self.npages == 0 {
            0
        } else {
            (((self.npages - self.free) * 100) / self.npages) as u32
        }
    }

    pub fn swpused(&self) -> u32 {
        if self.swpages == 0 {
            0
        } else {
            ((self.swpgonly * 100) / self.swpages) as u32
        }
    }
}
