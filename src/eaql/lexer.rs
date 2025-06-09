use std::str::{SplitWhitespace};

use crate::utils::logger as logger;

pub fn evaluate(query: &String) {
    let split: SplitWhitespace = query.split_whitespace();
    let parts = split.collect::<Vec<&str>>();
    logger::debug(&format!("Query split -> {:?}", parts))
}