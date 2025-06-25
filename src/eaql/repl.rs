use std::io::{self, Write};
use crate::eaql::lexer::{self as lexer, Lexer};
use crate::eaql::parser::parser;
use crate::utils::logger as logger;

pub fn run() {
    loop {
        // Get input
        let mut line: String = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string(); 
        logger::debug(&format!("Received Query String -> \"{}\"", line));
        
        // Tokenize input
        let tokenized: Result<Lexer, String> = lexer::scan_tokens(
            &line);

        let tokens = match tokenized {
            Ok(tokenized) => {
                logger::debug(&format!("Tokenized String -> \n{tokenized}"));
                tokenized.tokens
            },
            Err(e) => {
                logger::warning(&e);
                continue;
            }
        };


        // Parse into an Abstract Syntax Tree
        let parsed: Result<parser::Query, String> = parser::parse(
            &tokens);

        let _ast = match parsed {
            Ok(parsed) => {
                logger::debug(&format!("Abstract Syntax Tree -> \n{parsed}"));
                parsed
            },
            Err(e) => {
                logger::warning(&e);
                continue;
            }
        };
    }
}