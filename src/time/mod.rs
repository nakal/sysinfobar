extern crate time;

pub fn fmt() -> String {
    let tm = time::now();
    format!("{}, {}. {} {:02}:{:02}",
            weekday(tm.tm_wday),
            tm.tm_mday,
            month(tm.tm_mon),
            tm.tm_hour, tm.tm_min)
}

fn weekday(wd: i32) -> &'static str {
    match wd {
        0 => "So",
        1 => "Mo",
        2 => "Di",
        3 => "Mi",
        4 => "Do",
        5 => "Fr",
        6 => "Sa",
        _ => "??",
    }
}

fn month(m: i32) -> &'static str {
    match m {
        0 => "Jan",
        1 => "Feb",
        2 => "Mär",
        3 => "Apr",
        4 => "Mai",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Okt",
        10 => "Nov",
        11 => "Dez",
        _ => "??",
    }
}
