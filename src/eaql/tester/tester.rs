use std::io::{self, Write};
use crate::{
    utils::{
        logger
    },
    eaql::{
        eaql::process_query
    }
};

pub fn run() {
    // Should probably begin to abstract this functionality out, will need to be used elsewhere soon
    loop {
        // Get input
        let mut line: String = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string(); 
        logger::debug(&format!("Received Query String -> \"{}\"", line));

        match process_query(&line) {
            Some(_) => logger::info("Valid query!"),
            None => logger::warning("Invalid query, see above warnings for issues!")
        };
    }
}