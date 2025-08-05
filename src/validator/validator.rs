use crate::{
    utils::{
        io,
        colors::{
            colorize,
            AnsiColor
        },
        query::process_query
    },
};

pub fn repl_loop() {
    loop {
        // Get input
        let query: String = io::query_stdin("validator");

        match process_query(&query) {
            Some(_) => println!("{}", colorize("Valid query!", AnsiColor::BrightGreen)),
            None => println!("{}", colorize("Invalid query, see above warnings for issues!", AnsiColor::BrightRed))
        };
    }
}