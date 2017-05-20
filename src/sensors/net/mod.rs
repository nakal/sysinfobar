use std::sync::{Arc, Mutex};
use std::thread;
use std::process::{Command, Stdio, ChildStdout};
use std::io::Read;
use std::cmp;

const INTERVAL: u64 = 5;

macro_rules! constants_for_system {
    ( $s:expr => $(const $l:ident: $t:ty = $e:expr);*; ) =>
    {
        $(
        #[cfg(target_os = $s)]
        const $l : $t = $e
        );*;
    }
}

constants_for_system!("openbsd" =>
                      const NETSTAT_COLUMNS: usize = 2;
                      const NETSTAT_COLUMN_RX: usize = 2;
                      const NETSTAT_COLUMN_TX: usize = 1;
                     );

constants_for_system!("freebsd" =>
                      const NETSTAT_COLUMNS: usize = 8;
                      const NETSTAT_COLUMN_RX: usize = 5;
                      const NETSTAT_COLUMN_TX: usize = 2;
                     );


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
        let netstat_thread = netstat.clone();
        let netstat_main = netstat.clone();
        thread::spawn(move || {
            match Command::new("netstat")
                .arg("-ib")
                .arg(INTERVAL.to_string())
                .stdout(Stdio::piped())
                .spawn() {
                    Err(_)  => (),
                    Ok(p)   => NetStat::read_loop(netstat_thread, &mut p.stdout.unwrap()),
                }
        });
        netstat_main
    }

    fn read_loop(netstat: Arc<Mutex<NetStat>>, out: &mut ChildStdout) {
        for _ in 1..3 {
            let mut buf = vec![0;512];
            out.read(&mut buf).unwrap();
        }
        loop {
            let mut buf = vec![0;512];
            let size = out.read(&mut buf).unwrap();
            buf.truncate(size);
            let s = String::from_utf8(buf).unwrap();
            let words: Vec<&str> = s.split_whitespace().collect();
            let wc = words.len();
            if wc >= NETSTAT_COLUMNS {
                let mut n = netstat.lock().unwrap();
                if let Ok(newrx) = words[wc - NETSTAT_COLUMN_RX].parse::<u64>() {
                    n.rx = newrx / INTERVAL;
                    n.rx_max = cmp::max(n.rx, n.rx_max);
                }
                if let Ok(newtx) = words[wc - NETSTAT_COLUMN_TX].parse::<u64>() {
                    n.tx = newtx / INTERVAL;
                    n.tx_max = cmp::max(n.tx, n.tx_max);
                }
            }
        }
    }

    pub fn fetch(netstat: &Arc<Mutex<NetStat>>) -> (u32, u32) {
        let n = netstat.lock().unwrap();
        (
            (n.rx * 100 / n.rx_max) as u32,
            (n.tx * 100 / n.tx_max) as u32
        )
    }
}
