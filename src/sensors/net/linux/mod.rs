use std::sync::{Arc, Mutex};

pub struct NetStat {
    rx: u64,
    tx: u64,
    rx_max: u64,
    tx_max: u64,
}

impl NetStat {
    pub fn open() -> Arc<Mutex<NetStat>> {
        let netstat = Arc::new(Mutex::new(
                NetStat { rx: 0, tx: 0, rx_max: 1, tx_max: 1 }
                ));
        netstat.clone()
    }

    pub fn fetch(netstat: &Arc<Mutex<NetStat>>) -> (u32, u32) {
        let n = netstat.lock().unwrap();
        (
            (n.rx * 100 / n.rx_max) as u32,
            (n.tx * 100 / n.tx_max) as u32
        )
    }
}
