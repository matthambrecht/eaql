use crate::utils::logger;
use crate::eaql::tokens;

pub fn evaluate(query: &String) {
    let mut parts: Vec<&str> = query.split_whitespace().collect();

    for token_idx in 0..parts.len() {
        for category in tokens::TOKENS.iter() {
            if category.0.contains(parts[token_idx]) {
                parts[token_idx] = category.1;
            }
        }
    }

    logger::debug(&format!("Query split -> {:?}", parts));
}
