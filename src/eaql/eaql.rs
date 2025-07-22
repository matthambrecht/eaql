// This is the main interface to the interworkings of the EAQL language
use crate::eaql::language::{
    lexer::{self, Lexer},
    parser::parser::{self, Query},
    tokens::{self}
};

use crate::utils::logger;

// Process query to low level components (parsed)
pub fn process_query(
    query: &String
) -> Option<Query> {
     // Tokenize input
    let tokenized: Result<Lexer, String> = lexer::scan_tokens(
        &query);

    let tokens = match tokenized {
        Ok(tokenized) => {
            logger::debug(&format!("Tokenized String -> \n{tokenized}"));
            tokenized.tokens
        },
        Err(e) => {
            logger::warning(&e);
            return None;
        }
    };

    // This will go once we see the ability for mutliple queries chained together
    if tokens.len() != 0 && tokens[tokens.len() - 1].token_type != tokens::TokenType::EoqToken {
        logger::warning("Missing end of query delimiter!");
        return None;
    }

    // Parse into an Abstract Syntax Tree
    let parsed: Result<parser::Query, String> = parser::parse(
        &tokens);

    let ast = match parsed {
        Ok(parsed) => {
            logger::debug(&format!("Abstract Syntax Tree -> \n{parsed}"));
            parsed
        },
        Err(e) => {
            logger::warning(&e);
            return None;
        }
    };

    return Some(ast);
}