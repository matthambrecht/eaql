use crate::eaql::tokens::{Token, TokenType};

const TAB_SIZE: u16 = 2;

pub fn validate_length(
    tokens: &Vec<Token>,
    idx: &usize,
    required: bool
) -> Result<(), String>{
    if *idx >= tokens.len() && required {
        return Err(
                format!("Query valid until after \"{}\"! Possible unfinished query?",
                tokens[..*idx].into_iter().map(
                |x| x.lexeme.as_str()
                ).collect::<Vec<&str>>().join(" ")).to_string());
    }

    Ok(())
}

pub fn get_tab(
    depth: u16
) -> String {
    " ".repeat(TAB_SIZE as usize * depth as usize)
}

pub fn valid_until_warning(
    tokens: &Vec<Token>,
    idx: &usize
) -> String {
    if *idx >= tokens.len() {
        return "N/A".to_string();
    }

    format!("Query valid until after \"{}\"!",
        tokens[..*idx].into_iter().map(
        |x| x.lexeme.as_str()
        ).collect::<Vec<&str>>().join(" ")).to_string()
}

// Look a token ahead
pub fn peek_one(
    tokens: &Vec<Token>,
    idx: &usize
) -> TokenType {
    if *idx + 1 >= tokens.len() {
        TokenType::NullToken
    } else {
        tokens[*idx + 1].token_type.clone()
    }
}