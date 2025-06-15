// EAQL Logger - Will need to clean this up later
use chrono::Local;

// TODO: Move these to config?
const TEST_MODE: bool = true;
const LOG_LEVEL: i32 = 0;

const DEBUG: (i32, &str) = (1, "\x1b[90mDebug\x1b[0m");
const ERROR: (i32, &str) = (4, "\x1b[91mError\x1b[0m");
const INFO: (i32, &str) = (2, "\x1b[92mInfo\x1b[0m");
const WARNING: (i32, &str) = (3, "\x1b[93mWarning\x1b[0m");


fn send_msg(log_level: &str, msg: &str) -> String {
    let timestamp = get_timestamp();
    let prefix = match log_level.to_ascii_lowercase().as_str() {
        "debug" => DEBUG.1,
        "error" => ERROR.1,
        "info" => INFO.1,
        "warning" => WARNING.1,
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

pub fn info(msg: &str) -> () {
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