use crate::{
    language::parser::parser::Query,
    utils::{
        colors::{
            colorize,
            AnsiColor
        },
        io,
        logger,
        query::process_query
    }
};

pub fn engine() {
    loop {
        let query: String = io::query_stdin("transpiler");

        let parsed: Query = match process_query(&query) {
            Some(state) => state,
            None => {
                logger::warning("Invalid query, see above warnings for issues!");
                continue
            }
        };

        let transpiled: (String, String) = parsed.transpile();
        
        println!(
            "‣ {} {};",
            colorize("Reduced Query:", AnsiColor::BrightBlack), transpiled.0); 
        println!(
            "‣ {} {};",
            colorize("SQL Query:", AnsiColor::BrightBlack), transpiled.1); 
    };
}   
