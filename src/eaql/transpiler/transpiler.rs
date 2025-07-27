use crate::{
    utils::{
        io,
        logger,
    },
    eaql::{
        eaql::process_query,
        language::{
            parser::{
                parser::Query
            }
        },
    }
};

pub fn run() {
    loop {
        let query: String = io::query_stdin();

        let parsed: Query = match process_query(&query) {
            Some(state) => state,
            None => {
                logger::warning("Invalid query, see above warnings for issues!");
                continue
            }
        };

        parsed.transpile();
    };
}   
