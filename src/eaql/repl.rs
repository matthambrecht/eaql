use std::io::{self, Write};
use crate::eaql::lexer as lexer;
use crate::utils::logger as logger;

pub fn run() {
    loop {
        let mut line: String = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        logger::debug(&format!("Received REPL String -> \"{}\"", line));
        lexer::evaluate(&line);
    }
}