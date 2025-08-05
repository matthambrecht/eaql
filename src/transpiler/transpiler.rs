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

/// Starts a Transpiling loop that accepts queries from STDIN
/// and outputs color coded SQL matching cooresponding parts
/// of the input queries.
pub fn repl_loop() {
    loop {
        let query: String = io::query_stdin("transpiler");

        let parsed: Query = match process_query(&query) {
            Some(state) => state,
            None => {
                logger::warning("Invalid query, see above warnings for issues!");
                continue
            }
        };

        let transpiled: (String, String) = parsed.transpile_color();
        
        println!(
            "‣ {} {};",
            colorize("Reduced Query:", AnsiColor::BrightBlack), transpiled.0); 
        println!(
            "‣ {} {};",
            colorize("SQL Query:", AnsiColor::BrightBlack), transpiled.1); 
    };
}

/// Transpile Input Query (String) to SQL
/// 
/// # Example
/// ```
/// use eaql::transpiler::engine;
/// assert_eq!(engine(&"Get everything from db_1!"), Ok("SELECT * FROM db_1;".to_string()));
/// ```
///
pub fn engine(query: &str) -> Result<String, String> {
    let parsed: Query = match process_query(&query.to_string()) {
        Some(state) => state,
        None => {
            return Err("Invalid query, see above logged warnings for issues!".to_string());
        }
    };

    return Ok(format!("{};", parsed.transpile_raw()));
}