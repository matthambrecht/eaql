use std::io::{self, Write};
use crate::{
    utils::logger
};

pub fn query_stdin() -> String {
    // Get input
    let mut line: String = String::new();
    print!(">>> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string(); 
    logger::debug(&format!("Received Query String -> \"{}\"", line));
    line
}