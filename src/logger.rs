use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::ffi::OsStr;

pub fn logger(log_file: &OsStr, message: &str) {
    let mut log = File::options()
        .append(true)
        .create(true)
        .open(log_file)
        .unwrap();

    let dt = Local::now();
    let date_time_cr: String = dt.to_string();
    let date_time_formatted = date_time_cr[..19].to_string() + String::from(" >> ").as_str();

    log.write(date_time_formatted.as_bytes())
        .expect("error wrirting date to log");
    log.write(message.as_bytes())
        .expect("error writing log message");
    log.write("\n".as_bytes())
        .expect("error writing new line marker");
}
