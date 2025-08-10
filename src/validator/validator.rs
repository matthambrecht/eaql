use crate::utils::{
    colors::{AnsiColor, colorize},
    io,
    query::process_query,
};

/// Starts a Validator loop that accepts queries from STDIN
/// and validates queries while outputting error information
pub fn repl_loop() {
    loop {
        // Get input
        let query: String = io::query_stdin("validator");

        match process_query(&query) {
            Some(_) => println!("{}", colorize("Valid query!", AnsiColor::BrightGreen)),
            None => println!(
                "{}",
                colorize(
                    "Invalid query, see above warnings for issues!",
                    AnsiColor::BrightRed
                )
            ),
        };
    }
}

/// Validate Input Query (String)
///
/// # Example
/// ```
/// use eaql::validator::engine;
/// assert_eq!(engine(&"Get everything from db_1!"), true);
/// assert_eq!(engine(&"Get everything from db_1"), false);
/// ```
///
pub fn engine(query: &str) -> bool {
    match process_query(&query.to_string()) {
        Some(_) => true,
        None => false,
    }
}
