use crate::{
    utils::{
        logger,
        io
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
            Some(_) => logger::info("Valid query!"),
            None => logger::warning("Invalid query, see above warnings for issues!")
        };
    }
}