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

    fn cpu_color(load: u32, ncpus: u32) -> &'static str {
        let abs = load * ncpus;

        match abs {
            0...49  => COLOR_NORMAL,
            50...69 => COLOR_MEDIUM,
            70...89 => COLOR_WARNING,
            _       => COLOR_CRITICAL,
        }
    }

    fn mem_color(perc: u32) -> &'static str {
        match perc {
            0...49  => COLOR_NORMAL,
            50...69 => COLOR_MEDIUM,
            70...89 => COLOR_WARNING,
            _       => COLOR_CRITICAL,
        }
    }

    fn swp_color(perc: u32) -> &'static str {
        match perc {
            0...4   => COLOR_NORMAL,
            5...19  => COLOR_MEDIUM,
            20...49 => COLOR_WARNING,
            _       => COLOR_CRITICAL,
        }
    }
}

impl Output for Xmobar {
    fn refresh(status_data: &StatusData) {
        println!("<fc={}>|</fc> <fc={}>CPU:{:3}%</fc> <fc={}>MEM:{:3}%</fc> \
                 <fc={}>SWP:{:3}%</fc> {}<fc={}>|</fc> <fc={}>{}</fc>",
                 COLOR_INACTIVE,
                 Xmobar::cpu_color(status_data.load, status_data.cpus),
                 status_data.load,
                 Xmobar::mem_color(status_data.memused),
                 status_data.memused,
                 Xmobar::swp_color(status_data.swpused),
                 status_data.swpused,
                 Xmobar::out_battery(&status_data.power_info),
                 COLOR_INACTIVE,
                 COLOR_ACTIVE,
                 status_data.time
                );
        io::stdout().flush().unwrap();
    }
}
