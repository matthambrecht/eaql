use crate::{
    utils::{
        logger,
        io,
        colors::{
            colorize,
            AnsiColor
        }
    },
    eaql::{
        eaql::process_query
    }
};

pub fn run() {
    loop {
        // Get input
        let query: String = io::query_stdin();

        match process_query(&query) {
            Some(_) => println!("{}", colorize("Valid query!", AnsiColor::BrightGreen)),
            None => println!("{}", colorize("Invalid query, see above warnings for issues!", AnsiColor::BrightRed))
        };
    }
}