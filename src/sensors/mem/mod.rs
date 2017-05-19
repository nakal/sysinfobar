extern crate libc;

use self::libc::{size_t, c_void};
use std::ptr;
use std::mem;

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

    pub fn fetch() -> MemInfo {
        let mut mib: [libc::c_int; 2] = [ libc::CTL_VM, VM_UVMEXP ];
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

    pub fn memused(&self) -> u32 {
        (((self.npages - self.free) * 100) / self.npages) as u32
    }

    pub fn swpused(&self) -> u32 {
        ((self.swpgonly * 100) / self.swpages) as u32
    }
}
