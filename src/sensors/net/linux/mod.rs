use std::cmp;
use std::fs;

const INTERVAL: u64 = 1;

pub struct NetStat {
    rx: u64,
    tx: u64,
    rx_max: u64,
    tx_max: u64,
}

impl NetStat {

    pub fn open() -> NetStat {
        let mut ns = NetStat { rx: 0, tx: 0, rx_max: 1, tx_max: 1 };
        let (rxv, txv) = NetStat::read_step(& mut ns);
        NetStat { rx: rxv, tx: txv, rx_max: 1, tx_max: 1 }
    }

    fn read_step(oldns: & mut NetStat) -> (u64, u64) {
        let contents = fs::read_to_string("/proc/net/netstat")
                    .expect("Cannot access /proc/net/netstat");

        let mut newrx: u64 = 0;
        let mut newtx: u64 = 0;

        for line in contents.lines() {
            let v: Vec<&str> = line.split_whitespace().collect();
            if v[0].eq("IpExt:") {
                let rx = match v[7].parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let tx = match v[8].parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                newrx = rx;
                newtx = tx;
                break;
            }
        }

        let diffrx = (newrx - oldns.rx) / INTERVAL;
        let difftx = (newtx - oldns.tx) / INTERVAL;

        *oldns = NetStat {
            rx: newrx,
            tx: newtx,
            rx_max: cmp::max(diffrx, oldns.rx_max),
            tx_max: cmp::max(difftx, oldns.tx_max),
        };

        (diffrx, difftx)
    }

    pub fn fetch(n: & mut NetStat) -> (u32, u32) {
        let (rx, tx) = NetStat::read_step(n);
        (
            (rx * 100 / n.rx_max) as u32,
            (tx * 100 / n.tx_max) as u32
        )
    }
}
