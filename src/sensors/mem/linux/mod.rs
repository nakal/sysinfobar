extern crate libc;

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
        MemInfo::new()
    }

    pub fn memused(&self) -> u32 {
        0
    }

    pub fn swpused(&self) -> u32 {
        0
    }
}
