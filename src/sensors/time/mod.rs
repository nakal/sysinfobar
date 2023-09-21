extern crate chrono;

pub fn fmt() -> String {
    let tm = chrono::Local::now();
    tm.format_localized("%a, %d. %b %H:%M", chrono::Locale::de_DE).to_string()
}
