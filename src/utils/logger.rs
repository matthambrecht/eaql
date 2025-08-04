// EAQL Logger - Will need to clean this up later
use chrono::Local;
use crate::utils::colors::{
    AnsiColor,
    colorize
};

// TODO: Move these to config?
pub const TEST_MODE: bool = true;
pub const LOG_LEVEL: i32 = 0;

pub const DEBUG: (i32, &str, AnsiColor) = (1, "Debug", AnsiColor::BrightBlack);
pub const ERROR: (i32, &str, AnsiColor) = (4, "Error", AnsiColor::BrightRed);
pub const INFO: (i32, &str, AnsiColor) = (2, "Info", AnsiColor::BrightBlue);
pub const WARNING: (i32, &str, AnsiColor) = (3, "Warning", AnsiColor::BrightYellow);

fn send_msg(log_level: &str, msg: &str) -> String {
    let timestamp = get_timestamp();
    let prefix = match log_level.to_ascii_lowercase().as_str() {
        "debug" => &colorize(DEBUG.1, DEBUG.2),
        "error" => &colorize(ERROR.1, ERROR.2),
        "info" => &colorize(INFO.1, INFO.2),
        "warning" => &colorize(WARNING.1, WARNING.2),
        _ => "Unknown",
    };

    format!("[{timestamp}][{prefix}] {msg}")
}


fn get_timestamp() -> String {
    return Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn debug(msg: &str) -> () {
    if TEST_MODE && LOG_LEVEL <= DEBUG.0 {
        println!("{}", send_msg("debug", msg));
    }
}

pub fn _info(msg: &str) -> () {
    if TEST_MODE && LOG_LEVEL <= INFO.0 {
        println!("{}", send_msg("info", msg));
    }
}

pub fn warning(msg: &str) -> () {
    if TEST_MODE && LOG_LEVEL <= WARNING.0 {
        eprintln!("{}", send_msg("warning", msg));
    }
}

pub fn error(msg: &str) -> ! {    
    panic!("{}", send_msg("error", msg));
}