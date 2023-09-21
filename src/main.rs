mod sensors;
mod output;

use output::xmobar::{Xmobar};
use output::Output;
use sensors::*;

fn main() {
    let ncpus = cpu::CPUInfo::ncpus();
    let mut old_cpuinfo = cpu::CPUInfo::fetch();
    let mut netstat = net::NetStat::open();
    loop {
        let cpuinfo = cpu::CPUInfo::fetch();
        let meminfo = mem::MemInfo::fetch();
        let (rx, tx) = net::NetStat::fetch(& mut netstat);
        let data = output::StatusData {
            power_info: battery::PowerInfo::fetch(),
            time: time::fmt(),
            load: cpuinfo.get_busy(&old_cpuinfo),
            cpus: ncpus,
            memused: meminfo.memused(),
            swpused: meminfo.swpused(),
            net_rx: rx,
            net_tx: tx,
        };
        Xmobar::refresh(&data);
        std::thread::sleep(std::time::Duration::new(1, 0));
        old_cpuinfo = cpuinfo;
    }
}
