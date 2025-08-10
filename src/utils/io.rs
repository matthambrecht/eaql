use crate::utils::logger;
use std::io::{self, Write};

pub fn query_stdin(tag: &str) -> String {
    // Get input
    let mut line: String = String::new();
    print!("({}) >>> ", tag);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    logger::debug(&format!("Received Query String -> \"{}\"", line));
    line
}
