use super::{Output, StatusData, PowerInfo};
use std::io::{self, Write};

const COLOR_INACTIVE: &'static str = "#606060";
const COLOR_ACTIVE: &'static str = "#a8ff60";
const COLOR_CRITICAL: &'static str = "red";
const COLOR_WARNING: &'static str = "yellow";
const COLOR_NORMAL: &'static str = "orange";
const COLOR_MEDIUM: &'static str = COLOR_ACTIVE;

pub struct Xmobar {
}

impl Xmobar {
    fn out_battery(pow: &PowerInfo) -> String {
        let symb = match pow.battery_life {
            0...10   => format!("<fc={}>[ ! ]=</fc> ", COLOR_CRITICAL),
            11...25  => format!("<fc={}>['  ]=</fc> ", COLOR_WARNING),
            26...33  => format!("<fc={}>[|  ]=</fc> ", COLOR_WARNING),
            34...50  => format!("<fc={}>[|' ]=</fc> ", COLOR_MEDIUM),
            51...66  => format!("<fc={}>[|| ]=</fc> ", COLOR_NORMAL),
            67...80  => format!("<fc={}>[||']=</fc> ", COLOR_NORMAL),
            81...100 => format!("<fc={}>[||']=</fc> ", COLOR_NORMAL),
            _        => String::new(),
        };
        let minutes = match pow.battery_life {
            0...100 => format!("{}min ", pow.minutes_left),
            _       => String::new(),
        };
        symb + &minutes
    }
}

impl Output for Xmobar {
    fn refresh(status_data: &StatusData) {
        println!("<fc={}>|</fc> {}<fc={}>|</fc> <fc={}>{}</fc>",
               COLOR_INACTIVE,
               Xmobar::out_battery(&status_data.power_info),
               COLOR_INACTIVE,
               COLOR_ACTIVE,
               status_data.time
               );
        io::stdout().flush().unwrap();
    }
}
