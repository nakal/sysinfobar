mod battery;
mod time;
mod output;

use output::xmobar::{Xmobar};
use output::Output;

fn main() {
    loop {
        let data = output::StatusData {
            power_info: battery::PowerInfo::fetch(),
            time: time::fmt(),
        };
        Xmobar::refresh(&data);
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}
