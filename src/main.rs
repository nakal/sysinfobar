mod battery;
mod time;
mod cpu;
mod mem;
mod output;

use output::xmobar::{Xmobar};
use output::Output;

fn main() {
    let ncpus = cpu::CPUInfo::ncpus();
    let mut old_cpuinfo = cpu::CPUInfo::fetch();
    loop {
        let cpuinfo = cpu::CPUInfo::fetch();
        let meminfo = mem::MemInfo::fetch();
        let data = output::StatusData {
            power_info: battery::PowerInfo::fetch(),
            time: time::fmt(),
            load: cpuinfo.get_busy(&old_cpuinfo),
            cpus: ncpus,
            memused: meminfo.memused(),
            swpused: meminfo.swpused(),
        };
        Xmobar::refresh(&data);
        std::thread::sleep(std::time::Duration::new(1, 0));
        old_cpuinfo = cpuinfo;
    }
}
